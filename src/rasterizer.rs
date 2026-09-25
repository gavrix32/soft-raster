use crate::framebuffer::Framebuffer;
use crate::math;
use crate::math::{Mat4, Vec2, Vec2I, Vec2U, Vec3, Vec3I, Vec4};
use crate::mesh::{Mesh, Vertex};
use crate::shader::Shader;
use crate::texture::Texture;

#[allow(dead_code)]
pub enum Material {
    Solid(Vec3),
    Textured(Texture),
}

impl Material {
    pub fn sample(&self, uv: Vec2) -> Vec3 {
        match self {
            Material::Solid(c) => *c,
            Material::Textured(t) => t.sample(uv),
        }
    }
}

#[allow(dead_code)]
pub enum PolygonMode {
    Point(i32),
    Line(i32),
    Fill,
}

pub struct Rasterizer<'a> {
    framebuffer: &'a mut Framebuffer,
    polygon_mode: PolygonMode,
    view_proj: Mat4,
    shader: Shader,
}

impl<'a> Rasterizer<'a> {
    pub fn new(
        framebuffer: &'a mut Framebuffer,
        polygon_mode: PolygonMode,
        view_proj: Mat4,
        shader: Shader,
    ) -> Self {
        Self {
            framebuffer,
            polygon_mode,
            view_proj,
            shader,
        }
    }

    fn clip_to_ndc(&self, v: Vec4) -> Vec3 {
        Vec3::new(v.x / v.w, v.y / v.w, v.z / v.w)
    }

    fn ndc_to_screen(&self, v: Vec3) -> Vec2I {
        let x = ((v.x + 1.0) / 2.0) * (self.framebuffer.width - 1) as f32;
        let y = ((v.y + 1.0) / 2.0) * (self.framebuffer.height - 1) as f32;

        Vec2I {
            x: x.floor() as i32,
            y: y.floor() as i32,
        }
    }

    fn project(&self, v: Vec3) -> (Vec2I, f32, f32) {
        let v_clip = self.view_proj * Vec4::new(v.x, v.y, v.z, 1.0);
        let v_ndc = self.clip_to_ndc(v_clip);
        (self.ndc_to_screen(v_ndc), v_ndc.z, v_clip.w)
    }

