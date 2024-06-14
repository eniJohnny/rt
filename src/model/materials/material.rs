use std::fmt::Debug;

use crate::model::maths::{hit::Hit, vec3::Vec3};

use super::{color::Color, diffuse::Diffuse, texture::Texture};

pub trait Material: Debug + Sync + Send {
    fn color(&self) -> &Texture;
    fn norm(&self) -> &Texture;
    fn metalness(&self) -> &Texture;
    fn refraction(&self) -> &Texture;
    fn roughness(&self) -> &Texture;
    fn emissive(&self) -> &Texture;

    fn set_color(&mut self, color: Texture);
    fn set_norm(&mut self, norm: Texture);
    fn set_metalness(&mut self, metalness: Texture);
    fn set_refraction(&mut self, refraction: Texture);
    fn set_roughness(&mut self, roughness: Texture);
    fn set_emissive(&mut self, emissive: Texture);
}

impl dyn Material {
    pub fn default() -> Box<Self> {
        Box::new(Diffuse::new(Texture::Value(Vec3::from_value(0.)), Texture::Value(Vec3::from_value(0.)), Texture::Value(Vec3::from_value(0.)), Texture::Value(Vec3::from_value(0.)), Texture::Value(Vec3::from_value(0.)), Texture::Value(Vec3::from_value(0.))))
    }
}