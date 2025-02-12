use std::collections::HashMap;

use image::RgbaImage;

use crate::model::{materials::texture::TexturePattern, maths::vec3::Vec3};

use super::{basic::{get_number, get_string, get_vec3}, json::JsonValue};



pub fn get_texture(json_obj: &HashMap<String, JsonValue>) -> Result<(String, RgbaImage), String> {
    let pattern = get_string(&json_obj, "pattern", None)?;
    let name = get_string(&json_obj, "name", None)?;
    let res_x = get_number(&json_obj, "res_x", Some(0.), None, Some(500.))? as u32;
    let res_y = get_number(&json_obj, "res_y", Some(0.), None, Some(500.))? as u32;
    let img = match pattern.as_str() {
        "sine" => {
            get_sine_texture(json_obj, res_x, res_y)?
        },
        "worley" => {
            get_worley_texture(json_obj, res_x, res_y)?
        },
        "bumpSine" => {
            get_bump_sine(json_obj, res_x, res_y)?
        }
        "checkerboard" => {
            get_checkerboard_texture(json_obj, res_x, res_y)?
        },
        _ => {
            return Err(format!("The pattern {} of the texture named {} isn't a known pattern", pattern, name));
        }
    };
    Ok((name, img))
}

pub fn get_sine_texture(json_obj: &HashMap<String, JsonValue>, res_x: u32, res_y: u32) -> Result<RgbaImage, String> {
    let u_min = get_vec3(json_obj, "u_min", Some(0.), Some(255.), Some(Vec3::new(255., 255., 255.)))? / 255.;
    let u_max = get_vec3(json_obj, "u_max", Some(0.), Some(255.), Some(Vec3::new(0., 0., 0.)))? / 255.;
    let v_min = get_vec3(json_obj, "v_min", Some(0.), Some(255.), Some(Vec3::new(255., 255., 255.)))? / 255.;
    let v_max = get_vec3(json_obj, "v_max", Some(0.), Some(255.), Some(Vec3::new(0., 0., 0.)))? / 255.;
    Ok(TexturePattern::Cosine(u_min, u_max, v_min, v_max).generate(res_x, res_y))
}

pub fn get_bump_sine(json_obj: &HashMap<String, JsonValue>, res_x: u32, res_y: u32) -> Result<RgbaImage, String> {
    let u_variation = get_number(json_obj, "u_variation", None, None, Some(1.))?;
    let v_variation = get_number(json_obj, "v_variation", None, None, Some(1.))?;
    Ok(TexturePattern::BumpSine(u_variation, v_variation).generate(res_x, res_y))
}

pub fn get_checkerboard_texture(json_obj: &HashMap<String, JsonValue>, res_x: u32, res_y: u32) -> Result<RgbaImage, String> {
    let color_a = get_vec3(json_obj, "color_a", Some(0.), Some(255.), Some(Vec3::new(255., 255., 255.)))? / 255.;
    let color_b = get_vec3(json_obj, "color_b", Some(0.), Some(255.), Some(Vec3::new(0., 0., 0.)))? / 255.;
    Ok(TexturePattern::CheckerBoard(color_a, color_b).generate(res_x, res_y))
}

pub fn get_worley_texture(json_obj: &HashMap<String, JsonValue>, res_x: u32, res_y: u32) -> Result<RgbaImage, String> {
    let dots = get_number(json_obj, "dots", Some(1.), None, Some(30.))? as u32;
    let color_a = get_vec3(json_obj, "color_a", Some(0.), Some(255.), Some(Vec3::new(255., 255., 255.)))? / 255.;
    let color_b = get_vec3(json_obj, "color_b", Some(0.), Some(255.), Some(Vec3::new(0., 0., 0.)))? / 255.;
    Ok(TexturePattern::Worley(dots, color_a, color_b).generate(res_x, res_y))
}