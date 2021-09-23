use raytracing::vec3f::Vec3f;
use std::io::{BufWriter, Write};
use std::ops::Index;

fn render() -> std::io::Result<()> {
    let width = 1024;
    let height = 768;
    let mut frame_buffer = vec![Vec3f::default(); width * height];
    for j in 0..height {
        for i in 0..width {
            frame_buffer[i + j * width] = Vec3f {
                x: j as f32 / height as f32,
                y: i as f32 / width as f32,
                z: 0.0,
            }
        }
    }
    let file = std::fs::File::create("./out.ppm")?;
    let mut writer = BufWriter::new(file);
    writeln!(&mut writer, "P6\n{} {}\n255", width, height)?;
    let mut buffer = Vec::with_capacity(width * height * 3);
    for vector in frame_buffer {
        for i in 0..3 {
            buffer.push((255.0 * vector[i].clamp(0.0, 1.0)) as u8);
        }
    }
    writer.write_all(&buffer)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    render()?;
    Ok(())
}
