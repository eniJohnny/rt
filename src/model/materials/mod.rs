use super::maths::vec3::Vec3;
use crate::model::materials::unicolor::Unicolor;
use std::fmt::Debug;
pub mod unicolor;
use std::ops::{ Add, Mul };
use std::cmp::min;

#[derive(Clone, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    // Getters
    pub fn r(&self) -> u8 {
        self.r
    }
    pub fn g(&self) -> u8 {
        self.g
    }
    pub fn b(&self) -> u8 {
        self.b
    }

    // Constructors
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Add for Color {
	type Output = Self;
	fn add(self: Self, rhs: Self) -> Self::Output {
		Self::Output {
			r: min(255 as u16, self.r as u16 + rhs.r as u16) as u8,
			g: min(255 as u16, self.g as u16 + rhs.g as u16) as u8,
			b: min(255 as u16, self.b as u16 + rhs.b as u16) as u8
		}
	}
}

impl Add for &Color {
	type Output = Color;
	fn add(self: Self, rhs: Self) -> Self::Output {
		Self::Output {
			r: min(255 as u16, self.r as u16 + rhs.r as u16) as u8,
			g: min(255 as u16, self.g as u16 + rhs.g as u16) as u8,
			b: min(255 as u16, self.b as u16 + rhs.b as u16) as u8
		}
	}
}

impl Add<Color> for &Color {
	type Output = Color;
	fn add(self: Self, rhs: Color) -> Self::Output {
		Self::Output {
			r: min(255 as u16, self.r as u16 + rhs.r as u16) as u8,
			g: min(255 as u16, self.g as u16 + rhs.g as u16) as u8,
			b: min(255 as u16, self.b as u16 + rhs.b as u16) as u8
		}
	}
}

impl Add<&Color> for Color {
	type Output = Self;
	fn add(self: Self, rhs: &Self) -> Self::Output {
		Self::Output {
			r: min(255, self.r as u16 + rhs.r as u16) as u8,
			g: min(255, self.g as u16 + rhs.g as u16) as u8,
			b: min(255, self.b as u16 + rhs.b as u16) as u8
		}
	}
}

impl Mul<f64> for Color {
	type Output = Self;
	fn mul(self: Self, rhs: f64) -> Self::Output {
		Self::Output {
			r: min(255, (self.r as f64 * rhs) as u16) as u8,
			g: min(255, (self.g as f64 * rhs) as u16) as u8,
			b: min(255, (self.b as f64 * rhs) as u16) as u8
		}
	}
}

impl Mul<Color> for f64 {
	type Output = Color;
	fn mul(self: Self, rhs: Color) -> Self::Output {
		Self::Output {
			r: min(255, (self * rhs.r as f64) as u16) as u8,
			g: min(255, (self * rhs.g as f64) as u16) as u8,
			b: min(255, (self * rhs.b as f64) as u16) as u8
		}
	}
}

impl Mul<f64> for &Color {
	type Output = Color;
	fn mul(self: Self, rhs: f64) -> Self::Output {
		Self::Output {
			r: min(255, (self.r as f64 * rhs) as u16) as u8,
			g: min(255, (self.g as f64 * rhs) as u16) as u8,
			b: min(255, (self.b as f64 * rhs) as u16) as u8
		}
	}
}

impl Mul<&Color> for f64 {
	type Output = Color;
	fn mul(self: Self, rhs: &Color) -> Self::Output {
		Self::Output {
			r: min(255, (self * rhs.r as f64) as u16) as u8,
			g: min(255, (self * rhs.g as f64) as u16) as u8,
			b: min(255, (self * rhs.b as f64) as u16) as u8
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
        Box::new(Unicolor::new(0, 0, 0))
    }
}
