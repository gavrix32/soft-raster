mod framebuffer;
mod math;
mod rasterizer;

use crate::framebuffer::Framebuffer;
use crate::math::Vec3;
use crate::rasterizer::{PolygonMode, Rasterizer};

fn main() -> std::io::Result<()> {
    let width = 1024;
    let height = 1024;

    let mut framebuffer = Framebuffer::new(width, height);
    let mut rasterizer = Rasterizer::new(&mut framebuffer, PolygonMode::Line, 60.0, 3.0);

    let cube_vertices = [
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
    ];

    let cube_indices = [
        (4, 5, 6),
        (4, 6, 7),
        (1, 0, 3),
        (1, 3, 2),
        (0, 4, 7),
        (0, 7, 3),
        (5, 1, 2),
        (5, 2, 6),
        (7, 6, 2),
        (7, 2, 3),
        (0, 1, 5),
        (0, 5, 4),
    ];

    for (i0, i1, i2) in cube_indices {
        rasterizer.draw_triangle(
            cube_vertices[i0],
            cube_vertices[i1],
            cube_vertices[i2],
            &[255, 0, 0],
        );
    }

    framebuffer.save_ppm("out.ppm")?;
    Ok(())
}
