use std::ops::{Add, Mul, Sub, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};
use super::vec3::Vec3;

#[derive(PartialEq, Debug)]
pub struct Quaternion {
	x: f64,
	y: f64,
	z: f64,
	w: f64
}

impl Quaternion {
	pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
		Self { x, y, z, w }
	}

	pub fn from_euler_angles(roll: f64, pitch: f64, yaw: f64) -> Self {
		let cr = (roll * 0.5).cos();
    	let sr = (roll * 0.5).sin();
    	let cp = (pitch * 0.5).cos();
    	let sp = (pitch * 0.5).sin();
    	let cy = (yaw * 0.5).cos();
    	let  sy = (yaw * 0.5).sin();
		Self {
			w: cr * cp * cy + sr * sp * sy,
			x: sr * cp * cy - cr * sp * sy,
			y: cr * sp * cy + sr * cp * sy,
			z: cr * cp * sy - sr * sp * cy
		}
	}

	pub fn new_from_axis_angle(axis: &Vec3, angle: f64) -> Self {
		let half_angle: f64 = angle / 2.;
		let sin: f64 = half_angle.sin();
		let cos: f64 = half_angle.cos();

		(sin * axis.clone().normalize()).to_quaternion(cos)
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

	pub fn w(&self) -> &f64 {
		&self.w
	}

	pub fn length(&self) -> f64 {
		(self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
	}

	pub fn normalize(self) -> Self {
		let norm: f64 = self.length();
		Self {
			x: self.x / norm,
			y: self.y / norm,
			z: self.z / norm,
			w: self.w / norm
		}
	}

	pub fn conjugate(&self) -> Self {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
			w: self.w
		}
	}

	pub fn rotate(&self, v: &Vec3) -> Vec3 {
		let qv = Quaternion::new(*v.x(), *v.y(), *v.z(), 0.);
		let result = self * qv * self.conjugate();
		result.to_vec3()
	}

	pub fn to_vec3(&self) -> Vec3 {
		Vec3::new(self.x, self.y, self.z)
	}
}

impl Add for Quaternion {
	type Output = Self;
	fn add(self, other: Self) -> Self::Output {
		Self {
			x: self.x + &other.x,
			y: self.y + &other.y,
			z: self.z + &other.z,
			w: self.w + &other.w
		}
	}
}

impl Add<&Self> for Quaternion {
	type Output = Self;
	fn add(self, other: &Self) -> Self::Output {
		Self {
			x: self.x + &other.x,
			y: self.y + &other.y,
			z: self.z + &other.z,
			w: self.w + &other.w
		}
	}
}

impl Add<Quaternion> for &Quaternion {
	type Output = Quaternion;
	fn add(self, other: Quaternion) -> Self::Output {
		Quaternion {
			x: self.x + &other.x,
			y: self.y + &other.y,
			z: self.z + &other.z,
			w: self.w + &other.w
		}
	}
}

impl Add for &Quaternion {
	type Output = Quaternion;
	fn add(self, other: Self) -> Self::Output {
		Quaternion {
			x: self.x + &other.x,
			y: self.y + &other.y,
			z: self.z + &other.z,
			w: self.w + &other.w
		}
	}
}

impl Sub for Quaternion {
	type Output = Self;
	fn sub(self, other: Self) -> Self::Output {
		Self {
			x: self.x - &other.x,
			y: self.y - &other.y,
			z: self.z - &other.z,
			w: self.w - &other.w
		}
	}
}

impl Sub<&Self> for Quaternion {
	type Output = Self;
	fn sub(self, other: &Self) -> Self::Output {
		Self {
			x: self.x - &other.x,
			y: self.y - &other.y,
			z: self.z - &other.z,
			w: self.w - &other.w
		}
	}
}

impl Sub<Quaternion> for &Quaternion {
	type Output = Quaternion;
	fn sub(self, other: Quaternion) -> Self::Output {
		Quaternion {
			x: self.x - &other.x,
			y: self.y - &other.y,
			z: self.z - &other.z,
			w: self.w - &other.w
		}
	}
}

impl Sub for &Quaternion {
	type Output = Quaternion;
	fn sub(self, other: Self) -> Self::Output {
		Quaternion {
			x: self.x - &other.x,
			y: self.y - &other.y,
			z: self.z - &other.z,
			w: self.w - &other.w
		}
	}
}

