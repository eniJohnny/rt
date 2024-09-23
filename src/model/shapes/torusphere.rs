use std::f64::consts::PI;

use crate::model::{materials::{color, material::Material, texture::{Texture, TextureType}}, maths::vec3::Vec3, Element};
use super::sphere::Sphere;

pub struct Torusphere {
    pub pos: Vec3,
    pub dir: Vec3,
    pub radius: f64,
    pub sphere_radius: f64,
    pub sphere_color: Vec3,
    pub material: Box<dyn Material>,
    pub elements: Vec<Element>,
}

impl Torusphere {
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, color: Vec3) -> Torusphere {
        let mut sph_vec: [Vec3; 12] = [Default::default(); 12];
        let mut elements: Vec<Element> = Vec::new();
        let mut material: Box<dyn Material> = <dyn Material>::default();
        material.set_color(Texture::Value(color, TextureType::Color));
        
        let sphere_radius = 0.2 * radius;
        let dir_y =  dir;
        let dir_x;
        if dir == Vec3::new(0.0, 1.0, 0.0) {
            dir_x =  Vec3::new(1.0, 0.0, 0.0);
        } else {
            dir_x =  Vec3::new(0.0, 1.0, 0.0);
        }

        sph_vec[0] = dir_y;
        sph_vec[1] = (PI / 3.0).sin() * dir_y + (PI / 3.0).cos() * dir_x;
        sph_vec[2] = (PI / 6.0).sin() * dir_y + (PI / 6.0).cos() * dir_x;
        sph_vec[3] = dir_x;
        sph_vec[4] = (-PI / 6.0).sin() * dir_y + (-PI / 6.0).cos() * dir_x;
        sph_vec[5] = (-PI / 3.0).sin() * dir_y + (-PI / 3.0).cos() * dir_x;
        sph_vec[6] = -dir_y;
        sph_vec[7] = (2. * -PI / 3.0).sin() * dir_y + (2. * -PI / 3.0).cos() * dir_x;
        sph_vec[8] = (5. * -PI / 6.0).sin() * dir_y + (5. * -PI / 6.0).cos() * dir_x;
        sph_vec[9] = -dir_x;
        sph_vec[10] = (5. * PI / 6.0).sin() * dir_y + (5. * PI / 6.0).cos() * dir_x;
        sph_vec[11] = (2. * PI / 3.0).sin() * dir_y + (2. * PI / 3.0).cos() * dir_x;

        for i in 0..12 {
            let sphere = Sphere::new(pos + sph_vec[i] * radius, dir_y, sphere_radius);
            let element = Element::new(Box::new(sphere), material.copy());
            elements.push(element);
        }

        Torusphere {
            pos,
            dir,
            radius,
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

        *self = Torusphere::new(pos, dir, radius, color);
    }
}