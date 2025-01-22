use std::collections::HashMap;

use crate::model::{materials::{color::Color, texture::{Texture, TextureType}}, maths::vec3::Vec3};
use super::json::JsonValue;

pub fn get_vec3(object: &HashMap<String, JsonValue>, key: &str, min: Option<f64>, max: Option<f64>, default: Option<Vec3>) -> Result<Vec3, String> {
    if let Some(json_vec) = object.get(key) {
        if let JsonValue::Array(json_vec) = json_vec {
            if json_vec.len() != 3 {
                return Err(format!("The {} of an object must have 3 components", key));
            }
            let mut vec: Vec<f64> = Vec::new();
            for component in json_vec {
                if let JsonValue::Number(component) = component {
                    if let Some(min) = min {
                        if *component < min {
                            return Err(format!("The {} of an object must be greater than {}", key, min));
                        }
                    }
                    if let Some(max) = max {
                        if *component > max {
                            return Err(format!("The {} of an object must be lower than {}", key, max));
                        }
                    }
                    vec.push(*component);
                } else { return Err(format!("The {} of an object must only contains number", key)); }
            }
            return Ok(Vec3::new(vec[0], vec[1], vec[2]));
        }
        return Err(format!("The {} of an object must be an array", key));
    }
    if let Some(default) = default {
        return Ok(default);
    }
    Err(format!("The {} of an object is missing", key))
}

pub fn get_color(object: &HashMap<String, JsonValue>, key: &str) -> Result<Color, String> {
    return Ok(Color::from_vec3(&(get_vec3(object, key, Some(0.), Some(255.), None)? / 255.)));
}

pub fn get_string(object: &HashMap<String, JsonValue>, key: &str, default: Option<String>) -> Result<String, String> {
	if let Some(string) = object.get(key) {
		if let JsonValue::String(string) = string {
			return Ok(string.clone());
		}
		return Err(format!("The {} of an object must be a string", key));
	}
	if let Some(default) = default {
		return Ok(default);
	}
	Err(format!("The {} of an object is missing", key))
}

pub fn get_number(object: &HashMap<String, JsonValue>, key: &str, min: Option<f64>, max: Option<f64>, default: Option<f64>) -> Result<f64, String> {
    if let Some(number) = object.get(key) {
        if let JsonValue::Number(number) = number {
            if let Some(min) = min {
                if *number < min {
                    return Err(format!("The {} of an object must be greater than {}", key, min));
                }
            }
            if let Some(max) = max {
                if *number > max {
                    return Err(format!("The {} of an object must be lower than {}", key, max));
                }
            }
            return Ok(*number);
        }
        return Err(format!("The {} of an object must be an number", key))
    }
    if let Some(default) = default {
        return Ok(default);
    }
    Err(format!("The {} of an object is missing", key))
}

pub fn get_color_texture(json_color: &HashMap<String, JsonValue>) -> Result<Texture, String> {
    if let Some(json_color) = json_color.get("color") {
        match json_color {
            JsonValue::String(color_texture_path) => {
                return Ok(Texture::Texture(color_texture_path.clone(), TextureType::Color))
            }
            JsonValue::Array(color) => {
                if color.len() != 3 {
                    return Err("The color of an object must have 3 components".to_string());
                }
                let mut vec: Vec<f64> = Vec::new();
                for component in color {
                    if let JsonValue::Number(component) = component {
                        if *component < 0. || *component > 255. {
                            return Err("The color of an object must be in range [0; 255]".to_string());
                        }
                        vec.push(*component);
                    } else { return Err("The color of an object must only contains number".to_string()); }
                }
                return Ok(Texture::Value(Vec3::new(vec[0], vec[1], vec[2]) / 255., TextureType::Color))
            }
            _ => {
                return Err("The color of an object must be either an array or a file path".to_string());
            }
        }
    }
    Err("The color of an object is missing".to_string())
}

pub fn get_vec1_texture(json_texture: &HashMap<String, JsonValue>, key: &str, min: Option<f64>, max: Option<f64>, default: f64) -> Result<Texture, String> {
    if let Some(json_texture) = json_texture.get(key) {
        match json_texture {
            JsonValue::String(texture_path) => {
                return Ok(Texture::Texture(texture_path.clone(), TextureType::Float))
            }
            JsonValue::Number(value) => {
                if let Some(min) = min {
                    if *value < min {
                        return Err(format!("The {} of an object must be greater than {}", key, min));
                    }
                }
                if let Some(max) = max {
                    if *value > max {
                        return Err(format!("The {} of an object must be lower than {}", key, max));
                    }
                }
                return Ok(Texture::Value(Vec3::new(*value, *value, *value), TextureType::Float))
            }
            _ => {
                return Err(format!("The {} of an object must be either a float or a file path", key));
            }
        }
    }
    return Ok(Texture::Value(Vec3::new(default, default, default), TextureType::Float))
}

pub fn get_normal_texture(json_texture: &HashMap<String, JsonValue>) -> Result<Texture, String> {
    if let Some(json_texture) = json_texture.get("normal") {
        match json_texture {
            JsonValue::String(texture_path) => {
                return Ok(Texture::Texture(texture_path.clone(), TextureType::Vector))
            }
            _ => {
                return Err("The normal of an object must be a file path".to_string());
            }
        }
    }
    Ok(Texture::Value(Vec3::new(0., 0., 1.), TextureType::Vector))
}

pub fn get_opacity_texture(json_texture: &HashMap<String, JsonValue>) -> Result<Texture, String> {
    if let Some(json_texture) = json_texture.get("opacity") {
        match json_texture {
            JsonValue::String(texture_path) => {
                return Ok(Texture::Texture(texture_path.clone(), TextureType::Float))
            }
            JsonValue::Number(value) => {
                if *value < 0. {
                    return Err("The opacity of an object must be greater than 0".to_string());
                }
                if *value > 1. {
                    return Err("The opacity of an object must be lower than 1".to_string());
                }
                return Ok(Texture::Value(Vec3::new(*value, *value, *value), TextureType::Float))
            }
            _ => {
                return Err("The opacity of an object must be either a number or a file path".to_string());
            }
        }
    }
    return Ok(Texture::Value(Vec3::new(1., 1., 1.), TextureType::Float))
}

pub fn get_displacement_texture(json_texture: &HashMap<String, JsonValue>) -> Result<Texture, String> {
    if let Some(json_texture) = json_texture.get("displacement") {
        match json_texture {
            JsonValue::String(texture_path) => {
                return Ok(Texture::Texture(texture_path.clone(), TextureType::Float))
            }
            JsonValue::Number(value) => {
                if *value < 0. {
                    return Err("The displacement of an object must be greater than 0".to_string());
                }
                return Ok(Texture::Value(Vec3::new(*value, *value, *value), TextureType::Float))
            }
            _ => {
                return Err("The displacement of an object must be either a number or a file path".to_string());
            }
        }
    }
    return Ok(Texture::Value(Vec3::new(0., 0., 0.), TextureType::Float))
}