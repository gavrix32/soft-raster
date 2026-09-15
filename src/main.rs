mod framebuffer;

use crate::framebuffer::Framebuffer;
fn main() -> std::io::Result<()> {
    let width = 1024;
    let height = 1024;
    let mut framebuffer = Framebuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let color = [(x * 255 / width) as u8, (y * 255 / height) as u8, 0];
            framebuffer.set_pixel(x, y, &color, 0.0);
        }
    }

    framebuffer.save_ppm("out.ppm")?;
    Ok(())
}
