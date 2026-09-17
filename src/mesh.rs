use crate::math::Vec3;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<[usize; 3]>,
}

impl Mesh {
    pub fn load_obj(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut vertices = Vec::new();
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
                Some("f") => {
                    let mut f = [0; 3];
                    for i in 0..3 {
                        if let Some(word) = words.next() {
                            let f_str = word.split('/').next().unwrap();
                            f[i] = f_str.parse::<usize>().unwrap_or(1) - 1;
                        }
                    }
                    faces.push(f);
                }
                _ => {}
            }
        }
        Ok(Self { vertices, faces })
    }
}
