use std::ops::{Add, Mul, Sub};

pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        unimplemented!()
    }

    pub fn cross(&self, other: &Self) -> Self {
        unimplemented!()
    }

    pub fn normalize(self) -> Self {
        unimplemented!()
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - &rhs.x,
            y: self.y - &rhs.y,
            z: self.z - &rhs.z
        }
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * &rhs.x,
            y: self.y * &rhs.y,
            z: self.z * &rhs.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}