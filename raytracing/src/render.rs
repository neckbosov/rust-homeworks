use std::io::{self, BufWriter, Write};

use crate::sphere::{scene_intersect, Sphere};
use crate::vec3f::Vec3f;

pub struct Light {
    pub position: Vec3f,
    pub intensity: f32,
}

fn cast_ray(origin: Vec3f, direction: Vec3f, spheres: &[Sphere], lights: &[Light]) -> Vec3f {
    if let Some(intersection) = scene_intersect(origin, direction, &spheres) {
        let mut diffuse_light_intensity = 0.0;
        for light in lights {
            let mut light_direction = light.position - intersection.hit;
            light_direction.normalize();
            diffuse_light_intensity += light.intensity * 0.0f32.max(light_direction * intersection.normal);
        }
        intersection.material.diffuse_color * diffuse_light_intensity
    } else {
        Vec3f::new(0.2, 0.7, 0.8)
    }
}

pub fn render(spheres: &[Sphere], lights: &[Light]) -> io::Result<()> {
    let width = 1024;
    let height = 768;
    let fov = std::f32::consts::PI / 2.0;

    let mut frame_buffer = vec![Vec3f::default(); width * height];
    for j in 0..height {
        for i in 0..width {
            let x = (2.0 * (i as f32 + 0.5) / width as f32 - 1.0) * (fov / 2.0).tan() * width as f32 / height as f32;
            let y = -(2.0 * (j as f32 + 0.5) / height as f32 - 1.0) * (fov / 2.0).tan();
            let mut direction = Vec3f::new(x, y, -1.0);
            direction.normalize();
            frame_buffer[i + j * width] = cast_ray(Vec3f::new(0.0, 0.0, 0.0), direction, &spheres, &lights);
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
