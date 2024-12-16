use super::{sphere::Sphere, ComposedShape};
use std::f64::consts::PI;
use crate::model::{
    materials::{
        diffuse::Diffuse,
        material::Material,
        texture::{Texture, TextureType}
    },
    maths::vec3::Vec3,
    Element
};

#[derive(Debug)]
pub struct Torusphere {
    pub pos: Vec3,
    pub dir: Vec3,
    pub radius: f64,
    pub steps: usize,
    pub sphere_radius: f64,
    pub sphere_color: Vec3,
    pub material: Box<dyn Material>,
    pub elements: Vec<Element>,
}

impl ComposedShape for Torusphere {
    fn material(&self) -> &dyn Material {
        return self.material.as_ref();
    }
    fn elements(&self) -> &Vec<Element> {
        return &self.elements();
    }
    fn elements_as_mut(&mut self) -> &mut Vec<Element> {
        return &mut self.elements;
    }
    fn as_torusphere(&self) -> Option<&self::Torusphere> {
        return Some(self);
    }
}

impl Torusphere {
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, steps: usize, color: Vec3) -> Torusphere {
        let mut sph_vec: Vec<Vec3> = Vec::new();
        let mut elements: Vec<Element> = Vec::new();
        let mut material: Box<Diffuse> = Diffuse::default();
        material.set_color(Texture::Value(color, TextureType::Color));
        material.set_opacity(Texture::Value(Vec3::from_value(1.0), TextureType::Float));

        let sphere_radius = 0.2 * radius;
        let dir_y =  dir.normalize();
        let dir_x;
        if dir == Vec3::new(0.0, 1.0, 0.0) {
            dir_x =  Vec3::new(1.0, 0.0, 0.0);
        } else {
            dir_x =  Vec3::new(0.0, 1.0, 0.0);
        }

        for i in 1..steps + 1 {
            // let factor = (i as i32 - steps as i32 / 2) as f64 * 2.0;
            let factor = (i * 2) as f64;
            sph_vec.push((PI * factor / steps as f64).sin() * dir_y + (PI * factor / steps as f64).cos() * dir_x);
        }

        for i in 0..steps {
            let sphere = Sphere::new(pos + sph_vec[i] * radius, dir_y, sphere_radius);
            let element = Element::new(Box::new(sphere), material.clone());
            elements.push(element);
        }

        Torusphere {
            pos,
            dir,
            radius,
            steps,
            sphere_radius,
            sphere_color: color,
            material,
            elements: elements
        }
    }

    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn sphere_radius(&self) -> f64 { self.sphere_radius }
    pub fn sphere_color(&self) -> &Vec3 { &self.sphere_color }
    pub fn material(&self) -> &dyn Material { self.material.as_ref() }
    pub fn elements(&self) -> &Vec<Element> { &self.elements }

    // Setters
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.update();
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
        self.update();
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
        self.update();
    }
    pub fn set_color(&mut self, color: Vec3) {
        self.sphere_color = color;
        self.material.set_color(Texture::Value(color, TextureType::Color));
        self.update();
    }

    // Methods
    pub fn update(&mut self) {
        let pos = self.pos;
        let dir = self.dir;
        let radius = self.radius;
        let color = self.sphere_color;
        let steps = self.steps;

        *self = Torusphere::new(pos, dir, radius, steps, color);
    }
}