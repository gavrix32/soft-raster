mod framebuffer;
mod math;
mod mesh;
mod rasterizer;
mod shader;
mod texture;

use crate::framebuffer::Framebuffer;
use crate::math::{Mat4, Vec3};
use crate::mesh::Mesh;
use crate::rasterizer::{Material, PolygonMode, Rasterizer};
use crate::shader::{DirectionalLight, PointLight, Shader};
use crate::texture::Texture;

fn main() -> std::io::Result<()> {
    let width = 2048;
    let height = 2048;

    let sky_color = Vec3::new(0.3, 0.6, 1.0) * 0.5;
    // let sky_color = Vec3::splat(0.0);

    let mut framebuffer = Framebuffer::new(width, height);
    framebuffer.clear_color(sky_color);
    let eye = Vec3::new(0.0, 0.0, 2.5);
    // let eye = Vec3::new(-0.02, 0.1, 0.2);
    let forward = Vec3::new(0.0, 0.0, -1.0);
    let view = Mat4::look_at(eye, eye + forward, Vec3::new(0.0, 1.0, 0.0));
    let proj = Mat4::perspective(
        60.0_f32.to_radians(),
        width as f32 / height as f32,
        0.1,
        100.0,
    );
    let mut shader = Shader::new(sky_color);
    shader.add_light(DirectionalLight::new(
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
    ));
    shader.add_light(PointLight::new(
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 1.0),
    ));
    let mut rasterizer = Rasterizer::new(&mut framebuffer, PolygonMode::Fill, proj * view, shader);

    let mesh1 = Mesh::load_obj("assets/african_head/african_head.obj")?;
    let texture1 = Texture::load_ppm("assets/african_head/african_head_diffuse.ppm")?;

    let mesh2 = Mesh::load_obj("assets/african_head/african_head_eye_inner.obj")?;
    let texture2 = Texture::load_ppm("assets/african_head/african_head_eye_inner_diffuse.ppm")?;

    rasterizer.draw_mesh(&mesh1, &Material::Textured(texture1));
    rasterizer.draw_mesh(&mesh2, &Material::Textured(texture2));

    framebuffer.save_ppm("out.ppm")?;
    Ok(())
}
