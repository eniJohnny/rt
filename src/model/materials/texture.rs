use std::fmt::Display;

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
}