use crate::sphere::{Material, Sphere};
use crate::vec::Vec3f;

#[derive(Copy, Clone, Debug, Default)]
pub struct SceneIntersection {
    pub hit: Vec3f,
    pub normal: Vec3f,
    pub material: Material,
}

pub fn scene_intersect(
    origin: Vec3f,
    direction: Vec3f,
    spheres: &[Sphere],
) -> Option<SceneIntersection> {
    let mut res: Option<SceneIntersection> = None;
    let mut distance: Option<f32> = None;
    for sphere in spheres {
        if let Some(dist) = sphere.ray_intersect(origin, direction) {
            if distance.filter(|val| *val < dist).is_none() {
                distance = Some(dist);
                let hit = origin + direction * dist;
                let mut normal = hit - sphere.center;
                normal.normalize();
                res = Some(SceneIntersection {
                    hit,
                    normal,
                    material: sphere.material,
                })
            }
        }
    }
    if direction[1].abs() > 1e-3 {
        let dist = -(origin[1] + 4.0) / direction[1];
        let point = origin + direction * dist;

        if dist > 0.0
            && point[0].abs() < 10.0
            && point[2] < -10.0
            && point[2] > -30.0
            && distance.filter(|val| *val <= dist).is_none()
        {
            let mut material = if let Some(intersection) = res {
                intersection.material
            } else {
                Material::default()
            };
            material.diffuse_color =
                if ((0.5 * point[0] + 1000.0) as i32 + (0.5 * point[2]) as i32) % 2 == 1 {
                    Vec3f::new(1.0, 1.0, 1.0)
                } else {
                    Vec3f::new(1.0, 0.7, 0.3)
                } * 0.3;
            res = Some(SceneIntersection {
                hit: point,
                normal: Vec3f::new(0.0, 1.0, 0.0),
                material,
            })
        }
    }
    res
}
