use crate::model::maths::vec3::Vec3;
use image::Rgba;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Clone, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
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

	pub fn to_vec3(&self) -> Vec3 {
		Vec3::new(self.r, self.g, self.b)
	}

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba([
            (self.r * 255.) as u8,
            (self.g * 255.) as u8,
            (self.b * 255.) as u8,
            255,
        ])
    }

    pub fn from_vec3(vec: &Vec3) -> Self {
        Color::new(*vec.x(), *vec.y(), *vec.z())
    }

    pub fn from_rgba(color: &Rgba<u8>) -> Self {
        Color::new(
            color.0[0] as f64 / 255.,
            color.0[1] as f64 / 255.,
            color.0[2] as f64 / 255.,
        )
    }

    pub fn clamp(&self, min: f64, max: f64) -> Self {
        Self {
            r: self.r.clamp(min, max),
            g: self.g.clamp(min, max),
            b: self.b.clamp(min, max),
        }
    }

    pub fn apply_gamma(&mut self) {
        self.r = self.r.sqrt();
        self.g = self.g.sqrt();
        self.b = self.b.sqrt();
    }

    pub fn as_weight(&self) -> f64 {
        self.r() + self.b() + self.g()
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self: Self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r: {} g: {} b: {}", self.r, self.g, self.b)
    }
}

impl Add for &Color {
    type Output = Color;
    fn add(self: Self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Add<Color> for &Color {
    type Output = Color;
    fn add(self: Self, rhs: Color) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Add<&Color> for Color {
    type Output = Self;
    fn add(self: Self, rhs: &Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign<&Color> for Color {
    fn add_assign(&mut self, rhs: &Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul for Color {
    type Output = Self;
    fn mul(self: Self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul for &Color {
    type Output = Color;
    fn mul(self: Self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<Color> for &Color {
    type Output = Color;
    fn mul(self: Self, rhs: Color) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<&Color> for Color {
    type Output = Self;
    fn mul(self: Self, rhs: &Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self: Self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self: Self, rhs: Color) -> Self::Output {
        Self::Output {
            r: rhs.r * self,
            g: rhs.g * self,
            b: rhs.b * self,
        }
    }
}

impl Mul<f64> for &Color {
    type Output = Color;
    fn mul(self: Self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<&Color> for f64 {
    type Output = Color;
    fn mul(self: Self, rhs: &Color) -> Self::Output {
        Self::Output {
            r: rhs.r * self,
            g: rhs.g * self,
            b: rhs.b * self,
        }
    }
}

impl Div for Color {
    type Output = Self;
    fn div(self: Self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl Div for &Color {
    type Output = Color;
    fn div(self: Self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl Div<Color> for &Color {
    type Output = Color;
    fn div(self: Self, rhs: Color) -> Self::Output {
        Self::Output {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl Div<&Color> for Color {
    type Output = Self;
    fn div(self: Self, rhs: &Self) -> Self::Output {
        Self::Output {
            r: self.r / rhs.r,
            g: self.g / rhs.g,
            b: self.b / rhs.b,
        }
    }
}

impl Div<f64> for Color {
    type Output = Self;
    fn div(self: Self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl Div<Color> for f64 {
    type Output = Color;
    fn div(self: Self, rhs: Color) -> Self::Output {
        Self::Output {
            r: rhs.r / self,
            g: rhs.g / self,
            b: rhs.b / self,
        }
    }
}

impl Div<f64> for &Color {
    type Output = Color;
    fn div(self: Self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl Div<&Color> for f64 {
    type Output = Color;
    fn div(self: Self, rhs: &Color) -> Self::Output {
        Self::Output {
            r: rhs.r / self,
            g: rhs.g / self,
            b: rhs.b / self,
        }
    }
}
