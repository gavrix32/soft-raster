use crate::framebuffer::Framebuffer;
use crate::math::{Mat4, Vec2I, Vec2U, Vec3, Vec3I, Vec4};

#[allow(dead_code)]
pub enum PolygonMode {
    Point,
    Line,
    Fill,
}

pub struct Rasterizer<'a> {
    framebuffer: &'a mut Framebuffer,
    polygon_mode: PolygonMode,
    view_proj: Mat4,
}

impl<'a> Rasterizer<'a> {
    pub fn new(
        framebuffer: &'a mut Framebuffer,
        polygon_mode: PolygonMode,
        view_proj: Mat4,
    ) -> Self {
        Self {
            framebuffer,
            polygon_mode,
            view_proj,
        }
    }

    fn clip_to_ndc(&self, v: Vec4) -> Vec3 {
        Vec3::new(v.x / v.w, v.y / v.w, v.z / v.w)
    }

    fn ndc_to_screen(&self, v: Vec3) -> Vec2I {
        let x = ((v.x + 1.0) / 2.0) * (self.framebuffer.width - 1) as f32;
        let y = ((-v.y + 1.0) / 2.0) * (self.framebuffer.height - 1) as f32;

        Vec2I {
            x: x.floor() as i32,
            y: y.floor() as i32,
        }
    }

    fn project(&self, v: Vec3) -> (Vec2I, f32) {
        let v_clip = self.view_proj * Vec4::new(v.x, v.y, v.z, 1.0);
        let v_ndc = self.clip_to_ndc(v_clip);
        (self.ndc_to_screen(v_ndc), v_ndc.z)
    }

    fn draw_point(&mut self, v: Vec3, color: &[u8]) {
        let (p, z) = self.project(v);
        self.framebuffer.set_pixel(p.as_vec2u(), color, z);
    }

    /**
     * Bresenham's line algorithm
     * Web: https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
     */
    fn draw_line(&mut self, v0: Vec3, v1: Vec3, color: &[u8]) {
        let (mut p0, z0) = self.project(v0);
        let (p1, z1) = self.project(v1);

        let dx = (p1.x - p0.x).abs();
        let sx = if p0.x < p1.x { 1 } else { -1 };
        let dy = -(p1.y - p0.y).abs();
        let sy = if p0.y < p1.y { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            self.framebuffer.set_pixel(p0.as_vec2u(), color, 0.0);
            if p0 == p1 {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                error += dy;
                p0.x += sx;
            }
            if e2 <= dx {
                error += dx;
                p0.y += sy;
            }
        }
    }

    fn fill_triangle(&mut self, v0: Vec3, v1: Vec3, v2: Vec3, color: &[u8]) {
        // CW -> CCW
        let (v1, v2) = (v2, v1);

        let (p0, z0) = self.project(v0);
        let (p1, z1) = self.project(v1);
        let (p2, z2) = self.project(v2);

        let area = (p1 - p0).cross(p2 - p0);
        let inv_area = 1.0 / area as f32;

        if area <= 0 {
            return;
        }

        let bbox_min = Vec2I::new(p0.x.min(p1.x.min(p2.x)), p0.y.min(p1.y.min(p2.y)));
        let bbox_max = Vec2I::new(p0.x.max(p1.x.max(p2.x)), p0.y.max(p1.y.max(p2.y)));

        fn top_left_bias(a: Vec2I, b: Vec2I) -> i32 {
            let edge = b - a;
            let is_top = edge.y == 0 && edge.x > 0;
            let is_left = edge.y < 0;

            if is_top || is_left { 0 } else { -1 }
        }

        let bias = Vec3I::new(
            top_left_bias(p1, p2),
            top_left_bias(p2, p0),
            top_left_bias(p0, p1),
        );

        let delta_p0 = p1 - p2;
        let delta_p1 = p2 - p0;
        let delta_p2 = p0 - p1;

        let mut w0_x = (p2 - p1).cross(bbox_min - p1);
        let mut w1_x = (p0 - p2).cross(bbox_min - p2);
        let mut w2_x = (p1 - p0).cross(bbox_min - p0);

        for y in bbox_min.y..=bbox_max.y {
            let mut w0 = w0_x;
            let mut w1 = w1_x;
            let mut w2 = w2_x;
            for x in bbox_min.x..=bbox_max.x {
                let is_inside = w0 + bias.x >= 0 && w1 + bias.y >= 0 && w2 + bias.z >= 0;

                if is_inside {
                    // let barycentric = Vec3::new(
                    //     e0 as f32 * inv_area,
                    //     e1 as f32 * inv_area,
                    //     e2 as f32 * inv_area,
                    // );

                    let p = Vec2I::new(x, y);
                    let z = (w0 as f32 * z0 + w1 as f32 * z1 + w2 as f32 * z2) * inv_area;

                    // self.framebuffer.set_pixel(p.as_vec2u(), color, z);
                    self.framebuffer.set_pixel(p.as_vec2u(), &[((z + 1.0) * 0.5 * 255.0) as u8, 0, 0], z);
                }
                w0 += delta_p0.y;
                w1 += delta_p1.y;
                w2 += delta_p2.y;
            }
            w0_x -= delta_p0.x;
            w1_x -= delta_p1.x;
            w2_x -= delta_p2.x;
        }
    }

    pub fn draw_triangle(&mut self, v0: Vec3, v1: Vec3, v2: Vec3, color: &[u8]) {
        match self.polygon_mode {
            PolygonMode::Point => {
                self.draw_point(v0, color);
                self.draw_point(v1, color);
                self.draw_point(v2, color);
            }
            PolygonMode::Line => {
                self.draw_line(v0, v1, color);
                self.draw_line(v1, v2, color);
                self.draw_line(v2, v0, color);
            }
            PolygonMode::Fill => self.fill_triangle(v0, v1, v2, color),
        }
    }
}
