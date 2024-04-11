use self::unicolor::Unicolor;

use super::maths::vec3::Vec3;

mod unicolor;

pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    pub fn new(color: &Self) -> Self{
        Self {
            r: color.r,
            g: color.g,
            b: color.b
        }
    }
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub enum Material{
    Unicolor(Unicolor)
}

impl Material {
    pub fn new(color: Color) -> Self {
        Self::Unicolor(Unicolor::new(color))
    }
}

pub trait HasMaterial {
    fn color(&self, x: i32, y: i32) -> Color;
    fn norm(&self, x: i32, y: i32) -> Vec3;
    fn reflection_coef(&self) -> f64;
    fn refraction_coef(&self) -> f64;
}