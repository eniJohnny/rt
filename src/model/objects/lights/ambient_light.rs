use crate::model::materials::color::Color;

#[derive(Debug)]
pub struct AmbientLight {
    intensity: f64,
    color: Color,
}

impl AmbientLight {
    // Accessors
    pub fn intensity(&self) -> f64 {
        self.intensity
    }
    pub fn color(&self) -> &Color {
        &self.color
    }

    // Constructor
    pub fn new(intensity: f64, color: Color) -> Self {
        self::AmbientLight { intensity, color }
    }
    pub fn default() -> Self {
        Self {
            intensity: 0.,
            color: Color::new(1., 1., 1.),
        }
    }
}