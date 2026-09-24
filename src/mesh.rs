use crate::math::{Vec2, Vec3};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub texcoord: Vec2,
    pub normal: Vec3,
}

#[derive(Default)]
pub struct Face {
    pub pos_indices: [usize; 3],
    pub uv_indices: [usize; 3],
    pub norm_indices: [usize; 3],
}

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub texcoords: Vec<Vec2>,
    pub normals: Vec<Vec3>,
    pub faces: Vec<Face>,
}

impl Mesh {
    pub fn load_obj(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut texcoords = Vec::new();
        let mut faces = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            let mut words = line.split_whitespace();
            match words.next() {
                Some("v") => {
                    let mut v = [0.0; 3];
                    for i in 0..3 {
                        if let Some(word) = words.next() {
                            v[i] = word.parse::<f32>().unwrap_or(0.0);
                        }
                    }
                    vertices.push(Vec3::from_array(v));
                }
                Some("vt") => {
                    let mut vt = [0.0; 2];
                    for i in 0..2 {
                        if let Some(word) = words.next() {
                            vt[i] = word.parse::<f32>().unwrap_or(0.0);
                        }
                    }
                    texcoords.push(Vec2::from_array(vt));
                }
                Some("vn") => {
                    let mut vn = [0.0; 3];
                    for i in 0..3 {
                        if let Some(word) = words.next() {
                            vn[i] = word.parse::<f32>().unwrap_or(0.0);
                        }
                    }
                    normals.push(Vec3::from_array(vn));
                }
                Some("f") => {
                    let mut f = Face::default();
                    for i in 0..3 {
                        if let Some(word) = words.next() {
                            let mut parts = word.split('/');
                            let pos = parts.next().unwrap();
                            f.pos_indices[i] = pos.parse::<usize>().unwrap_or(1) - 1;
                            if let Some(uv) = parts.next() {
                                f.uv_indices[i] = uv.parse::<usize>().unwrap_or(1) - 1;
                            }
                            if let Some(norm) = parts.next() {
                                f.norm_indices[i] = norm.parse::<usize>().unwrap_or(1) - 1;
                            }
                        }
                    }
                    faces.push(f);
                }
                _ => {}
            }
        }
        Ok(Self {
            vertices,
            texcoords,
            normals,
            faces,
        })
    }

    pub fn get_vertex(&self, face_index: usize, vertex_index: usize) -> Vertex {
        let position = self
            .vertices
            .get(self.faces[face_index].pos_indices[vertex_index])
            .cloned()
            .unwrap_or(Vec3::splat(0.0));
        let texcoord = self
            .texcoords
            .get(self.faces[face_index].uv_indices[vertex_index])
            .cloned()
            .unwrap_or(Vec2::splat(0.0));
        let normal = self
            .normals
            .get(self.faces[face_index].norm_indices[vertex_index])
            .cloned()
            .unwrap_or(Vec3::splat(0.0));

        Vertex {
            position,
            texcoord,
            normal,
        }
    }
}
