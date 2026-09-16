use crate::framebuffer::Framebuffer;

pub struct Rasterizer<'a> {
    framebuffer: &'a mut Framebuffer,
}

impl<'a> Rasterizer<'a> {
    pub fn new(framebuffer: &'a mut Framebuffer) -> Self {
        Self { framebuffer }
    }

    /**
     * Bresenham's line algorithm
     * Web: https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
     */
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: &[u8]) {
        let mut x0 = x0;
        let mut y0 = y0;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut error = dx + dy;

        loop {
            self.framebuffer
                .set_pixel(x0 as usize, y0 as usize, color, 0.0);
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
}
