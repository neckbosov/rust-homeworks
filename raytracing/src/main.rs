use raytracing::render::render;
use raytracing::sphere::{Material, Sphere};
use raytracing::vec3f::Vec3f;

fn main() -> std::io::Result<()> {
    let ivory = Material { diffuse_color: Vec3f { x: 0.4, y: 0.4, z: 0.3 } };
    let red_rubber = Material { diffuse_color: Vec3f { x: 0.3, y: 0.1, z: 0.1 } };
    let mut spheres = Vec::new();
    spheres.push(Sphere { center: Vec3f { x: -3.0, y: 0.0, z: -16.0 }, radius: 2.0, material: ivory });
    spheres.push(Sphere { center: Vec3f { x: -1.0, y: -1.5, z: -12.0 }, radius: 2.0, material: red_rubber });
    spheres.push(Sphere { center: Vec3f { x: -1.5, y: -0.5, z: -18.0 }, radius: 3.0, material: red_rubber });
    spheres.push(Sphere { center: Vec3f { x: 7.0, y: 5.0, z: -18.0 }, radius: 4.0, material: ivory });
    render(&spheres)
}
