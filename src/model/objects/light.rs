use crate::model::{materials::Color, maths::vec3::Vec3};

pub struct Light {
    pos: Vec3,
    intensity: f64,
    color: Color
}

impl Light {
    // Accessors
    pub fn get_pos(&self) -> &Vec3 { &self.pos }
    pub fn get_intensity(&self) -> f64 { self.intensity }
    pub fn get_color(&self) -> &Color { &self.color }

    // Constructor
    pub fn new(pos: Vec3, intensity: f64, color: Color) -> Self {
        self::Light { pos, intensity, color }
    }
}

pub struct AmbientLight {
    intensity: f64,
    color: Color
}

impl AmbientLight {
    // Accessors
    pub fn get_intensity(&self) -> f64 { self.intensity }
    pub fn get_color(&self) -> &Color { &self.color }

    // Constructor
    pub fn new(intensity: f64, color: Color) -> Self {
        self::AmbientLight { intensity, color }
    }
    pub fn default() -> Self {
        Self {
            intensity: 0.5,
            color: Color::new(255, 255, 255)
        }
    }
}
