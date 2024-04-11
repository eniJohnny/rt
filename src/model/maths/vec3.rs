use std::ops::{Add, Mul, Sub, AddAssign, SubAssign, MulAssign, Neg};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64
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
	
    pub fn dot(&self, other: &Self) -> f64 {
		self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
		Self {
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x
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
			z: self.z / norm
		}
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + &rhs.x,
            y: self.y + &rhs.y,
            z: self.z + &rhs.z
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

impl AddAssign for Vec3 {
	fn add_assign(&mut self, rhs: Self) {
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

impl MulAssign<Self> for Vec3 {
	fn mul_assign(&mut self, rhs: Self) {
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

impl Neg for Vec3 {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z
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
    use crate::model::maths::vec3::Vec3;

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
	fn test_neg() {
		let v1: Vec3 = Vec3::new(1., 2., 3.);
		assert_eq!(-v1, Vec3::new(-1., -2., -3.))
	}
}