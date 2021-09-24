use crate::primitives::{Material, Ray};
use crate::scene::{RayIntersection, Renderable};
use crate::vec::Vec3f;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
    pub material: Material,
}

impl Renderable for Sphere {
    /// Check if given ray with such `origin` and `direction` intersects sphere.
    ///
    /// Returns `None` if not, else returns `Some(dist)` where `dist` is distance from `origin` to sphere.
    fn ray_intersect(&self, ray: Ray) -> Option<RayIntersection> {
        let origin_to_center = self.center - ray.origin;
        let otc_ray_projection = origin_to_center * ray.direction;
        let center_to_ray_distance_sqr =
            origin_to_center * origin_to_center - otc_ray_projection * otc_ray_projection;
        if center_to_ray_distance_sqr > self.radius * self.radius {
            return None;
        }
        let radius_to_ray_projection =
            (self.radius * self.radius - center_to_ray_distance_sqr).sqrt();
        let origin_to_sphere_distance = {
            let t0 = otc_ray_projection - radius_to_ray_projection;
            let t1 = otc_ray_projection + radius_to_ray_projection;
            if t0 >= 0.0 {
                t0
            } else if t1 >= 0.0 {
                t1
            } else {
                return None;
            }
        };
        let hit = ray.origin + ray.direction * origin_to_sphere_distance;
        let normal = (hit - self.center).normalized();
        Some(RayIntersection {
            distance: origin_to_sphere_distance,
            hit,
            normal,
            material: self.material,
        })
    }
}