    fn fill_rect(&mut self, min: Vec2U, max: Vec2U, color: Vec3, depth: f32) {
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                self.framebuffer.set_pixel(Vec2U::new(x, y), color, depth);
            }
        }
    }

    fn draw_point(&mut self, v: Vertex, material: &Material, size: i32) {
        let (p, z, _w) = self.project(v.position);
        let base_color = material.sample(v.texcoord);
        let color = self.shader.shade(base_color, v.position, v.normal);
        let half_size = size / 2;
        let min = p - half_size;
        let max = min + size - 1;
        self.fill_rect(min.as_vec2u(), max.as_vec2u(), color, z);
    }

    /**
     * Bresenham's line algorithm
     * Web: https://en.wikipedia.org/wiki/Bresenham's_line_algorithm
     */
    fn draw_line(&mut self, v0: Vertex, v1: Vertex, material: &Material, width: i32) {
        let (mut p0, z0, w0) = self.project(v0.position);
        let (p1, z1, w1) = self.project(v1.position);

        let half_width = width / 2;

        let dx = (p1.x - p0.x).abs();
        let sx = if p0.x < p1.x { 1 } else { -1 };
        let dy = -(p1.y - p0.y).abs();
        let sy = if p0.y < p1.y { 1 } else { -1 };
        let mut error = dx + dy;

        let major = dx.max(-dy);
        let mut i = 0;

        let v0_w = v0.position / w0;
        let v1_w = v1.position / w1;
        let uv0_w = v0.texcoord / w0;
        let uv1_w = v1.texcoord / w1;
        let n0_w = v0.normal / w0;
        let n1_w = v1.normal / w1;

        loop {
            let t = if major == 0 {
                0.0
            } else {
                i as f32 / major as f32
            };
            let nt = 1.0 - t;

            let min = p0 - half_width;
            let max = min + width - 1;

            let inv_w = nt / w0 + t / w1;
            let pos = (v0_w * nt + v1_w * t) / inv_w;
            let uv = (uv0_w * nt + uv1_w * t) / inv_w;
            let norm = (n0_w * nt + n1_w * t) / inv_w;
            let depth = nt * z0 + t * z1;

            let base_color = material.sample(uv);
            let color = self.shader.shade(base_color, pos, norm);

            self.fill_rect(min.as_vec2u(), max.as_vec2u(), color, depth);

            if p0 == p1 {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                error += dy;
                p0.x += sx;
            }
            if e2 <= dx {
                error += dx;
                p0.y += sy;
            }

            i += 1;
        }
    }

    fn fill_triangle(&mut self, v0: Vertex, v1: Vertex, v2: Vertex, material: &Material) {
        let (p0, z0, w0) = self.project(v0.position);
        let (p1, z1, w1) = self.project(v1.position);
        let (p2, z2, w2) = self.project(v2.position);

        let area = (p1 - p0).cross(p2 - p0);

        // back face culling
        if area <= 0 {
            return;
        }

        let inv_area = 1.0 / area as f32;

        let bbox_min = Vec2I::new(p0.x.min(p1.x.min(p2.x)), p0.y.min(p1.y.min(p2.y)));
        let bbox_max = Vec2I::new(p0.x.max(p1.x.max(p2.x)), p0.y.max(p1.y.max(p2.y)));

        fn top_left_bias(a: Vec2I, b: Vec2I) -> i32 {
            let edge = b - a;
            let is_top = edge.y == 0 && edge.x > 0;
            let is_left = edge.y < 0;

            if is_top || is_left { 0 } else { -1 }
        }

        let bias = Vec3I::new(
            top_left_bias(p1, p2),
            top_left_bias(p2, p0),
            top_left_bias(p0, p1),
        );

        let delta_p0 = p1 - p2;
        let delta_p1 = p2 - p0;
        let delta_p2 = p0 - p1;

        let mut e0_x = (p2 - p1).cross(bbox_min - p1);
        let mut e1_x = (p0 - p2).cross(bbox_min - p2);
        let mut e2_x = (p1 - p0).cross(bbox_min - p0);

        let v0_w = v0.position / w0;
        let v1_w = v1.position / w1;
        let v2_w = v2.position / w2;

        let uv0_w = v0.texcoord / w0;
        let uv1_w = v1.texcoord / w1;
        let uv2_w = v2.texcoord / w2;

        let n0_w = v0.normal / w0;
        let n1_w = v1.normal / w1;
        let n2_w = v2.normal / w2;

        for y in bbox_min.y..=bbox_max.y {
            let mut e0 = e0_x;
            let mut e1 = e1_x;
            let mut e2 = e2_x;
            for x in bbox_min.x..=bbox_max.x {
                let is_inside = e0 + bias.x >= 0 && e1 + bias.y >= 0 && e2 + bias.z >= 0;

                if is_inside {
                    let pixel = Vec2I::new(x, y);

                    let bary = Vec3::new(
                        e0 as f32 * inv_area,
                        e1 as f32 * inv_area,
                        e2 as f32 * inv_area,
                    );

                    let inv_w = bary.x / w0 + bary.y / w1 + bary.z / w2;
                    let pos = math::bary_lerp(bary, v0_w, v1_w, v2_w) / inv_w;
                    let uv = math::bary_lerp(bary, uv0_w, uv1_w, uv2_w) / inv_w;
                    let norm = math::bary_lerp(bary, n0_w, n1_w, n2_w) / inv_w;
                    let depth = math::bary_lerp(bary, z0, z1, z2);

                    let base_color = material.sample(uv);
                    let color = self.shader.shade(base_color, pos, norm);

                    self.framebuffer.set_pixel(pixel.as_vec2u(), color, depth);
                }
                e0 += delta_p0.y;
                e1 += delta_p1.y;
                e2 += delta_p2.y;
            }
            e0_x -= delta_p0.x;
            e1_x -= delta_p1.x;
            e2_x -= delta_p2.x;
        }
    }

    fn draw_triangle(&mut self, v0: Vertex, v1: Vertex, v2: Vertex, material: &Material) {
        match self.polygon_mode {
            PolygonMode::Point(size) => {
                self.draw_point(v0, material, size);
                self.draw_point(v1, material, size);
                self.draw_point(v2, material, size);
            }
            PolygonMode::Line(width) => {
                self.draw_line(v0, v1, material, width);
                self.draw_line(v1, v2, material, width);
                self.draw_line(v2, v0, material, width);
            }
            PolygonMode::Fill => self.fill_triangle(v0, v1, v2, material),
        }
    }

    pub fn draw_mesh(&mut self, mesh: &Mesh, material: &Material) {
        for i in 0..mesh.faces.len() {
            let mut v0 = mesh.get_vertex(i, 0);
            let mut v1 = mesh.get_vertex(i, 1);
            let mut v2 = mesh.get_vertex(i, 2);

            if mesh.normals.is_empty() {
                let normal = (v1.position - v0.position)
                    .cross(v2.position - v0.position)
                    .normalize();

                v0.normal = normal;
                v1.normal = normal;
                v2.normal = normal;
            }

            self.draw_triangle(v0, v1, v2, material);
        }
    }
}
