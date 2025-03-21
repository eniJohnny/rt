use super::vec3::Vec3;
use std::collections::HashMap;
use image::RgbaImage;
use crate::model::{
    materials::{color::Color, material::Projection, texture::Texture},
    element::Element,
};

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
	transparency: f64,
    emissive: f64,
    opacity: f64,
    reflectivity: f64,
    all_dist: Vec<f64>,
    t_list: Vec<(&'a Element, Vec<f64>)>
}

impl<'a> Hit<'a> {
    pub fn new(
        element: &'a Element,
        dist: f64,
        pos: Vec3,
        ray_dir: &Vec3,
        textures: &HashMap<String, RgbaImage>,
        all_dist: Vec<f64>
    ) -> Self {
        let mut norm = element.shape().norm(&pos);
        if norm.dot(ray_dir) > 0. {
            norm = -norm;
        }
        let mut hit = Hit {
            element,
            dist,
            norm,
            pos,
            projection: None,
            color: Color::new(0., 0., 0.),
            metalness: 0.,
            roughness: 0.,
            transparency: 0.,
            emissive: 0.,
            reflectivity: element.material().reflectivity(),
            opacity: 1.,
            all_dist,
            t_list: vec![]
        };
        hit.map_norm(textures);
        hit.map_opacity(textures);
        hit
    }

    pub fn set_t_list(&mut self, t_list: Vec<(&'a Element, Vec<f64>)>) {
        self.t_list = t_list;
    }
    pub fn set_norm(&mut self, norm: Vec3) {
        self.norm = norm;
    }

    pub fn t_list(&self) -> &Vec<(&'a Element, Vec<f64>)> {
        &self.t_list
    }

    pub fn t_list_mut(&mut self) -> &mut Vec<(&'a Element, Vec<f64>)> {
        &mut self.t_list
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

    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
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
    pub fn transparency(&self) -> f64 {
        self.transparency
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
    pub fn reflectivity(&self) -> f64 {
        self.reflectivity
    }

    pub fn map_texture(&mut self, texture: &Texture, map: &HashMap<String, RgbaImage>, default: Vec3) -> Vec3 {
        match texture {
            Texture::Texture(file, _) => {
                let projection = self.projection();
                if let Some(img) = map.get(file) {
                    let color = Texture::get(projection, img);
                    return Vec3::from_color(color);
                }
                default
            }
            Texture::Value(value, _) => value.clone(),
        }
    }

    fn map_color(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.color = Color::from_vec3(&self.map_texture(self.element.material().color(), textures, Vec3::from_value(1.)));
    }

    fn map_norm(&mut self, textures: &HashMap<String, RgbaImage>) {
        let vec = self.map_texture(self.element.material().norm(), textures, Vec3::new(0., 0., 1.));
        let norm: Vec3;
        let mut is_value = false;
        if let Texture::Value(_, _) = self.element.material().norm() {
            is_value = true;
        }
        let projection = self.projection();
        if is_value {
            norm = *vec.x() * projection.i.clone()
                + *vec.y() * projection.j.clone()
                + *vec.z() * projection.k.clone();
        } else {
            norm = (vec.x() - 0.5) * 2. * projection.i.clone()
                + ((1. - vec.y()) - 0.5) * 2. * projection.j.clone()
                + (vec.z() - 0.5) * 2. * projection.k.clone();
        }
        self.norm = norm.normalize();
    }

    fn map_roughness(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.roughness = self
            .map_texture(self.element.material().roughness(), textures, Vec3::from_value(1.))
            .to_value();
    }

    fn map_metalness(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.metalness = self
            .map_texture(self.element.material().metalness(), textures, Vec3::from_value(0.))
            .to_value();
    }

    fn map_emissive(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.emissive = self
            .map_texture(self.element.material().emissive(), textures, Vec3::from_value(0.))
            .to_value() * self.element.material().emissive_intensity();
    }

    fn map_transparency(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.transparency = self
            .map_texture(self.element.material().transparency(), textures, Vec3::from_value(0.))
            .to_value();
    }

    fn map_opacity(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.opacity = self
            .map_texture(self.element.material().opacity(), textures, Vec3::from_value(1.))
            .to_value();
    }

    pub fn projection(&mut self) -> &Projection {
        let projection = match self.projection.take() {
            None => {
                let mut projection = self.element().shape().projection(self);
                projection.u = (projection.u * self.element().material().u_scale() - self.element().material().u_shift()).rem_euclid(1.);
                projection.v = (projection.v * self.element().material().v_scale() - self.element().material().v_shift()).rem_euclid(1.);
                projection
            },
            Some(p) => p,
        };
        self.projection = Some(projection);
        self.projection.as_ref().unwrap()
    }

    pub fn map_textures(&mut self, textures: &HashMap<String, RgbaImage>) {
        self.map_color(textures);
        self.map_roughness(textures);
        self.map_metalness(textures);
		self.map_transparency(textures);
        self.map_emissive(textures);
    }

    pub fn all_dist(&self) -> &Vec<f64> {
        &self.all_dist
    }
}
