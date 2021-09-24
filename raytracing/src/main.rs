use raytracing::checkerboard::Checkerboard;
use raytracing::primitives::{Light, Material};
use raytracing::render::{render, RenderParams};
use raytracing::scene::{Renderable, Scene};
use raytracing::sphere::Sphere;
use raytracing::vec::Vec3f;

fn main() -> std::io::Result<()> {
    let ivory = Material {
        albedo: [0.6, 0.3, 0.1, 0.0],
        diffuse_color: Vec3f::new(0.4, 0.4, 0.3),
        spectacular_component: 50.0,
        refractive_index: 1.0,
    };
    let glass = Material {
        albedo: [0.0, 0.5, 0.1, 0.8],
        diffuse_color: Vec3f::new(0.6, 0.7, 0.8),
        spectacular_component: 125.0,
        refractive_index: 1.5,
    };
    let red_rubber = Material {
        albedo: [0.9, 0.1, 0.0, 0.0],
        diffuse_color: Vec3f::new(0.3, 0.1, 0.1),
        spectacular_component: 10.0,
        refractive_index: 1.0,
    };
    let mirror = Material {
        albedo: [0.0, 10.0, 0.8, 0.0],
        diffuse_color: Vec3f::new(1.0, 1.0, 1.0),
        spectacular_component: 1425.0,
        refractive_index: 1.0,
    };

    let objects: Vec<Box<dyn Renderable>> = vec![
        Box::new(Sphere {
            center: Vec3f::new(-3.0, 0.0, -16.0),
            radius: 2.0,
            material: ivory,
        }),
        Box::new(Sphere {
            center: Vec3f::new(-1.0, -1.5, -12.0),
            radius: 2.0,
            material: glass,
        }),
        Box::new(Sphere {
            center: Vec3f::new(1.5, -0.5, -18.0),
            radius: 3.0,
            material: red_rubber,
        }),
        Box::new(Sphere {
            center: Vec3f::new(7.0, 5.0, -18.0),
            radius: 4.0,
            material: mirror,
        }),
        Box::new(Checkerboard),
    ];

    let lights = vec![
        Light {
            position: Vec3f::new(-20.0, 20.0, 20.0),
            intensity: 1.5,
        },
        Light {
            position: Vec3f::new(30.0, 50.0, -25.0),
            intensity: 1.8,
        },
        Light {
            position: Vec3f::new(30.0, 20.0, 30.0),
            intensity: 1.7,
        },
    ];
    let scene = Scene {
        objects,
        lights,
        background_color: Vec3f::new(0.2, 0.7, 0.8),
    };
    let params = RenderParams {
        field_of_view: std::f32::consts::PI / 2.0,
        width: 1024,
        height: 768,
        depth: 4,
    };
    render(&scene, params)
}
