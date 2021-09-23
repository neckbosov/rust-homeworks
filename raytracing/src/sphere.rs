use crate::vec::{Vec3f, Vec4f};

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub albedo: Vec4f,
    pub diffuse_color: Vec3f,
    pub spectacular_component: f32,
    pub refractive_index: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            albedo: Vec4f {
                coordinates: [1.0, 0.0, 0.0, 0.0],
            },
            diffuse_color: Vec3f::new(0.0, 0.0, 0.0),
            spectacular_component: 0.0,
            refractive_index: 1.0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    /// Check if given ray with such `origin` and `direction` intersects sphere.
    ///
    /// Returns `None` if not, else returns `Some(dist)` where `dist` is distance from `origin` to sphere.
    pub fn ray_intersect(&self, origin: Vec3f, direction: Vec3f) -> Option<f32> {
        let origin_to_center = self.center - origin;
        let otc_ray_projection = origin_to_center * direction;
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
        Some(origin_to_sphere_distance)
    }
}
