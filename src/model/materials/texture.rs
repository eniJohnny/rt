use std::{fmt::Display, u8};

use image::RgbaImage;

use crate::model::maths::vec3::Vec3;

use super::{color::Color, material::Projection};

#[derive(Clone, Debug)]
pub enum Texture {
    Value(Vec3),
    Texture(String),
}

impl Texture {
    pub fn to_string(&self) -> String {
        match self {
            Self::Texture(file) => file.to_string(),
            Self::Value(value) => value.to_string(),
        }
    }

    pub fn from_file_or(file: &String, default_vec: Vec3) -> Self {
        if file == "" {
            Texture::Value(default_vec)
        } else {
            Texture::Texture(file.clone())
        }
    }

    pub fn from_float_litteral(string: &String) -> Self {
        if let Ok(value) = string.parse::<f64>() {
            Texture::Value(Vec3::from_value(value))
        } else if string == "" {
            Texture::Value(Vec3::from_value(0.))
        } else {
            Texture::Texture(string.clone())
        }
    }

    pub fn get(proj: &Projection, img: &RgbaImage) -> Color {
        let x = ((proj.u * img.width() as f64) as u32)
            .max(0)
            .min(img.width() - 1);
        let y = (((1. - &proj.v) * img.height() as f64) as u32)
            .max(0)
            .min(img.height() - 1);

        Color::from_rgba(img.get_pixel(x, y))
    }
}
