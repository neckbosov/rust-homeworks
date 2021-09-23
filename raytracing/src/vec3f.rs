use std::ops::Index;

#[derive(Default, Copy, Clone)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Index<usize> for Vec3f {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            ind => panic!("Index {} is out of bounds 0..3", ind)
        }
    }
}
