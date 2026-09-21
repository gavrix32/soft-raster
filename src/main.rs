#![feature(random)]

mod framebuffer;
mod math;
mod mesh;
mod rasterizer;

use crate::framebuffer::Framebuffer;
use crate::math::{Mat4, Vec3};
use crate::mesh::Mesh;
use crate::rasterizer::{PolygonMode, Rasterizer};

// TODO: Clipping

fn main() -> std::io::Result<()> {
    let width = 2048;
    let height = 2048;

    let mut framebuffer = Framebuffer::new(width, height);
    // let eye = Vec3::new(0.0, 0.0, 3.0);
    let eye = Vec3::new(-0.02, 0.1, 0.2);
    let forward = Vec3::new(0.0, 0.0, -1.0);
    let view = Mat4::look_at(eye, eye + forward, Vec3::new(0.0, 1.0, 0.0));
    let proj = Mat4::perspective(
        60.0_f32.to_radians(),
        width as f32 / height as f32,
        0.1,
        100.0,
    );
    let mut rasterizer = Rasterizer::new(&mut framebuffer, PolygonMode::Fill, proj * view);
    // let mut rasterizer = Rasterizer::new(&mut framebuffer, PolygonMode::Line, Mat4::identity());

    // rasterizer.draw_triangle(
    //     Vec3::new(0.0, 0.5, 0.0),
    //     Vec3::new(0.5, -0.5, 0.0),
    //     Vec3::new(-0.5, -0.5, 0.0),
    //     &[0, 255, 0],
    // );

    let mesh = Mesh::load_obj("models/bunny.obj")?;
    for face in mesh.faces {
        let r: u8 = std::random::random(..);
        let g: u8 = std::random::random(..);
        let b: u8 = std::random::random(..);

        rasterizer.draw_triangle(
            mesh.vertices[face[0]],
            mesh.vertices[face[1]],
            mesh.vertices[face[2]],
            &[r, g, b],
        );
    }

    framebuffer.save_ppm("out.ppm")?;
    Ok(())
}
