mod framebuffer;
mod math;
mod mesh;
mod rasterizer;

use crate::framebuffer::Framebuffer;
use crate::mesh::Mesh;
use crate::rasterizer::{PolygonMode, Rasterizer};

fn main() -> std::io::Result<()> {
    let width = 4096;
    let height = 4096;

    let mut framebuffer = Framebuffer::new(width, height);
    let mut rasterizer = Rasterizer::new(&mut framebuffer, PolygonMode::Line, 50.0, 0.21);

    let mesh = Mesh::load_obj("models/bunny.obj")?;
    for face in mesh.faces {
        rasterizer.draw_triangle(
            mesh.vertices[face[0]],
            mesh.vertices[face[1]],
            mesh.vertices[face[2]],
            &[0, 255, 0],
        );
    }

    framebuffer.save_ppm("out.ppm")?;
    Ok(())
}
