use crate::model::maths::vec3::Vec3;

use super::{Color, Material};


#[derive(Clone, Debug)]
pub struct Unicolor {
    color: Color,
}

impl Unicolor {
    pub fn new(r: f64, g: f64, b: f64)-> Self {
        Self {
            color: Color::new(r, g, b)
        }
    }
    pub fn from(color: Color) -> Self {
        Self {
            color
        }
    }
}

unsafe impl Send for Unicolor {}

impl Material for Unicolor {
    fn color(&self, _: i32, _: i32) -> Color {
        Color::new(self.color.r(), self.color.g(), self.color.b())
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