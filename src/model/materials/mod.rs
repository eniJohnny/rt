use super::maths::vec3::Vec3;

mod unicolor;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn new(color: &Self) -> Self{
        Self {
            r: color.r,
            g: color.g,
            b: color.b
        }
    }
}

pub trait Material {
    fn color(&self, x: i32, y: i32) -> Color;
    fn norm(&self, x: i32, y: i32) -> Vec3;
    fn reflection_coef(&self) -> f64;
    fn refraction_coef(&self) -> f64;
    fn needs_projection(&self) -> bool;
}