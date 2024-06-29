use crate::model::{
    materials::{
        color::Color,
        diffuse::Diffuse,
        material::Material,
        texture::{Texture, TextureType},
    },
    maths::vec3::Vec3,
    shapes::{cone, cylinder, plane, sphere, Shape},
};
use std::cmp::min;
pub fn update_color(key: String, value: String, material: &mut dyn Material) {
    if let Texture::Value(color, _) = material.color() {
        let mut new_color: (u8, u8, u8) = (
            (color.x() * 255.) as u8,
            (color.y() * 255.) as u8,
            (color.z() * 255.) as u8,
        );
        let new_value = min(value.parse::<u32>().unwrap(), 255) as u8;
        match key.as_str() {
            "colr" => {
                new_color.0 = new_value;
            }
            "colg" => {
                new_color.1 = new_value;
            }
            "colb" => {
                new_color.2 = new_value;
            }
            _ => (),
        }
        let new_color = Vec3::new(
            new_color.0 as f64 / 255.,
            new_color.1 as f64 / 255.,
            new_color.2 as f64 / 255.,
        );
        material.set_color(Texture::Value(new_color, TextureType::Color));
    }
}
