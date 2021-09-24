use std::ops::{Add, Index, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec3f {
    pub coordinates: [f32; 3],
}

impl Default for Vec3f {
    fn default() -> Self {
        Self {
            coordinates: [0.0; 3],
        }
    }
}

impl Index<usize> for Vec3f {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coordinates[index]
    }
}

impl Add for Vec3f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Self::default();
        for i in 0..self.coordinates.len() {
            res.coordinates[i] = self.coordinates[i] + rhs.coordinates[i];
        }
        res
    }
}

impl Sub for Vec3f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Self::default();
        for i in 0..self.coordinates.len() {
            res.coordinates[i] = self.coordinates[i] - rhs.coordinates[i];
        }
        res
    }
}

impl Mul for Vec3f {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = 0.0;
        for i in 0..self.coordinates.len() {
            res += self.coordinates[i] * rhs.coordinates[i];
        }
        res
    }
}

impl Mul<f32> for Vec3f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut res = Self::default();
        for i in 0..self.coordinates.len() {
            res.coordinates[i] = self.coordinates[i] * rhs;
        }
        res
    }
}

impl Vec3f {
    pub fn normalize(&mut self) {
        let norm = self.norm();
        for coord in &mut self.coordinates {
            *coord /= norm;
        }
    }
    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }
    pub fn norm(&self) -> f32 {
        (*self * *self).sqrt()
    }
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            coordinates: [x, y, z],
        }
    }
}
