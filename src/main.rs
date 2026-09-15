use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let width = 1024;
    let height = 1024;
    let mut pixels = vec![0; width * height * 3];

    for y in 0..height {
        for x in 0..width {
            let offset = (y * width + x) * 3;
            pixels[offset + 0] = (x * 255 / width) as u8;
            pixels[offset + 1] = (y * 255 / height) as u8;
            pixels[offset + 2] = 0;
        }
    }

    let file = File::create("out.ppm")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{}", "P6")?; // PPM magic number
    writeln!(writer, "{} {}", width, height)?; // Image size
    writeln!(writer, "{}", "255")?; // Max value

    writer.write_all(&pixels)?;
    writer.flush()?;
    Ok(())
}
