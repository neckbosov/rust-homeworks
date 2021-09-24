use crate::vec::Vec3f;

#[derive(Copy, Clone, Debug)]
pub struct Material {
    pub albedo: [f32; 4],
    pub diffuse_color: Vec3f,
    pub spectacular_component: f32,
    pub refractive_index: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            albedo: [1.0, 0.0, 0.0, 0.0],
            diffuse_color: Vec3f::new(0.0, 0.0, 0.0),
            spectacular_component: 0.0,
            refractive_index: 1.0,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Ray {
    pub origin: Vec3f,
    pub direction: Vec3f,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Light {
    pub position: Vec3f,
    pub intensity: f32,
}