impl Mul<Self> for Quaternion {
	type Output = Self;
	fn mul(self, other: Self) -> Self::Output {
		Self {
			x: self.w * &other.x + self.x * &other.w + self.y * &other.z - self.z * &other.y,
			y: self.w * &other.y - self.x * &other.z + self.y * &other.w + self.z * &other.x,
			z: self.w * &other.z + self.x * &other.y - self.y * &other.x + self.z * &other.w,
			w: self.w * &other.w - self.x * &other.x - self.y * &other.y - self.z * &other.z
		}
	}
}

impl Mul<&Self> for Quaternion {
	type Output = Self;
	fn mul(self, other: &Self) -> Self::Output {
		Self {
			x: self.w * &other.x + self.x * &other.w + self.y * &other.z - self.z * &other.y,
			y: self.w * &other.y - self.x * &other.z + self.y * &other.w + self.z * &other.x,
			z: self.w * &other.z + self.x * &other.y - self.y * &other.x + self.z * &other.w,
			w: self.w * &other.w - self.x * &other.x - self.y * &other.y - self.z * &other.z
		}
	}
}

impl Mul<Quaternion> for &Quaternion {
	type Output = Quaternion;
	fn mul(self, other: Quaternion) -> Self::Output {
		Quaternion {
			x: self.w * &other.x + self.x * &other.w + self.y * &other.z - self.z * &other.y,
			y: self.w * &other.y - self.x * &other.z + self.y * &other.w + self.z * &other.x,
			z: self.w * &other.z + self.x * &other.y - self.y * &other.x + self.z * &other.w,
			w: self.w * &other.w - self.x * &other.x - self.y * &other.y - self.z * &other.z
		}
	}
}

impl Mul<Self> for &Quaternion {
	type Output = Quaternion;
	fn mul(self, other: Self) -> Self::Output {
		Quaternion {
			x: self.w * &other.x + self.x * &other.w + self.y * &other.z - self.z * &other.y,
			y: self.w * &other.y - self.x * &other.z + self.y * &other.w + self.z * &other.x,
			z: self.w * &other.z + self.x * &other.y - self.y * &other.x + self.z * &other.w,
			w: self.w * &other.w - self.x * &other.x - self.y * &other.y - self.z * &other.z
		}
	}
}

impl Mul<f64> for Quaternion {
	type Output = Self;
	fn mul(self, scalar: f64) -> Self::Output {
		Self {
			x: self.x * scalar,
			y: self.y * scalar,
			z: self.z * scalar,
			w: self.w * scalar
		}
	}
}

impl Mul<&f64> for Quaternion {
	type Output = Self;
	fn mul(self, scalar: &f64) -> Self::Output {
		Self {
			x: self.x * scalar,
			y: self.y * scalar,
			z: self.z * scalar,
			w: self.w * scalar
		}
	}
}

impl Mul<Quaternion> for f64 {
	type Output = Quaternion;
	fn mul(self, q: Self::Output) -> Self::Output {
		Self::Output {
			x: q.x * self,
			y: q.y * self,
			z: q.z * self,
			w: q.w * self
		}
	}
}

impl Mul<&Quaternion> for f64 {
	type Output = Quaternion;
	fn mul(self, q: &Self::Output) -> Self::Output {
		Self::Output {
			x: q.x * self,
			y: q.y * self,
			z: q.z * self,
			w: q.w * self
		}
	}
}

impl Mul<f64> for &Quaternion {
	type Output = Quaternion;
	fn mul(self, scalar: f64) -> Self::Output {
		Quaternion {
			x: self.x * scalar,
			y: self.y * scalar,
			z: self.z * scalar,
			w: self.w * scalar
		}
	}
}

impl Mul<&f64> for &Quaternion {
	type Output = Quaternion;
	fn mul(self, scalar: &f64) -> Self::Output {
		Quaternion {
			x: self.x * scalar,
			y: self.y * scalar,
			z: self.z * scalar,
			w: self.w * scalar
		}
	}
}

impl Div<f64> for Quaternion {
	type Output = Self;
	fn div(self, scalar: f64) -> Self::Output {
		if scalar == 0. {
			panic!("Division by zero");
		}
		Self {
			x: self.x / scalar,
			y: self.y / scalar,
			z: self.z / scalar,
			w: self.w / scalar
		}
	}
}

impl Div<f64> for &Quaternion {
	type Output = Quaternion;
	fn div(self, scalar: f64) -> Self::Output {
		if scalar == 0. {
			panic!("Division by zero");
		}
		Quaternion {
			x: self.x / scalar,
			y: self.y / scalar,
			z: self.z / scalar,
			w: self.w / scalar
		}
	}
}

