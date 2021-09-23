use std::ops::{Add, Index, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct MyVec<const N: usize> {
    pub coordinates: [f32; N],
}

impl<const N: usize> Default for MyVec<N> {
    fn default() -> Self {
        Self {
            coordinates: [0.0; N]
        }
    }
}

pub type Vec3f = MyVec<3>;
pub type Vec4f = MyVec<4>;

impl<const N: usize> Index<usize> for MyVec<N> {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.coordinates[index]
    }
}

impl<const N: usize> Add for MyVec<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = Self::default();
        for i in 0..self.coordinates.len() {
            res.coordinates[i] = self.coordinates[i] + rhs.coordinates[i];
        }
        res
    }
}

impl<const N: usize> Sub for MyVec<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = Self::default();
        for i in 0..self.coordinates.len() {
            res.coordinates[i] = self.coordinates[i] - rhs.coordinates[i];
        }
        res
    }
}

impl<const N: usize> Mul for MyVec<N> {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = 0.0;
        for i in 0..self.coordinates.len() {
            res += self.coordinates[i] * rhs.coordinates[i];
        }
        res
    }
}

impl<const N: usize> Mul<f32> for MyVec<N> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut res = Self::default();
        for i in 0..self.coordinates.len() {
            res.coordinates[i] = self.coordinates[i] * rhs;
        }
        res
    }
}

impl<const N: usize> MyVec<N> {
    pub fn normalize(&mut self) {
        let norm = self.norm();
        for coord in &mut self.coordinates {
            *coord /= norm;
        }
    }
    pub fn norm(&self) -> f32 {
        (*self * *self).sqrt()
    }
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            coordinates: [x, y, z]
        }
    }
}