use crate::model::materials::color::Color;

use super::quaternion::Quaternion;
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn x(&self) -> &f64 {
        &self.x
    }

    pub fn y(&self) -> &f64 {
        &self.y
    }

    pub fn z(&self) -> &f64 {
        &self.z
    }

    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }
    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn dot_consume(&self, other: Self) -> f64 {
        self.dot(&other)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Self {
        let norm: f64 = self.length();
        Self {
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }

    pub fn to_quaternion(&self, w: f64) -> Quaternion {
        Quaternion::new(self.x, self.y, self.z, w)
    }

    pub fn rotate(&self, q: &Quaternion) -> Self {
        let q_conj = q.conjugate();
        let p = self.to_quaternion(0.);
        (q * p * q_conj).to_vec3()
    }

    pub fn rotate_from_axis_angle(&self, angle: f64, axis: &Self) -> Self {
        let q = Quaternion::new_from_axis_angle(axis, angle);
        self.rotate(&q)
    }

    pub fn from_value(value: f64) -> Self {
        Vec3::new(value, value, value)
    }

    pub fn to_value(&self) -> f64 {
        (self.x + self.y + self.z) / 3.
    }

    pub fn from_color(color: Color) -> Self {
        Vec3::new(color.r(), color.g(), color.b())
    }

    pub fn abs(&self) -> Self {
        Vec3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub fn max(&self) -> f64 {
        self.x.max(self.y).max(self.z)
    }

    pub fn min(&self) -> f64 {
        self.x.min(self.y).min(self.z)
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: &Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<&f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: &f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<&f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: &f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Self::Output) -> Self::Output {
        Self::Output {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Self::Output) -> Self::Output {
        Self::Output {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0. {
            panic!("Division by zero");
        }
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<&f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: &f64) -> Self::Output {
        if *rhs == 0. {
            panic!("Division by zero");
        }
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        if rhs == 0. {
            panic!("Division by zero");
        }
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<&f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: &f64) -> Self::Output {
        if *rhs == 0. {
            panic!("Division by zero");
        }
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<&Vec3> for f64 {
    type Output = Vec3;
    fn div(self, rhs: &Vec3) -> Self::Output {
        if rhs.x == 0. || rhs.y == 0. || rhs.z == 0. {
            panic!("Division by zero");
        }
        Vec3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<&Self> for Vec3 {
    fn add_assign(&mut self, rhs: &Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<&Self> for Vec3 {
    fn sub_assign(&mut self, rhs: &Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign<Self> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<&Self> for Vec3 {
    fn mul_assign(&mut self, rhs: &Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl MulAssign<&f64> for Vec3 {
    fn mul_assign(&mut self, rhs: &f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        if rhs == 0. {
            panic!("Division by zero");
        }
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl DivAssign<&f64> for Vec3 {
    fn div_assign(&mut self, rhs: &f64) {
        if *rhs == 0. {
            panic!("Division by zero");
        }
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn test_add() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        let v2: Vec3 = Vec3::new(4., 5., 6.);
        assert_eq!(v1 + v2, Vec3::new(5., 7., 9.))
    }

    #[test]
    fn test_sub() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        let v2: Vec3 = Vec3::new(4., 5., 6.);
        assert_eq!(v1 - v2, Vec3::new(-3., -3., -3.))
    }

    #[test]
    fn test_mul() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        let v2: Vec3 = Vec3::new(4., 5., 6.);
        assert_eq!(v1 * v2, Vec3::new(4., 10., 18.))
    }

    #[test]
    fn test_mul_scalar() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        assert_eq!(v1 * 2., Vec3::new(2., 4., 6.))
    }

    #[test]
    fn test_div_scalar() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        assert_eq!(v1 / 2., Vec3::new(0.5, 1., 1.5))
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_div_zero() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        let _ = v1 / 0.;
    }

    #[test]
    fn test_dot() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        let v2: Vec3 = Vec3::new(4., 5., 6.);
        assert_eq!(v1.dot(&v2), 32.)
    }

    #[test]
    fn test_cross() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        let v2: Vec3 = Vec3::new(4., 5., 6.);
        assert_eq!(v1.cross(&v2), Vec3::new(-3., 6., -3.))
    }

    #[test]
    fn test_length() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        assert_eq!(v1.length(), 14_f64.sqrt())
    }

    #[test]
    fn test_normalize() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        assert_eq!(v1.normalize().length(), 1.)
    }

    #[test]
    fn test_add_assign() {
        let mut v1: Vec3 = Vec3::new(1., 2., 3.);
        let v2: Vec3 = Vec3::new(4., 5., 6.);
        v1 += v2;
        assert_eq!(v1, Vec3::new(5., 7., 9.))
    }

    #[test]
    fn test_sub_assign() {
        let mut v1: Vec3 = Vec3::new(1., 2., 3.);
        let v2: Vec3 = Vec3::new(4., 5., 6.);
        v1 -= v2;
        assert_eq!(v1, Vec3::new(-3., -3., -3.))
    }

    #[test]
    fn test_mul_assign() {
        let mut v1: Vec3 = Vec3::new(1., 2., 3.);
        let v2: Vec3 = Vec3::new(4., 5., 6.);
        v1 *= v2;
        assert_eq!(v1, Vec3::new(4., 10., 18.))
    }

    #[test]
    fn test_mul_assign_scalar() {
        let mut v1: Vec3 = Vec3::new(1., 2., 3.);
        v1 *= 2.;
        assert_eq!(v1, Vec3::new(2., 4., 6.))
    }

    #[test]
    fn test_div_assign() {
        let mut v1: Vec3 = Vec3::new(1., 2., 3.);
        v1 /= 2.;
        assert_eq!(v1, Vec3::new(0.5, 1., 1.5))
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn test_div_assign_zero() {
        let mut v1: Vec3 = Vec3::new(1., 2., 3.);
        v1 /= 0.;
    }

    #[test]
    fn test_neg() {
        let v1: Vec3 = Vec3::new(1., 2., 3.);
        assert_eq!(-v1, Vec3::new(-1., -2., -3.))
    }

    #[test]
    fn test_rotate() {
        let v1: Vec3 = Vec3::new(1., 0., 0.);
        let axis: Vec3 = Vec3::new(0., 0., 1.);
        let rotated = v1.rotate_from_axis_angle(std::f64::consts::PI / 2., &axis);
        assert!((rotated.x() - 0.).abs() <= f64::EPSILON);
        assert!((rotated.y() - 1.).abs() <= f64::EPSILON);
        assert!((rotated.z() - 0.).abs() <= f64::EPSILON);
    }
}
