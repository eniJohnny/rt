use std::ops::{Add, Mul, Sub, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug)]
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

	pub fn length(&self) -> f64 {
		(self.x * self.x + self.y * self.y).sqrt()
	}

	pub fn normalize(&self) -> Self {
		let norm: f64 = self.length();
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
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
	}
}

impl Add for &Vec2 {
	type Output = Vec2;
	fn add(self, rhs: Self) -> Self::Output {
		Vec2 {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
	}
}

impl Add<Vec2> for &Vec2 {
	type Output = Vec2;
	fn add(self, rhs: Vec2) -> Self::Output {
		Vec2 {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
	}
}

impl Add<&Self> for Vec2 {
	type Output = Self;
	fn add(self, rhs: &Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
	}
}

impl Sub for Vec2 {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
	}
}

impl Sub for &Vec2 {
	type Output = Vec2;
	fn sub(self, rhs: Self) -> Self::Output {
		Vec2 {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
	}
}

impl Sub<Vec2> for &Vec2 {
	type Output = Vec2;
	fn sub(self, rhs: Vec2) -> Self::Output {
		Vec2 {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
	}
}

impl Sub<&Self> for Vec2 {
	type Output = Self;
	fn sub(self, rhs: &Self) -> Self::Output {
		Self {
			x: self.x - rhs.x,
			y: self.y - rhs.y
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

impl Mul<Self> for &Vec2 {
	type Output = Vec2;
	fn mul(self, rhs: Self) -> Self::Output {
		Vec2 {
			x: self.x * rhs.x,
			y: self.y * rhs.y
		}
	}
}

impl Mul<Vec2> for &Vec2 {
	type Output = Vec2;
	fn mul(self, rhs: Vec2) -> Self::Output {
		Vec2 {
			x: self.x * rhs.x,
			y: self.y * rhs.y
		}
	}
}

impl Mul<&Self> for Vec2 {
	type Output = Self;
	fn mul(self, rhs: &Self) -> Self::Output {
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

impl Mul<f64> for &Vec2 {
	type Output = Vec2;
	fn mul(self, rhs: f64) -> Self::Output {
		Vec2 {
			x: self.x * rhs,
			y: self.y * rhs
		}
	}
}

impl Mul<&f64> for &Vec2 {
	type Output = Vec2;
	fn mul(self, rhs: &f64) -> Self::Output {
		Vec2 {
			x: self.x * rhs,
			y: self.y * rhs
		}
	}
}

impl Mul<&f64> for Vec2 {
	type Output = Self;
	fn mul(self, rhs: &f64) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs
		}
	}
}

impl Div<f64> for Vec2 {
	type Output = Self;
	fn div(self, rhs: f64) -> Self::Output {
		if rhs == 0. {
			panic!("Division by zero");
		}
		Self {
			x: self.x / rhs,
			y: self.y / rhs
		}
	}
}

impl Div<f64> for &Vec2 {
	type Output = Vec2;
	fn div(self, rhs: f64) -> Self::Output {
		if rhs == 0. {
			panic!("Division by zero");
		}
		Vec2 {
			x: self.x / rhs,
			y: self.y / rhs
		}
	}
}

impl Div<&f64> for &Vec2 {
	type Output = Vec2;
	fn div(self, rhs: &f64) -> Self::Output {
		if *rhs == 0. {
			panic!("Division by zero");
		}
		Vec2 {
			x: self.x / rhs,
			y: self.y / rhs
		}
	}
}

impl Div<&f64> for Vec2 {
	type Output = Self;
	fn div(self, rhs: &f64) -> Self::Output {
		if *rhs == 0. {
			panic!("Division by zero");
		}
		Self {
			x: self.x / rhs,
			y: self.y / rhs
		}
	}
}

impl AddAssign for Vec2 {
	fn add_assign(&mut self, rhs: Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl AddAssign<&Self> for Vec2 {
	fn add_assign(&mut self, rhs: &Self) {
		self.x += rhs.x;
		self.y += rhs.y;
	}
}

impl SubAssign for Vec2 {
	fn sub_assign(&mut self, rhs: Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl SubAssign<&Self> for Vec2 {
	fn sub_assign(&mut self, rhs: &Self) {
		self.x -= rhs.x;
		self.y -= rhs.y;
	}
}

impl MulAssign<Self> for Vec2 {
	fn mul_assign(&mut self, rhs: Self) {
		self.x *= rhs.x;
		self.y *= rhs.y;
	}
}

impl MulAssign<&Self> for Vec2 {
	fn mul_assign(&mut self, rhs: &Self) {
		self.x *= rhs.x;
		self.y *= rhs.y;
	}
}

impl MulAssign<f64> for Vec2 {
	fn mul_assign(&mut self, rhs: f64) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl MulAssign<&f64> for Vec2 {
	fn mul_assign(&mut self, rhs: &f64) {
		self.x *= rhs;
		self.y *= rhs;
	}
}

impl DivAssign<f64> for Vec2 {
	fn div_assign(&mut self, rhs: f64) {
		if rhs == 0. {
			panic!("Division by zero");
		}
		self.x /= rhs;
		self.y /= rhs;
	}
}

impl DivAssign<&f64> for Vec2 {
	fn div_assign(&mut self, rhs: &f64) {
		if *rhs == 0. {
			panic!("Division by zero");
		}
		self.x /= rhs;
		self.y /= rhs;
	}
}

impl Neg for Vec2 {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y
		}
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
	fn test_div_scalar() {
		let v1: Vec2 = Vec2::new(1., 2.);
		assert_eq!(v1 / 2., Vec2::new(0.5, 1.));
	}

	#[test]
	#[should_panic(expected = "Division by zero")]
	fn test_div_scalar_zero() {
		let v1: Vec2 = Vec2::new(1., 2.);
		let _ = v1 / 0.;
	}

	#[test]
	fn test_dot() {
		let v1: Vec2 = Vec2::new(1., 2.);
		let v2: Vec2 = Vec2::new(3., 4.);
		assert_eq!(v1.dot(&v2), 11.);
	}

	#[test]
	fn test_length() {
		let v1: Vec2 = Vec2::new(3., 4.);
		assert_eq!(v1.length(), 5.);
	}

	#[test]
	fn test_normalize() {
		let v1: Vec2 = Vec2::new(3., 4.);
		assert_eq!(v1.normalize().length(), 1.);
	}

	#[test]
	fn test_add_assign() {
		let mut v1: Vec2 = Vec2::new(1., 2.);
		let v2: Vec2 = Vec2::new(3., 4.);
		v1 += v2;
		assert_eq!(v1, Vec2::new(4., 6.));
	}

	#[test]
	fn test_sub_assign() {
		let mut v1: Vec2 = Vec2::new(1., 2.);
		let v2: Vec2 = Vec2::new(3., 4.);
		v1 -= v2;
		assert_eq!(v1, Vec2::new(-2., -2.));
	}

	#[test]
	fn test_mul_assign() {
		let mut v1: Vec2 = Vec2::new(1., 2.);
		let v2: Vec2 = Vec2::new(3., 4.);
		v1 *= v2;
		assert_eq!(v1, Vec2::new(3., 8.));
	}

	#[test]
	fn test_mul_assign_scalar() {
		let mut v1: Vec2 = Vec2::new(1., 2.);
		v1 *= 2.;
		assert_eq!(v1, Vec2::new(2., 4.));
	}

	#[test]
	fn test_div_assign() {
		let mut v1: Vec2 = Vec2::new(1., 2.);
		v1 /= 2.;
		assert_eq!(v1, Vec2::new(0.5, 1.));
	}

	#[test]
	#[should_panic(expected = "Division by zero")]
	fn test_div_assign_zero() {
		let mut v1: Vec2 = Vec2::new(1., 2.);
		v1 /= 0.;
	}

	#[test]
	fn test_neg() {
		let v1: Vec2 = Vec2::new(1., 2.);
		assert_eq!(-v1, Vec2::new(-1., -2.));
	}
}