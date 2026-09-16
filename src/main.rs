mod framebuffer;
mod rasterizer;

use crate::framebuffer::Framebuffer;
use crate::rasterizer::Rasterizer;

fn main() -> std::io::Result<()> {
    let width = 1024;
    let height = 1024;

    let mut framebuffer = Framebuffer::new(width, height);
    let mut rasterizer = Rasterizer::new(&mut framebuffer);

    let x0: i32 = 50;
    let y0: i32 = 70;
    let x1: i32 = 700;
    let y1: i32 = 900;

    rasterizer.draw_line(x0, y0, x1, y1, &[255, 0, 0]);

    framebuffer.save_ppm("out.ppm")?;
    Ok(())
}
