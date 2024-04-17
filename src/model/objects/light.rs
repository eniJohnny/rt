use crate::model::{materials::Color, maths::{ hit::Hit, ray::Ray, vec3::Vec3}};

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

	pub fn get_diffuse(&self, hit: &Hit) -> Color {
		let to_light = (self.pos() - hit.pos()).normalize();
		let mut ratio = to_light.dot(hit.norm());
		if ratio < 0. {
			return Color::new(0., 0., 0.);
		}
		ratio *= 0_f64.max(1. - (self.pos() - hit.pos()).length().powf(2.) / (self.intensity().powf(2.)));
		ratio * self.color()
	}

	pub fn get_specular(&self, hit: &Hit, ray: &Ray) -> Color {
		let to_light = (self.pos() - hit.pos()).normalize();
		let reflected = (-(&to_light) - hit.norm().dot(&-to_light) * 2. * hit.norm()).normalize();
		let mut ratio = (-ray.get_dir()).normalize().dot(&reflected);
		if ratio < 0. {
			return Color::new(0., 0., 0.);
		}
		ratio = ratio.powf(25.);
		ratio *= 0_f64.max(1. - (self.pos() - hit.pos()).length().powf(2.) / (self.intensity().powf(2.)));
		ratio * self.color()
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
