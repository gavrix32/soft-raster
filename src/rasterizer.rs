use crate::framebuffer::Framebuffer;
use crate::math::{Pixel, Vec2, Vec3};

#[allow(dead_code)]
pub enum PolygonMode {
    Point,
    Line,
    Fill,
}

pub struct Rasterizer<'a> {
    framebuffer: &'a mut Framebuffer,
    polygon_mode: PolygonMode,
    fov: f32,
    offset: f32,
}

impl<'a> Rasterizer<'a> {
    pub fn new(
        framebuffer: &'a mut Framebuffer,
        polygon_mode: PolygonMode,
        fov: f32,
        offset: f32,
    ) -> Self {
        Self {
            framebuffer,
            polygon_mode,
            fov,
            offset,
        }
    }

    fn ndc_to_screen(&self, v: Vec2) -> Pixel {
        let x = ((v.x + 1.0) / 2.0) * self.framebuffer.width as f32;
        let y = ((-v.y + 1.0) / 2.0) * self.framebuffer.height as f32;

        Pixel {
            x: x.floor() as usize,
            y: y.floor() as usize,
        }
    }

    fn project(&self, v: Vec3, offset: f32) -> Vec2 {
        let fov_radians = self.fov.to_radians();
        let aspect = self.framebuffer.width as f32 / self.framebuffer.height as f32;
        let z = v.z + offset;

        Vec2 {
            x: (v.x + 0.025) / z / (aspect * (fov_radians / 2.0).tan()),
            y: (v.y - 0.112) / z / (fov_radians / 2.0).tan(),
        }
    }

    pub fn draw_point(&mut self, v: Vec3, color: &[u8]) {
        let v_ndc = self.project(v, self.offset);
        let v_screen = self.ndc_to_screen(v_ndc);
        self.framebuffer.set_pixel(v_screen, color, 0.0);
    }

    /**
     * Bresenham's line algorithm
     * Web: https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
     */
    fn draw_line(&mut self, v0: Vec3, v1: Vec3, color: &[u8]) {
        let v0_ndc = self.project(v0, self.offset);
        let v1_ndc = self.project(v1, self.offset);
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
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 {
                    break;
                }
                error += dy;
                x0 += sx;
            }
            if e2 <= dx {
                if y0 == y1 {
                    break;
                }
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
