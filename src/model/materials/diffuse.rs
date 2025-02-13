use crate::model::maths::vec3::Vec3;
use super::{
    material::Material,
    texture::{Texture, TextureType}
};

#[derive(Clone, Debug)]
pub struct Diffuse {
    color: Texture,
    metalness: Texture,
    roughness: Texture,
    transparency: Texture,
    norm_variation: Texture,
    emissive: Texture,
    emissive_intensity: f64,
    opacity: Texture,
    displacement: Texture,
    refraction: f64,
    u_scale: f64,
    v_scale: f64,
    u_shift: f64,
    v_shift: f64,
}

impl Diffuse {
    pub fn new(
        color: Texture,
        metalness: Texture,
        roughness: Texture,
        emissive: Texture,
        emissive_intensity: f64,
        transparency: Texture,
        norm_variation: Texture,
        opacity: Texture,
        displacement: Texture,
        refraction: f64,
        u_scale: f64,
        v_scale: f64,
        u_shift: f64,
        v_shift: f64,
    ) -> Self {
        Self {
            color,
            metalness,
            roughness,
            emissive,
            emissive_intensity,
            transparency,
            norm_variation,
            opacity,
            displacement,
            refraction,
            u_scale,
            v_scale,
            u_shift,
            v_shift,
        }
    }

    pub fn default() -> Box<Self> {
        Box::new(Diffuse::new(
            Texture::Value(Vec3::from_value(1.), TextureType::Color),
            Texture::Value(Vec3::from_value(0.), TextureType::Float),
            Texture::Value(Vec3::from_value(0.5), TextureType::Float),
            Texture::Value(Vec3::from_value(0.), TextureType::Float),
            1.,
            Texture::Value(Vec3::from_value(0.), TextureType::Float),
            Texture::Value(Vec3::new(0., 0., 1.0), TextureType::Vector),
            Texture::Value(Vec3::from_value(1.), TextureType::Float),
            Texture::Value(Vec3::from_value(0.), TextureType::Float),
            0.,
            1.,
            1.,
            0.,
            0.,
        ))
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

    fn refraction(&self) -> f64 {
        self.refraction
    }
    fn set_refraction(&mut self, refraction: f64) {
        self.refraction = refraction;
    }

    fn transparency(&self) -> &Texture {
        &self.transparency
    }
    fn set_transparency(&mut self, transparency: Texture) {
        self.transparency = transparency;
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

    fn emissive_intensity(&self) -> f64 {
        self.emissive_intensity
    }

    fn set_emissive_intensity(&mut self, emissive_intensity: f64) {
        self.emissive_intensity = emissive_intensity;
    }

    fn opacity(&self) -> &Texture {
        &self.opacity
    }
    fn set_opacity(&mut self, opacity: Texture) {
        self.opacity = opacity;
    }

    fn displacement(&self) -> &Texture {
        &self.displacement
    }
    fn set_displacement(&mut self, displacement: Texture) {
        self.displacement = displacement;
    }

    fn u_scale(&self) -> f64 {
        self.u_scale
    }
    fn set_u_scale(&mut self, u_scale: f64) {
        self.u_scale = u_scale;
    }

    fn v_scale(&self) -> f64 {
        self.v_scale
    }
    fn set_v_scale(&mut self, v_scale: f64) {
        self.v_scale = v_scale;
    }

    fn u_shift(&self) -> f64 {
        self.u_shift
    }
    fn set_u_shift(&mut self, u_shift: f64) {
        self.u_shift = u_shift;
    }
    
    fn v_shift(&self) -> f64 {
        self.v_shift
    }
    fn set_v_shift(&mut self, v_shift: f64) {
        self.v_shift = v_shift;
    }
}
