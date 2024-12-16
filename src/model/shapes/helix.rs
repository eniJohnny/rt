use super::{sphere::Sphere, cylinder::Cylinder, ComposedShape};
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
pub struct Helix {
    pub pos: Vec3,
    pub dir: Vec3,
    pub height: f64,
    pub material: Box<dyn Material>,
    pub sphere_material: Box<dyn Material>,
    pub elements: Vec<Element>,
}

impl ComposedShape for Helix {
    fn material(&self) -> &dyn Material {
        return self.material.as_ref();
    }
    fn elements(&self) -> &Vec<Element> {
        return &self.elements();
    }
    fn elements_as_mut(&mut self) -> &mut Vec<Element> {
        return &mut self.elements;
    }
    fn as_helix(&self) -> Option<&self::Helix> {
        return Some(self);
    }
}

impl Helix {
    pub fn new(pos: Vec3, dir: Vec3, height: f64) -> Helix {
        let mut elements: Vec<Element> = Vec::new();
        let mut material: Box<Diffuse> = Diffuse::default();
        let mut sphere_material: Box<Diffuse> = Diffuse::default();
        let link_color = Texture::Value(Vec3::from_value(1.0), TextureType::Color);
        let sphere_color = Texture::Value(Vec3::new(1.0, 0.0,0.0), TextureType::Color);

        // Materials
        material.set_color(link_color);
        sphere_material.set_color(sphere_color);
        material.set_opacity(Texture::Value(Vec3::from_value(1.0), TextureType::Float));
        sphere_material.set_opacity(Texture::Value(Vec3::from_value(1.0), TextureType::Float));

        // Elements
        let steps = 20;
        let link_length = 0.3 * height;
        let link_radius = 0.25 * height / steps as f64;
        let sphere_radius = 2.0 * link_radius;

        let cross_vector;
        if dir == Vec3::new(0.0, 1.0, 0.0) {
            cross_vector = dir.cross(&Vec3::new(1.0, 0.0, 0.0));
        } else {
            cross_vector = dir.cross(&Vec3::new(0.0, 1.0, 0.0));
        }

        let pos = pos - dir * height / 2.0;
        let rotation_ratio = 2.0 * PI / steps as f64;

        for i in 1..steps + 1 {
            let current_dir = cross_vector.rotate_from_axis_angle(i as f64 * rotation_ratio, &dir);
            let mut origin = pos - current_dir * link_length / 2.0;
            origin = origin + dir * height / steps as f64 * i as f64;

            let link = Cylinder::new(origin, current_dir, link_radius, link_length);
            let sphere1 = Sphere::new(origin, current_dir, sphere_radius);
            let sphere2 = Sphere::new(origin + current_dir * link_length, current_dir, sphere_radius);

            let link_element = Element::new(Box::new(link), material.clone());
            let sphere1_element = Element::new(Box::new(sphere1), sphere_material.clone());
            let sphere2_element = Element::new(Box::new(sphere2), sphere_material.clone());

            elements.push(link_element);
            elements.push(sphere1_element);
            elements.push(sphere2_element);
        }
        

        // Composed shape
        let helix = Helix {
            pos: pos,
            dir: dir,
            height: height,
            material: material,
            sphere_material: sphere_material,
            elements: elements,
        };
        
        return helix;
    }

    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn height(&self) -> f64 { self.height }
    pub fn material(&self) -> &dyn Material { self.material.as_ref() }
    pub fn sphere_material(&self) -> &dyn Material { self.sphere_material.as_ref() }
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
    pub fn set_height(&mut self, radius: f64) {
        self.height = radius;
        self.update();
    }

    // Methods
    pub fn update(&mut self) {
        let pos = self.pos;
        let dir = self.dir;
        let height = self.height;

        *self = Helix::new(pos, dir, height);
    }
}