impl Div<&f64> for Quaternion {
	type Output = Self;
	fn div(self, scalar: &f64) -> Self::Output {
		if *scalar == 0. {
			panic!("Division by zero");
		}
		Self {
			x: self.x / scalar,
			y: self.y / scalar,
			z: self.z / scalar,
			w: self.w / scalar
		}
	}
}

impl Div<&f64> for &Quaternion {
	type Output = Quaternion;
	fn div(self, scalar: &f64) -> Self::Output {
		if *scalar == 0. {
			panic!("Division by zero");
		}
		Quaternion {
			x: self.x / scalar,
			y: self.y / scalar,
			z: self.z / scalar,
			w: self.w / scalar
		}
	}
}

impl AddAssign for Quaternion {
	fn add_assign(&mut self, other: Self) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
		self.w += other.w;
	}
}

impl AddAssign<&Self> for Quaternion {
	fn add_assign(&mut self, other: &Self) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
		self.w += other.w;
	}
}

impl SubAssign for Quaternion {
	fn sub_assign(&mut self, other: Self) {
		self.x -= other.x;
		self.y -= other.y;
		self.z -= other.z;
		self.w -= other.w;
	}
}

impl SubAssign<&Self> for Quaternion {
	fn sub_assign(&mut self, other: &Self) {
		self.x -= other.x;
		self.y -= other.y;
		self.z -= other.z;
		self.w -= other.w;
	}
}

impl MulAssign<Self> for Quaternion {
	fn mul_assign(&mut self, other: Self) {
		*self = Self {
			x: self.w * &other.x + self.x * &other.w + self.y * &other.z - self.z * &other.y,
			y: self.w * &other.y - self.x * &other.z + self.y * &other.w + self.z * &other.x,
			z: self.w * &other.z + self.x * &other.y - self.y * &other.x + self.z * &other.w,
			w: self.w * &other.w - self.x * &other.x - self.y * &other.y - self.z * &other.z
		}
	}
	
}

impl MulAssign<&Self> for Quaternion {
	fn mul_assign(&mut self, other: &Self) {
		*self = Self {
			x: self.w * &other.x + self.x * &other.w + self.y * &other.z - self.z * &other.y,
			y: self.w * &other.y - self.x * &other.z + self.y * &other.w + self.z * &other.x,
			z: self.w * &other.z + self.x * &other.y - self.y * &other.x + self.z * &other.w,
			w: self.w * &other.w - self.x * &other.x - self.y * &other.y - self.z * &other.z
		}
	}
	
}

impl MulAssign<f64> for Quaternion {
	fn mul_assign(&mut self, scalar: f64) {
		*self = Self {
			x: self.x * scalar,
			y: self.y * scalar,
			z: self.z * scalar,
			w: self.w * scalar
		}
	}
}

impl MulAssign<&f64> for Quaternion {
	fn mul_assign(&mut self, scalar: &f64) {
		*self = Self {
			x: self.x * scalar,
			y: self.y * scalar,
			z: self.z * scalar,
			w: self.w * scalar
		}
	}
}

impl DivAssign<f64> for Quaternion {
	fn div_assign(&mut self, scalar: f64) {
		if scalar == 0. {
			panic!("Division by zero");
		}
		self.x /= scalar;
		self.y /= scalar;
		self.z /= scalar;
		self.w /= scalar;
	}
}

impl DivAssign<&f64> for Quaternion {
	fn div_assign(&mut self, scalar: &f64) {
		if *scalar == 0. {
			panic!("Division by zero");
		}
		self.x /= scalar;
		self.y /= scalar;
		self.z /= scalar;
		self.w /= scalar;
	}
}

impl Display for Quaternion {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{}i + {}j + {}k + {}", self.x, self.y, self.z, self.w)
	}
}

impl Neg for Quaternion {
	type Output = Self;
	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
			z: -self.z,
			w: -self.w
		}
	}
}

#[cfg(test)]
mod tests {

use super::Quaternion;

	#[test]
	fn test_add() {
		let q1 = Quaternion::new(1., 2., 3., 4.);
		let q2 = Quaternion::new(5., 6., 7., 8.);
		let result = q1 + q2;
		assert_eq!(result, Quaternion::new(6., 8., 10., 12.));
	}

