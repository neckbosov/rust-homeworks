use std::ops::{Index, Mul, Sub};

#[derive(Default, Copy, Clone, Debug)]
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

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3f {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Vec3f {
    pub fn normalize(&mut self) {
        let norm = (*self * *self).sqrt();
        self.x /= norm;
        self.y /= norm;
        self.z /= norm;
    }
}