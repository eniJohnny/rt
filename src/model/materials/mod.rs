use image::Rgba;

use super::maths::vec3::Vec3;
use crate::model::materials::unicolor::Unicolor;
use std::fmt::Debug;
pub mod unicolor;
use std::ops::{ Add, Mul };

#[derive(Clone, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

impl Color {
    // Getters
    pub fn r(&self) -> f64 {
        self.r
    }
    pub fn g(&self) -> f64 {
        self.g
    }
    pub fn b(&self) -> f64 {
        self.b
    }

    // Constructors
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
	
    pub fn toRgba(self) -> Rgba<u8> {
        Rgba([(self.r * 255.) as u8, (self.g * 255.) as u8, (self.b * 255.) as u8, 255])
    }
}

impl Add for Color {
	type Output = Self;
	fn add(self: Self, rhs: Self) -> Self::Output {
		Self::Output {
			r: (self.r + rhs.r).clamp(0., 1.),
			g: (self.g + rhs.g).clamp(0., 1.),
			b: (self.b + rhs.b).clamp(0., 1.)
		}
	}
}

impl Add for &Color {
	type Output = Color;
	fn add(self: Self, rhs: Self) -> Self::Output {
		Self::Output {
			r: (self.r + rhs.r).clamp(0., 1.),
			g: (self.g + rhs.g).clamp(0., 1.),
			b: (self.b + rhs.b).clamp(0., 1.)
		}
	}
}

impl Add<Color> for &Color {
	type Output = Color;
	fn add(self: Self, rhs: Color) -> Self::Output {
		Self::Output {
			r: (self.r + rhs.r).clamp(0., 1.),
			g: (self.g + rhs.g).clamp(0., 1.),
			b: (self.b + rhs.b).clamp(0., 1.)
		}
	}
}

impl Add<&Color> for Color {
	type Output = Self;
	fn add(self: Self, rhs: &Self) -> Self::Output {
		Self::Output {
			r: (self.r + rhs.r).clamp(0., 1.),
			g: (self.g + rhs.g).clamp(0., 1.),
			b: (self.b + rhs.b).clamp(0., 1.)
		}
	}
}

impl Mul for Color {
	type Output = Self;
	fn mul(self: Self, rhs: Self) -> Self::Output {
		Self::Output {
			r: (self.r * rhs.r).clamp(0., 1.),
			g: (self.g * rhs.g).clamp(0., 1.),
			b: (self.b * rhs.b).clamp(0., 1.)
		}
	}
}

impl Mul for &Color {
	type Output = Color;
	fn mul(self: Self, rhs: Self) -> Self::Output {
		Self::Output {
			r: (self.r * rhs.r).clamp(0., 1.),
			g: (self.g * rhs.g).clamp(0., 1.),
			b: (self.b * rhs.b).clamp(0., 1.)
		}
	}
}

impl Mul<Color> for &Color {
	type Output = Color;
	fn mul(self: Self, rhs: Color) -> Self::Output {
		Self::Output {
			r: (self.r * rhs.r).clamp(0., 1.),
			g: (self.g * rhs.g).clamp(0., 1.),
			b: (self.b * rhs.b).clamp(0., 1.)
		}
	}
}

impl Mul<&Color> for Color {
	type Output = Self;
	fn mul(self: Self, rhs: &Self) -> Self::Output {
		Self::Output {
			r: (self.r * rhs.r).clamp(0., 1.),
			g: (self.g * rhs.g).clamp(0., 1.),
			b: (self.b * rhs.b).clamp(0., 1.)
		}
	}
}

impl Mul<f64> for Color {
	type Output = Self;
	fn mul(self: Self, rhs: f64) -> Self::Output {
		Self::Output {
			r: (self.r * rhs).clamp(0., 1.),
			g: (self.g * rhs).clamp(0., 1.),
			b: (self.b * rhs).clamp(0., 1.)
		}
	}
}

impl Mul<Color> for f64 {
	type Output = Color;
	fn mul(self: Self, rhs: Color) -> Self::Output {
		Self::Output {
			r: (rhs.r * self).clamp(0., 1.),
			g: (rhs.g * self).clamp(0., 1.),
			b: (rhs.b * self).clamp(0., 1.)
		}
	}
}

impl Mul<f64> for &Color {
	type Output = Color;
	fn mul(self: Self, rhs: f64) -> Self::Output {
		Self::Output {
			r: (self.r * rhs).clamp(0., 1.),
			g: (self.g * rhs).clamp(0., 1.),
			b: (self.b * rhs).clamp(0., 1.)
		}
	}
}

impl Mul<&Color> for f64 {
	type Output = Color;
	fn mul(self: Self, rhs: &Color) -> Self::Output {
		Self::Output {
			r: (rhs.r * self).clamp(0., 1.),
			g: (rhs.g * self).clamp(0., 1.),
			b: (rhs.b * self).clamp(0., 1.)
		}
	}
}

pub trait Material: Debug {
    fn color(&self, x: i32, y: i32) -> Color;
    fn norm(&self, x: i32, y: i32) -> Vec3;
    fn reflection_coef(&self) -> f64;
    fn refraction_coef(&self) -> f64;
    fn needs_projection(&self) -> bool;
}

impl dyn Material {
    pub fn new(color: Color) -> Box<Self> {
        Box::new(Unicolor::new(color.r(), color.g(), color.b()))
    }
    pub fn default() -> Box<Self> {
        Box::new(Unicolor::new(0., 0., 0.))
    }
}
