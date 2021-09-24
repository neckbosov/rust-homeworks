use std::ops::Mul;

use crate::primitives::{Light, Material, Ray};
use crate::vec::Vec3f;

#[derive(Copy, Clone, Debug, Default)]
pub struct RayIntersection {
    pub distance: f32,
    pub hit: Vec3f,
    pub normal: Vec3f,
    pub material: Material,
}

pub trait Renderable {
    fn ray_intersect(&self, ray: Ray) -> Option<RayIntersection>;
}

impl<I: AsRef<[Box<dyn Renderable>]>> Renderable for I {
    fn ray_intersect(&self, ray: Ray) -> Option<RayIntersection> {
        self.as_ref().iter()
            .map(|obj| obj.ray_intersect(ray))
            .flatten()
            .min_by(|ri1, ri2| ri1.distance.partial_cmp(&ri2.distance).unwrap())
    }
}

pub struct Scene {
    pub objects: Vec<Box<dyn Renderable>>,
    pub lights: Vec<Light>,
    pub background_color: Vec3f,
}

fn reflect(v: Vec3f, norm: Vec3f) -> Vec3f {
    v - norm * 2.0 * (v * norm)
}

fn refract(v: Vec3f, norm: Vec3f, refractive_index: f32) -> Vec3f {
    let mut cosi = -(v * norm).clamp(-1.0, 1.0);
    let mut etai = 1.0;
    let mut etat = refractive_index;
    let mut n = norm;
    if cosi < 0.0 {
        cosi = -cosi;
        std::mem::swap(&mut etai, &mut etat);
        n = Vec3f::default() - norm;
    }
    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0 {
        Vec3f::new(1.0, 0.0, 0.0)
    } else {
        v * eta + n * (eta * cosi - k.sqrt())
    }
}

impl Scene {
    pub fn cast_ray(&self, ray: Ray, depth: usize) -> Vec3f {
        if depth == 0 {
            return self.background_color;
        }
        if let Some(RayIntersection {
                        distance: _,
                        hit,
                        normal,
                        material,
                    }) = self.objects.ray_intersect(ray)
        {
            let reflect_direction = reflect(ray.direction, normal).normalized();
            let refract_direction =
                refract(ray.direction, normal, material.refractive_index).normalized();
            let eps = 1e-3;
            let reflect_orig = if reflect_direction * normal < 0.0 {
                hit - normal * eps
            } else {
                hit + normal * eps
            };
            let refract_orig = if refract_direction * normal < 0.0 {
                hit - normal * eps
            } else {
                hit + normal * eps
            };

            let reflect_color = self.cast_ray(
                Ray {
                    origin: reflect_orig,
                    direction: reflect_direction,
                },
                depth - 1,
            );
            let refract_color = self.cast_ray(
                Ray {
                    origin: refract_orig,
                    direction: refract_direction,
                },
                depth - 1,
            );

            let mut diffuse_light_intensity = 0.0;
            let mut spectacular_light_intensity = 0.0;
            for light in &self.lights {
                let light_direction = (light.position - hit).normalized();
                let light_distance = (light.position - hit).norm();

                let shadow_orig = if light_direction * normal < 0.0 {
                    hit - normal * 1e-3
                } else {
                    hit + normal * 1e-3
                };
                if let Some(intersection) = self.objects.ray_intersect(Ray {
                    origin: shadow_orig,
                    direction: light_direction,
                }) {
                    if (intersection.hit - shadow_orig).norm() < light_distance {
                        continue;
                    }
                }

                diffuse_light_intensity += light.intensity * 0.0f32.max(light_direction * normal);
                spectacular_light_intensity += 0.0f32
                    .max(reflect(light_direction, normal) * ray.direction)
                    .powf(material.spectacular_component)
                    .mul(light.intensity);
            }
            material.diffuse_color * diffuse_light_intensity * material.albedo[0]
                + Vec3f::new(1.0, 1.0, 1.0) * spectacular_light_intensity * material.albedo[1]
                + reflect_color * material.albedo[2]
                + refract_color * material.albedo[3]
        } else {
            self.background_color
        }
    }
}
