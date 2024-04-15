use image::{Rgba, RgbaImage};

use super::maths::vec3::Vec3;
use crate::model::materials::unicolor::Unicolor;
use std::fmt::Debug;
pub mod unicolor;

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
    pub fn toRgba(self) -> Rgba<u8> {
        Rgba([self.r, self.g, self.b, 255])
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