	#[test]
	fn test_sub() {
		let q1 = Quaternion::new(1., 2., 3., 4.);
		let q2 = Quaternion::new(5., 6., 7., 8.);
		let result = q1 - q2;
		assert_eq!(result, Quaternion::new(-4., -4., -4., -4.));
	}

	#[test]
	fn test_mul() {
		let q1 = Quaternion::new(1., 2., 3., 4.);
		let q2 = Quaternion::new(5., 6., 7., 8.);
		let result = q1 * q2;
		assert_eq!(result, Quaternion::new(24., 48., 48., -6.));
	}

	#[test]
	fn test_mul_scalar() {
		let q = Quaternion::new(1., 2., 3., 4.);
		let result = q * 2.;
		assert_eq!(result, Quaternion::new(2., 4., 6., 8.));
	}

	#[test]
	fn test_div_scalar() {
		let q = Quaternion::new(1., 2., 3., 4.);
		let result = q / 2.;
		assert_eq!(result, Quaternion::new(0.5, 1., 1.5, 2.));
	}

	#[test]
	#[should_panic(expected = "Division by zero")]
	fn test_div_zero() {
		let q = Quaternion::new(1., 2., 3., 4.);
		let _ = q / 0.;
	}

	#[test]
	fn test_add_assign() {
		let mut q1 = Quaternion::new(1., 2., 3., 4.);
		let q2 = Quaternion::new(5., 6., 7., 8.);
		q1 += q2;
		assert_eq!(q1, Quaternion::new(6., 8., 10., 12.));
	}

	#[test]
	fn test_sub_assign() {
		let mut q1 = Quaternion::new(1., 2., 3., 4.);
		let q2 = Quaternion::new(5., 6., 7., 8.);
		q1 -= q2;
		assert_eq!(q1, Quaternion::new(-4., -4., -4., -4.));
	}

	#[test]
	fn test_mul_assign() {
		let mut q1 = Quaternion::new(1., 2., 3., 4.);
		let q2 = Quaternion::new(5., 6., 7., 8.);
		q1 *= q2;
		assert_eq!(q1, Quaternion::new(24., 48., 48., -6.));
	}

	#[test]
	fn test_mul_assign_scalar() {
		let mut q = Quaternion::new(1., 2., 3., 4.);
		q *= 2.;
		assert_eq!(q, Quaternion::new(2., 4., 6., 8.));
	}

	#[test]
	fn test_div_assign_scalar() {
		let mut q = Quaternion::new(1., 2., 3., 4.);
		q /= 2.;
		assert_eq!(q, Quaternion::new(0.5, 1., 1.5, 2.));
	}

	#[test]
	#[should_panic(expected = "Division by zero")]
	fn test_div_assign_zero() {
		let mut q = Quaternion::new(1., 2., 3., 4.);
		q /= 0.;
	}

	#[test]
	fn test_neg() {
		let q = Quaternion::new(1., 2., 3., 4.);
		let result = -q;
		assert_eq!(result, Quaternion::new(-1., -2., -3., -4.));
	}

	#[test]
	fn test_length() {
		let q = Quaternion::new(1., 2., 3., 4.);
		assert_eq!(q.length(), 30_f64.sqrt());
	}

	#[test]
	fn test_normalize() {
		let q = Quaternion::new(1., 2., 3., 4.);
		let result = q.normalize();
		assert!(result.length() > 1. - f64::EPSILON);
		assert!(result.length() < 1. + f64::EPSILON);
	}

	#[test]
	fn test_conjugate() {
		let q = Quaternion::new(1., 2., 3., 4.);
		let result = q.conjugate();
		assert_eq!(result, Quaternion::new(-1., -2., -3., 4.));
	}

	#[test]
	fn test_display() {
		let q = Quaternion::new(1., 2., 3., 4.);
		assert_eq!(format!("{}", q), "1i + 2j + 3k + 4");
	}
	
	#[test]
	fn test_new_from_axis_angle() {
		let axis = super::Vec3::new(1., 0., 0.);
		let angle = std::f64::consts::PI / 3.;
		let q = Quaternion::new_from_axis_angle(&axis, angle);
		println!("{:?}", q);
		assert!((q.x() - 0.5).abs() < std::f64::EPSILON);
		assert_eq!(*q.y(), 0.);
		assert_eq!(*q.z(), 0.);
		assert!((q.w() - 3_f64.sqrt() / 2.).abs() - 0.5 < f64::EPSILON);
	}
}