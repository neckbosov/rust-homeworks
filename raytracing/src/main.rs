use raytracing::render::{Light, render};
use raytracing::sphere::{Material, Sphere};
use raytracing::vec3f::Vec3f;

fn main() -> std::io::Result<()> {
    let ivory = Material { diffuse_color: Vec3f::new(0.4, 0.4, 0.3) };
    let red_rubber = Material { diffuse_color: Vec3f::new(0.3, 0.1, 0.1) };
    let mut spheres = Vec::new();
    spheres.push(Sphere { center: Vec3f::new(-3.0, 0.0, -16.0), radius: 2.0, material: ivory });
    spheres.push(Sphere { center: Vec3f::new(-1.0, -1.5, -12.0), radius: 2.0, material: red_rubber });
    spheres.push(Sphere { center: Vec3f::new(-1.5, -0.5, -18.0), radius: 3.0, material: red_rubber });
    spheres.push(Sphere { center: Vec3f::new(7.0, 5.0, -18.0), radius: 4.0, material: ivory });

    let mut lights = Vec::new();
    lights.push(Light { position: Vec3f::new(-20.0, 20.0, 20.0), intensity: 1.5 });
    render(&spheres, &lights)
}
