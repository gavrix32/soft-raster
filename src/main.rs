#![feature(random)]

mod framebuffer;
mod math;
mod mesh;
mod rasterizer;
mod shader;

use crate::framebuffer::Framebuffer;
use crate::math::{Mat4, Vec3};
use crate::mesh::Mesh;
use crate::rasterizer::{PolygonMode, Rasterizer};
use crate::shader::{DirectionalLight, PointLight, Shader};

fn main() -> std::io::Result<()> {
    let width = 2048;
    let height = 2048;

    let sky_color = Vec3::new(0.3, 0.6, 1.0);

    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.clear_color(sky_color);
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
    let mut shader = Shader::new(sky_color * 0.5);
    shader.add_light(DirectionalLight::new(
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
    ));
    shader.add_light(PointLight::new(
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
    ));
    let mut rasterizer = Rasterizer::new(&mut framebuffer, PolygonMode::Fill, proj * view, shader);
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
            Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0),
        );
    }

    framebuffer.save_ppm("out.ppm")?;
    Ok(())
}
