use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct Framebuffer {
    width: usize,
    height: usize,
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

    // fn clear_color(&mut self) {}
    // fn clear_depth(&mut self) {}

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &[u8], depth: f32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = y * self.width + x;
        if depth >= self.depth[index] {
            return;
        }
        let base = index * 3;
        self.color[base..base + 3].copy_from_slice(color);
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
