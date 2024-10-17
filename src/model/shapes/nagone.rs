use std::f64::consts::PI;

use crate::model::{materials::{color, material::Material, texture::{Texture, TextureType}}, maths::vec3::Vec3, Element};
use super::{cylinder::{self, Cylinder}, sphere::Sphere, ComposedShape};

#[derive(Debug)]
pub struct Nagone {
    pub pos: Vec3,
    pub dir: Vec3,
    pub radius: f64,
    pub angles: usize,
    pub color: Vec3,
    pub material: Box<dyn Material>,
    pub elements: Vec<Element>,
}

impl ComposedShape for Nagone {
    fn material(&self) -> &dyn Material {
        return self.material.as_ref();
    }
    fn elements(&self) -> &Vec<Element> {
        return &self.elements();
    }
    fn elements_as_mut(&mut self) -> &mut Vec<Element> {
        return &mut self.elements;
    }
    fn as_nagone(&self) -> Option<&self::Nagone> {
        return Some(self);
    }
}

impl Nagone {
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, angles: usize, color: Vec3) -> Nagone {
        if angles < 3 {
            panic!("Nagone must have at least 3 angles");
        }

        let mut elements: Vec<Element> = Vec::new();
        let mut material: Box<dyn Material> = <dyn Material>::default();
        
        material.set_color(Texture::Value(color, TextureType::Color));
        material.set_opacity(Texture::Value(Vec3::from_value(1.0), TextureType::Float));

        let dir_y = dir.normalize();
        let dir_x;

        if dir == Vec3::new(0.0, 1.0, 0.0) {
            dir_x = Vec3::new(1.0, 0.0, 0.0);
        } else {
            dir_x = Vec3::new(0.0, 1.0, 0.0);
        }

        let mut origins_dirs: Vec<Vec3> = Vec::new();
        let sphere_radius = radius / angles as f64 * 1.3;
        let cylinder_radius = 0.5 * sphere_radius;

        for i in 1..angles + 1 {
            let factor = (i * 2) as f64;
            origins_dirs.push((PI * factor / angles as f64).sin() * dir_y + (PI * factor / angles as f64).cos() * dir_x);
        }

        for i in 0..angles {
            let sphere = Sphere::new(pos + origins_dirs[i] * radius, dir_y, sphere_radius);
            elements.push(Element::new(Box::new(sphere), material.copy()));

            let next_i = (i + 1) % angles;
            let cylinder_dir = ((pos + origins_dirs[next_i] * radius) - (pos + origins_dirs[i] * radius)).normalize();
            let cylinder_height = ((pos + origins_dirs[next_i] * radius) - (pos + origins_dirs[i] * radius)).length();

            let cylinder = Cylinder::new(pos + origins_dirs[i] * radius, cylinder_dir, cylinder_radius, cylinder_height);
            elements.push(Element::new(Box::new(cylinder), material.copy()));
        }

        Nagone {
            pos,
            dir,
            radius,
            angles,
            color,
            material,
            elements,
        }

    }

    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn angles(&self) -> usize { self.angles }
    pub fn color(&self) -> &Vec3 { &self.color }
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
    pub fn set_angles(&mut self, angles: usize) {
        self.angles = angles;
        self.update();
    }
    pub fn set_color(&mut self, color: Vec3) {
        self.color = color;
        self.material.set_color(Texture::Value(color, TextureType::Color));
        self.update();
    }

    // Methods
    pub fn update(&mut self) {
        let pos = self.pos;
        let dir = self.dir;
        let radius = self.radius;
        let color = self.color;
        let angles = self.angles;

        *self = Nagone::new(pos, dir, radius, angles, color);
    }
}