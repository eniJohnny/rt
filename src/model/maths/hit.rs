use std::collections::HashMap;

use image::RgbaImage;

use crate::{
    model::{
        materials::{color::Color, material::Projection, texture::Texture},
        Element,
    },
    MAX_EMISSIVE,
};

use super::vec3::Vec3;

#[derive(Debug, Clone)]
pub enum HitType {
    Reflect,
    Direct,
}

#[derive(Debug, Clone)]
pub struct Hit<'a> {
    element: &'a Element,
    dist: f64,
    pos: Vec3,
    norm: Vec3,
    projection: Option<Projection>,
    color: Color,
    metalness: f64,
    roughness: f64,
    refraction: f64,
    emissive: f64,
    opacity: f64,
}

impl<'a> Hit<'a> {
    pub fn new(
        element: &'a Element,
        dist: f64,
        pos: Vec3,
        ray_dir: &Vec3,
        textures: &HashMap<String, RgbaImage>,
    ) -> Self {
        let mut hit = Hit {
            element,
            dist,
            norm: element.shape().norm(&pos, &ray_dir),
            pos,
            projection: None,
            color: Color::new(0., 0., 0.),
            metalness: 0.,
            roughness: 0.,
            refraction: 0.,
            emissive: 0.,
            opacity: 1.,
        };
        hit.map_norm(textures);
        hit.map_opacity(textures);
        hit
    }

    pub fn element(&self) -> &'a Element {
        self.element
    }

    pub fn dist(&self) -> &f64 {
        &self.dist
    }

    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub fn norm(&self) -> &Vec3 {
        &self.norm
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
    pub fn metalness(&self) -> f64 {
        self.metalness
    }
    pub fn refraction(&self) -> f64 {
        self.refraction
    }
    pub fn roughness(&self) -> f64 {
        self.roughness
    }
    pub fn emissive(&self) -> f64 {
        self.emissive
    }
    pub fn opacity(&self) -> f64 {
        self.opacity
    }

    fn map_texture(&mut self, texture: &Texture, map: &HashMap<String, RgbaImage>) -> Vec3 {
        match texture {
            Texture::Texture(file) => {
                let projection = self.projection();
                let img = map.get(file).unwrap();
                let color = Texture::get(projection, img);
                Vec3::from_color(color)
            }
            Texture::Value(value) => value.clone(),
        }
    }

    fn map_color(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.color = Color::from_vec3(&self.map_texture(self.element.material().color(), textures));
    }

    fn map_norm(&mut self, textures: &HashMap<String, RgbaImage>) {
        let vec = self.map_texture(self.element.material().norm(), textures);
        let projection = self.projection();
        let norm = (vec.x() - 0.5) * 2. * projection.i.clone()
            + (-vec.y() + 0.5) * 2. * projection.j.clone()
            + (vec.z() - 0.5) * 2. * projection.k.clone();
        self.norm = norm.normalize();
    }

    fn map_roughness(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.roughness = self
            .map_texture(self.element.material().roughness(), textures)
            .to_value();
    }

    fn map_metalness(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.metalness = self
            .map_texture(self.element.material().metalness(), textures)
            .to_value();
    }

    fn map_emissive(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.emissive = self
            .map_texture(self.element.material().emissive(), textures)
            .to_value()
            * MAX_EMISSIVE;
    }

    fn map_refraction(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.refraction = self
            .map_texture(self.element.material().refraction(), textures)
            .to_value();
    }

    //Set to public, as we are checking opacity at every hit during get_closest_hit, so we don't need to compute the rest if we only go through
    fn map_opacity(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.opacity = self
            .map_texture(self.element.material().opacity(), textures)
            .to_value();
    }

    fn projection(&mut self) -> &Projection {
        let projection = match self.projection.take() {
            None => self.element().shape().projection(self),
            Some(p) => p,
        };
        self.projection = Some(projection);
        self.projection.as_ref().unwrap()
    }

    pub fn map_textures(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.map_color(textures);
        self.map_roughness(textures);
        self.map_metalness(textures);
        self.map_refraction(textures);
        self.map_emissive(textures);
    }
}
