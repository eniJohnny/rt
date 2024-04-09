use std::ops::{Add, Mul, Sub};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Vec2 {
    x: f64,
    y: f64
}

impl Vec2 {
	pub fn new(x: f64, y: f64) -> Self {
		Self { x, y }
	}

	pub fn x(&self) -> &f64 {
		&self.x
	}

	pub fn y(&self) -> &f64 {
		&self.y
	}

	pub fn dot(&self, other: &Self) -> f64 {
		self.x * other.x + self.y * other.y
	}

	pub fn get_norm(&self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}

	pub fn normalize(&self) -> Self {
		let norm: f64 = self.get_norm();
		Self {
			x: self.x / norm,
			y: self.y / norm
		}
	}
}

impl Add for Vec2 {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + &rhs.x,
			y: self.y + &rhs.y
		}
	}
}

impl Sub for Vec2 {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - &rhs.x,
			y: self.y - &rhs.y
		}
	}
}

impl Mul<Self> for Vec2 {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x * rhs.x,
			y: self.y * rhs.y
		}
	}
}

impl Mul<f64> for Vec2 {
	type Output = Self;
	fn mul(self, rhs: f64) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs
		}
	}
}

impl PartialEq for Vec2 {
	fn eq(&self, other: &Self) -> bool {
		self.x == other.x && self.y == other.y
	}

	fn ne(&self, other: &Self) -> bool {
		self.x != other.x || self.y != other.y
	}
}

impl Display for Vec2 {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

#[cfg(test)]
mod tests {
	use crate::model::maths::vec2::Vec2;

	#[test]
	fn test_add() {
		let v1: Vec2 = Vec2::new(1., 2.);
		let v2: Vec2 = Vec2::new(3., 4.);
		assert_eq!(v1 + v2, Vec2::new(4., 6.));
	}

	#[test]
	fn test_sub() {
		let v1: Vec2 = Vec2::new(1., 2.);
		let v2: Vec2 = Vec2::new(3., 4.);
		assert_eq!(v1 - v2, Vec2::new(-2., -2.));
	}

	#[test]
	fn test_mul() {
		let v1: Vec2 = Vec2::new(1., 2.);
		let v2: Vec2 = Vec2::new(3., 4.);
		assert_eq!(v1 * v2, Vec2::new(3., 8.));
	}

	#[test]
	fn test_mul_scalar() {
		let v1: Vec2 = Vec2::new(1., 2.);
		assert_eq!(v1 * 2., Vec2::new(2., 4.));
	}

	#[test]
	fn test_dot() {
		let v1: Vec2 = Vec2::new(1., 2.);
		let v2: Vec2 = Vec2::new(3., 4.);
		assert_eq!(v1.dot(&v2), 11.);
	}

	#[test]
	fn test_get_norm() {
		let v1: Vec2 = Vec2::new(3., 4.);
		assert_eq!(v1.get_norm(), 5.);
	}

	#[test]
	fn test_normalize() {
		let v1: Vec2 = Vec2::new(3., 4.);
		assert_eq!(v1.normalize().get_norm(), 1.);
	}
}