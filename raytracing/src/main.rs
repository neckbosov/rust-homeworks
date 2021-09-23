use raytracing::render::render;
use raytracing::sphere::Sphere;
use raytracing::vec3f::Vec3f;

fn main() -> std::io::Result<()> {
    let sphere = Sphere { center: Vec3f { x: -3.0, y: 0.0, z: -16.0 }, radius: 2.0 };
    render(sphere)
}
