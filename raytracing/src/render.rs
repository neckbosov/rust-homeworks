use std::io::{self, BufWriter, Write};

use crate::primitives::Ray;
use crate::scene::Scene;
use crate::vec::Vec3f;

#[derive(Debug, Copy, Clone)]
pub struct RenderParams {
    pub field_of_view: f32,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

pub fn render(scene: &Scene, params: RenderParams) -> io::Result<()> {
    let width = params.width;
    let height = params.height;
    let fov = params.field_of_view;

    let mut frame_buffer = vec![Vec3f::default(); width * height];
    for j in 0..height {
        for i in 0..width {
            let x =
                (2.0 * (i as f32 + 0.5) / width as f32 - 1.0) * (fov / 2.0).tan() * width as f32
                    / height as f32;
            let y = -(2.0 * (j as f32 + 0.5) / height as f32 - 1.0) * (fov / 2.0).tan();
            let mut direction = Vec3f::new(x, y, -1.0);
            direction.normalize();
            frame_buffer[i + j * width] = scene.cast_ray(
                Ray {
                    origin: Vec3f::new(0.0, 0.0, 0.0),
                    direction,
                },
                params.depth,
            );
        }
    }

    let mut raw_buffer = Vec::with_capacity(width * height * 3);
    for mut vector in frame_buffer {
        let max_coordinate = vector[0].max(vector[1]).max(vector[2]);
        if max_coordinate > 1.0 {
            vector = vector * (1.0 / max_coordinate);
        }
        for i in 0..3 {
            raw_buffer.push((255.0 * vector[i].clamp(0.0, 1.0)) as u8);
        }
    }

    let file = std::fs::File::create("./out.ppm")?;
    let mut writer = BufWriter::new(file);
    writeln!(&mut writer, "P6\n{} {}\n255", width, height)?;
    writer.write_all(&raw_buffer)?;
    Ok(())
}
