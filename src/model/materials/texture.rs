use std::f64::consts::PI;

use super::{color::Color, material::Projection};
use crate::{model::maths::{vec2::Vec2, vec3::Vec3}, ui::utils::misc::Value};
use image::RgbaImage;
use rand::Rng;

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
    pub fn from_vector(file: &str, default_vec: Vec3) -> Self {
        if file == "" {
            Texture::Value(default_vec, TextureType::Vector)
        } else {
            Texture::Texture(file.to_string(), TextureType::Vector)
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

    pub fn from_float_scaled(string: &String, default: f64) -> Self {
        if let Ok(value) = string.parse::<f64>() {
            Texture::Value(Vec3::from_value(value), TextureType::Float)
        } else if string == "" {
            Texture::Value(Vec3::from_value(default), TextureType::Float)
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

#[derive(Clone, Debug)]
pub enum TexturePattern {
    Cosine(Vec3, Vec3, Vec3, Vec3),
    BumpSine(f64, f64),
    CheckerBoard(Vec3, Vec3),
    Worley(u32, Vec3, Vec3)
}

impl TexturePattern {
    pub fn generate(self, res_x: u32, res_y: u32) -> RgbaImage {
        let mut img = RgbaImage::new(res_x, res_y);
        match self {
            TexturePattern::Cosine(u_min, u_max, v_min, v_max) => {
                for (x, y, pixel) in img.enumerate_pixels_mut() {
                    let u_ratio = (((x as f64 / res_x as f64) * 2. * PI - PI).cos() + 1.) / 2.0;
                    let v_ratio = (((y as f64 / res_y as f64) * 2. * PI - PI).cos() + 1.) / 2.0;
                    let color_u = u_min + (u_ratio * (u_max - u_min));
                    let color_v = v_min + (v_ratio * (v_max - v_min));
                    let final_color_vec = (color_u + color_v) / 2.;
                    let final_color = Color::from_vec3(&final_color_vec).to_rgba();
                    pixel.0 = final_color.0;
                }
            },
            TexturePattern::BumpSine(u_variation, v_variation) => {
                for x in 0..res_x {
                    let u_cos = - ((x as f64 / res_x as f64) * 2. * PI - PI).sin() * u_variation;
                    for y in 0..res_y {
                        let v_cos =  -((y as f64 / res_y as f64) * 2. * PI - PI).sin() * v_variation;
                        let norm = Vec3::new(-u_cos * u_variation, -v_cos * v_variation, 1.).normalize() / 2. + Vec3::new(0.5, 0.5, 0.5);
                        img.put_pixel(x, y, Color::from_vec3(&norm).to_rgba());
                    }
                }
            }
            TexturePattern::CheckerBoard(color_a, color_b) => {
                for (x, y, pixel) in img.enumerate_pixels_mut() {
                    let u_color = (x as f64 / res_x as f64) < 0.5;
                    let v_color = (y as f64 / res_y as f64) >= 0.5;
                    if u_color != v_color {
                        pixel.0 = Color::from_vec3(&color_a).to_rgba().0;
                    } else {
                        pixel.0 = Color::from_vec3(&color_b).to_rgba().0;
                    }
                }
            },
            TexturePattern::Worley(dots_number, color_a, color_b) => {
                let mut dots = vec![];
                for _ in 0..dots_number {
                    let x = rand::thread_rng().gen_range((0.)..(1.) as f64);
                    let y = rand::thread_rng().gen_range((0.)..(1.) as f64);

                    dots.push(Vec2::new(x, y));
                }
                let mut min = 1.;
                let mut next_min = 1.;
                for (x, y, pixel) in img.enumerate_pixels_mut() {
                    for dot in &dots {
                        let mut x_dist = ((x as f64 / res_x as f64) - dot.x()).abs();
                        if x_dist > 0.5 {
                            x_dist = 1. - x_dist;
                        }
                        let mut y_dist = ((y as f64 / res_y as f64) - dot.y()).abs(); 
                        if y_dist > 0.5 {
                            y_dist = 1. - y_dist;
                        }
                        let dist = (x_dist * x_dist + y_dist * y_dist).sqrt();
                        match dist {
                            n if n < min => {
                                next_min = min;
                                min = n;
                            },
                            n if n >= min && n < next_min => {
                                next_min = n;
                            }
                            _ => {}
                        }
                    }
                    if dots.len() > 1 {
                        let color_vec = color_a + (color_b - color_a) * ((next_min - min) / next_min).sqrt().min(1.);
                        pixel.0 = Color::from_vec3(&color_vec).to_rgba().0;
                    }
                    min = 1.;
                    next_min = 1.;
                }
            }
        }
        img
    }
}