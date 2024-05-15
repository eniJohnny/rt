use crate::model::maths::vec3::Vec3;

use super::{Color, Material};

#[derive(Clone, Debug)]
pub struct Metal {
    color: Color,
    metalness: f64,
    roughness: f64,
}

impl Metal {
    pub fn new(color: Color, metalness: f64, roughness: f64) -> Self {
        Self { color, metalness, roughness }
    }
}

unsafe impl Send for Metal {}

impl Material for Metal {
    fn color(&self, _: i32, _: i32) -> Color {
        Color::new(self.color.r(), self.color.g(), self.color.b())
    }
    fn norm(&self, _: i32, _: i32) -> Vec3 {
        Vec3::new(0., 0., 1.)
    }
    fn reflection_coef(&self) -> f64 {
        self.metalness
    }
    fn refraction_coef(&self) -> f64 {
        0.
    }
    fn roughness(&self) -> f64 {
        self.roughness
    }
    fn needs_projection(&self) -> bool {
        false
    }
}
