use crate::math::{Vec2U, Vec3};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    color: Vec<u8>,
    depth: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        Self {
            width,
            height,
            color: vec![0; width * height * 3],
            depth: vec![f32::INFINITY; width * height],
        }
    }

    pub fn clear_color(&mut self, color: Vec3) {
        for chunk in self.color.as_chunks_mut::<3>().0 {
            chunk.copy_from_slice(&to_rgb8(color));
        }
    }

    // fn clear_depth(&mut self) {}

    pub fn set_pixel(&mut self, p: Vec2U, color: Vec3, depth: f32) {
        if p.x >= self.width || p.y >= self.height {
            return;
        }
        let index = p.y * self.width + p.x;
        if depth >= self.depth[index] {
            return;
        }
        let base = index * 3;
        self.color[base..base + 3].copy_from_slice(&to_rgb8(color));
        self.depth[index] = depth;
    }

    pub fn save_ppm(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "{}", "P6")?; // PPM magic number
        writeln!(writer, "{} {}", self.width, self.height)?; // Image size
        writeln!(writer, "{}", "255")?; // Max value

        writer.write_all(&self.color)?;
        writer.flush()
    }
}

fn to_rgb8(color: Vec3) -> [u8; 3] {
    [
        (color.x * 255.0) as u8,
        (color.y * 255.0) as u8,
        (color.z * 255.0) as u8,
    ]
}
