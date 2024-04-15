use crate::model::{materials::Color, maths::vec3::Vec3};

#[derive(Debug)]
pub struct Light {
    pos: Vec3,
    intensity: f64,
    color: Color
}

impl Light {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn intensity(&self) -> f64 { self.intensity }
    pub fn color(&self) -> &Color { &self.color }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos }
    pub fn set_intensity(&mut self, intensity: f64) { self.intensity = intensity }
    pub fn set_color(&mut self, color: Color) { self.color = color }

    // Constructor
    pub fn new(pos: Vec3, intensity: f64, color: Color) -> Self {
        self::Light { pos, intensity, color }
    }
}

#[derive(Debug)]
pub struct AmbientLight {
    intensity: f64,
    color: Color
}

impl AmbientLight {
    // Accessors
    pub fn intensity(&self) -> f64 { self.intensity }
    pub fn color(&self) -> &Color { &self.color }

    // Constructor
    pub fn new(intensity: f64, color: Color) -> Self {
        self::AmbientLight { intensity, color }
    }
    pub fn default() -> Self {
        Self {
            intensity: 0.5,
            color: Color::new(1., 1., 1.)
        }
    }
}
