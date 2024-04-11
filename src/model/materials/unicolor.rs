use crate::model::maths::vec3::Vec3;

use super::{Color, Material};


pub struct Unicolor {
    color: Color,
}

impl Unicolor {
    pub fn new(color: &Color)-> Self {
        Self {
            color: Color::new(color)
        }
    }
}

impl Material for Unicolor {
    fn color(&self, _: i32, _: i32) -> Color {
        Color::new(&self.color)
    }
    fn norm(&self, _: i32, _: i32) -> Vec3 {
        Vec3::new(0., 0., 1.)
    }
    fn reflection_coef(&self) -> f64 {
        0.
    }
    fn refraction_coef(&self) -> f64 {
        0.
    }
    fn needs_projection(&self) -> bool {
        false
    }
}