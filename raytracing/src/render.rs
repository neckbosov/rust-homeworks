use std::io::{self, BufWriter, Write};

use crate::sphere::{scene_intersect, SceneIntersection, Sphere};
use crate::vec3f::Vec3f;

pub struct Light {
    pub position: Vec3f,
    pub intensity: f32,
}

fn reflect(v: Vec3f, norm: Vec3f) -> Vec3f {
    v - norm * 2.0 * (v * norm)
}

fn cast_ray(origin: Vec3f, direction: Vec3f, spheres: &[Sphere], lights: &[Light]) -> Vec3f {
    if let Some(SceneIntersection { distance: _, hit, normal, material }) = scene_intersect(origin, direction, &spheres) {
        let mut diffuse_light_intensity = 0.0;
        let mut spectacular_light_intensity = 0.0;
        for light in lights {
            let mut light_direction = light.position - hit;
            light_direction.normalize();
            let light_distance = (light.position - hit).norm();

            let shadow_orig = if light_direction * normal < 0.0 {
                hit - normal * 1e-3
            } else {
                hit + normal * 1e-3
            };
            if let Some(intersection) = scene_intersect(shadow_orig, light_direction, &spheres) {
                if (intersection.hit - shadow_orig).norm() < light_distance {
                    continue;
                }
            }

            diffuse_light_intensity += light.intensity * 0.0f32.max(light_direction * normal);
            spectacular_light_intensity += 0.0f32
                .max(reflect(light_direction, normal) * direction)
                .powf(material.spectacular_component) * light.intensity;
        }
        material.diffuse_color * diffuse_light_intensity * material.albedo.0 +
            Vec3f::new(1.0, 1.0, 1.0) * spectacular_light_intensity * material.albedo.1
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
    for mut vector in frame_buffer {
        let max_coordinate = vector[0].max(vector[1]).max(vector[2]);
        if max_coordinate > 1.0 {
            vector = vector * (1.0 / max_coordinate);
        }
        for i in 0..3 {
            buffer.push((255.0 * vector[i].clamp(0.0, 1.0)) as u8);
        }
    }
    writer.write_all(&buffer)?;
    Ok(())
}
