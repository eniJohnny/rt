use std::{fmt::Display, u8};

use image::RgbaImage;

use crate::{model::maths::vec3::Vec3, ui::utils::misc::Value};

use super::{color::Color, material::Projection};

#[derive(Clone, Debug)]
pub enum TextureType {
    Float,
    Vector,
    Color,
    Boolean,
}

impl TextureType {}

#[derive(Clone, Debug)]
pub enum Texture {
    Value(Vec3, TextureType),
    Texture(String, TextureType),
}

impl Texture {
    pub fn to_string(&self) -> String {
        match self {
            Self::Texture(file, _) => file.to_string(),
            Self::Value(vector, t) => match t {
                TextureType::Float => vector.to_value().to_string(),
                TextureType::Boolean => (vector.to_value() > 0.5).to_string(),
                TextureType::Color => Color::from_vec3(vector).to_string(),
                TextureType::Vector => vector.to_string(),
            },
        }
    }

    pub fn from_vector(file: &String, default_vec: Vec3) -> Self {
        if file == "" {
            Texture::Value(default_vec, TextureType::Vector)
        } else {
            Texture::Texture(file.clone(), TextureType::Vector)
        }
    }

    pub fn from_value(value: &Value) -> Self {
        match value {
            Value::Bool(value) => {
                let value = match value {
                    true => 1.,
                    false => 0.
                };
                Texture::from_float_litteral(&"".to_string(), value)
            }
            Value::Float(value) => {
                Texture::from_float_litteral(&"".to_string(), *value)
            }
            _ => panic!("Value is not convertible to texture")
        }
    }

    pub fn from_float_scaled(string: &String, default: f64, scale: f64) -> Self {
        if let Ok(value) = string.parse::<f64>() {
            Texture::Value(Vec3::from_value(value) / scale, TextureType::Float)
        } else if string == "" {
            Texture::Value(Vec3::from_value(default) / scale, TextureType::Float)
        } else {
            Texture::Texture(string.clone(), TextureType::Float)
        }
    }

    pub fn from_float_litteral(string: &String, default: f64) -> Self {
        if let Ok(value) = string.parse::<f64>() {
            Texture::Value(Vec3::from_value(value), TextureType::Float)
        } else if string == "" {
            Texture::Value(Vec3::from_value(default), TextureType::Float)
        } else {
            Texture::Texture(string.clone(), TextureType::Float)
        }
    }

    pub fn get(proj: &Projection, img: &RgbaImage) -> Color {
        let x = (((&proj.u - ((proj.u as u32) as f64)) * img.width() as f64) as u32)
            .max(0)
            .min(img.width() - 1);
        let y = (((1. - (&proj.v - ((proj.v as u32) as f64))) * img.height() as f64) as u32)
            .max(0)
            .min(img.height() - 1);

        Color::from_rgba(img.get_pixel(x, y))
    }
}