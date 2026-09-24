use crate::math::{Vec2, Vec3};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

pub struct Texture {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Texture {
    pub fn load_ppm(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        reader.read_line(&mut line)?; // P6
        line.clear();
        reader.read_line(&mut line)?; // Image size

        let mut it = line.split_whitespace();
        let width: usize = it.next().unwrap().parse().unwrap();
        let height: usize = it.next().unwrap().parse().unwrap();

        line.clear();
        reader.read_line(&mut line)?; // 255

        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        Ok(Self {
            width,
            height,
            data,
        })
    }

    pub fn sample(&self, uv: Vec2) -> Vec3 {
        let x = ((uv.x * self.width as f32) as usize).min(self.width - 1);
        let y = (((1.0 - uv.y) * self.height as f32) as usize).min(self.height - 1);
        let i = (y * self.width + x) * 3;
        Vec3::new(
            self.data[i] as f32 / 255.0,
            self.data[i + 1] as f32 / 255.0,
            self.data[i + 2] as f32 / 255.0,
        )
    }
}
