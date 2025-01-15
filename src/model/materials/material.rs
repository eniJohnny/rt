use super::{diffuse::Diffuse, texture::Texture};
use crate::model::maths::vec3::Vec3;
use std::fmt::Debug;

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
    fn transparency(&self) -> &Texture;
    fn roughness(&self) -> &Texture;
    fn emissive(&self) -> &Texture;
    fn emissive_intensity(&self) -> f64;
    fn opacity(&self) -> &Texture;
    fn displacement(&self) -> &Texture;
    fn refraction(&self) -> f64;
    fn u_size(&self) -> f64;
    fn v_size(&self) -> f64;
    fn u_shift(&self) -> f64;
    fn v_shift(&self) -> f64;

    fn set_color(&mut self, color: Texture);
    fn set_norm(&mut self, norm: Texture);
    fn set_metalness(&mut self, metalness: Texture);
    fn set_transparency(&mut self, transparency: Texture);
    fn set_roughness(&mut self, roughness: Texture);
    fn set_emissive(&mut self, emissive: Texture);
    fn set_emissive_intensity(&mut self, emissive: f64);
    fn set_opacity(&mut self, opacity: Texture);
    fn set_displacement(&mut self, displacement: Texture);
    fn set_refraction(&mut self, refraction: f64);
    fn set_u_size(&mut self, u_size: f64);
    fn set_v_size(&mut self, v_size: f64);
    fn set_u_shift(&mut self, u_shift: f64);
    fn set_v_shift(&mut self, v_shift: f64);

    fn clone(&self) -> Box<dyn Material + Send +Sync> {
        Box::new(Diffuse::new(self.color().clone(), self.metalness().clone(), self.roughness().clone(), self.emissive().clone(), self.emissive_intensity(), self.transparency().clone(), self.norm().clone(), self.opacity().clone(), self.displacement().clone(), self.refraction(), self.u_size(), self.v_size(), self.u_shift(), self.v_shift()))
    }
}