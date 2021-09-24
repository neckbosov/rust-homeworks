use crate::primitives::{Material, Ray};
use crate::scene::{RayIntersection, Renderable};
use crate::vec::Vec3f;

#[derive(Copy, Clone, Debug, Default)]
pub struct Checkerboard;

impl Renderable for Checkerboard {
    fn ray_intersect(&self, ray: Ray) -> Option<RayIntersection> {
        if ray.direction[1].abs() <= 1e-3 {
            return None;
        }
        let dist = -(ray.origin[1] + 4.0) / ray.direction[1];
        let point = ray.origin + ray.direction * dist;

        if dist > 0.0 && point[0].abs() < 10.0 && point[2] < -10.0 && point[2] > -30.0 {
            let even = ((0.5 * point[0] + 1000.0) as i32 + (0.5 * point[2]) as i32) % 2 == 1;
            let mut material = Material::default();
            material.diffuse_color = if even {
                Vec3f::new(1.0, 1.0, 1.0)
            } else {
                Vec3f::new(1.0, 0.7, 0.3)
            } * 0.3;
            Some(RayIntersection {
                distance: dist,
                hit: point,
                normal: Vec3f::new(0.0, 1.0, 0.0),
                material,
            })
        } else {
            None
        }
    }
}
