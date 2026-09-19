use crate::framebuffer::Framebuffer;
use crate::math::{Mat4, Pixel, Vec3, Vec4};

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

    fn ndc_to_screen(&self, v: Vec3) -> Pixel {
        let x = ((v.x + 1.0) / 2.0) * self.framebuffer.width as f32;
        let y = ((-v.y + 1.0) / 2.0) * self.framebuffer.height as f32;

        Pixel {
            x: x.floor() as usize,
            y: y.floor() as usize,
        }
    }

    pub fn draw_point(&mut self, v: Vec3, color: &[u8]) {
        let v_clip = self.view_proj * Vec4::new(v.x, v.y, v.z, 1.0);
        let v_ndc = self.clip_to_ndc(v_clip);
        let v_screen = self.ndc_to_screen(v_ndc);
        self.framebuffer.set_pixel(v_screen, color, 0.0);
    }

    /**
     * Bresenham's line algorithm
     * Web: https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
     */
    fn draw_line(&mut self, v0: Vec3, v1: Vec3, color: &[u8]) {
        let v0_clip = self.view_proj * Vec4::new(v0.x, v0.y, v0.z, 1.0);
        let v1_clip = self.view_proj * Vec4::new(v1.x, v1.y, v1.z, 1.0);
        let v0_ndc = self.clip_to_ndc(v0_clip);
        let v1_ndc = self.clip_to_ndc(v1_clip);
        let v0_screen = self.ndc_to_screen(v0_ndc);
        let v1_screen = self.ndc_to_screen(v1_ndc);

        let mut x0 = v0_screen.x as i32;
        let mut y0 = v0_screen.y as i32;
        let x1 = v1_screen.x as i32;
        let y1 = v1_screen.y as i32;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            self.framebuffer
                .set_pixel(Pixel::new(x0 as usize, y0 as usize), color, 0.0);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                error += dy;
                x0 += sx;
            }
            if e2 <= dx {
                error += dx;
                y0 += sy;
            }
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
            PolygonMode::Fill => {}
        }
    }
}
