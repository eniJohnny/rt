use std::fmt::Debug;

use crate::model::maths::vec3::Vec3;

use super::
    texture::Texture
;

#[derive(Debug, Clone, Default)]
pub struct Projection {
    pub u: f64,
    pub v: f64,
    pub i: Vec3,
    pub j: Vec3,
    pub k: Vec3,
}

pub trait Material: Debug + Sync + Send {
    fn color(&self) -> &Texture;
    fn norm(&self) -> &Texture;
    fn metalness(&self) -> &Texture;
    fn refraction(&self) -> &Texture;
    fn roughness(&self) -> &Texture;
    fn emissive(&self) -> &Texture;
    fn opacity(&self) -> &Texture;
	fn displacement(&self) -> &Texture;

    fn set_color(&mut self, color: Texture);
    fn set_norm(&mut self, norm: Texture);
    fn set_metalness(&mut self, metalness: Texture);
    fn set_refraction(&mut self, refraction: Texture);
    fn set_roughness(&mut self, roughness: Texture);
    fn set_emissive(&mut self, emissive: Texture);
    fn set_opacity(&mut self, opacity: Texture);
	fn set_displacement(&mut self, displacement: Texture);
}