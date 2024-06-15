use std::{fmt::Display, u8};

use crate::model::maths::vec3::Vec3;

use super::color::Color;

#[derive(Clone, Debug)]
pub enum Texture {
    Value(Vec3),
    Texture(String)
}

impl Texture {
    pub fn to_string(&self) -> String {
        match self {
            Self::Texture(file) => file.to_string(),
            Self::Value(value) => value.to_string()
        }
    }

	pub fn from_float_litteral(string: &String) -> Self {
		if let Ok(value) = string.parse::<f64>() {
			Texture::Value(Vec3::from_value(value))
		} else {
			Texture::Texture(string.clone())
		}
	}
}