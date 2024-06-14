use crate::model::maths::{hit::Hit, vec3::Vec3};

use super::{color::Color, material::Material, texture::Texture};

#[derive(Clone, Debug)]
pub struct Diffuse {
    color: Texture,
    metalness: Texture,
    roughness: Texture,
    refraction: Texture,
    norm_variation: Texture,
    emissive: Texture
}

impl Diffuse {
    pub fn new(color: Texture, metalness: Texture, roughness: Texture, emissive: Texture, refraction: Texture, norm_variation: Texture) -> Self {
        Self { color, metalness, roughness, emissive, refraction, norm_variation}
    }
}

unsafe impl Send for Diffuse {}

impl Material for Diffuse {
    fn color(&self) -> &Texture {
        &self.color
    }
    fn set_color(&mut self, color: Texture) {
        self.color = color;
    }

    fn norm(&self) -> &Texture {
        &self.norm_variation
    }
    fn set_norm(&mut self, norm: Texture) {
        self.norm_variation = norm;
    }

    fn metalness(&self) -> &Texture {
        &self.metalness
    }
    fn set_metalness(&mut self, metalness: Texture) {
        self.metalness = metalness;
    }

    fn refraction(&self) -> &Texture {
        &self.refraction
    }
    fn set_refraction(&mut self, refraction: Texture) {
        self.refraction = refraction;
    }

    fn roughness(&self) -> &Texture {
        &self.roughness
    }
    fn set_roughness(&mut self, roughness: Texture) {
        self.roughness = roughness;
    }

    fn emissive(&self) -> &Texture {
        &self.emissive
    }
    fn set_emissive(&mut self, emissive: Texture) {
        self.emissive = emissive;
    }

}
