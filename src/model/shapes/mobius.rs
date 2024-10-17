use std::f64::consts::PI;

use super::triangle::Triangle;
use super::ComposedShape;
use crate::model::materials::material::Material;
use crate::model::materials::texture::{Texture, TextureType};
use crate::model::maths::vec3::Vec3;
use crate::model::Element;

#[derive(Debug)]
pub struct Mobius {
    pub pos: Vec3,
    pub radius: f64,
    pub half_width: f64,
    pub color: Vec3,
    pub material: Box<dyn Material>,
    pub elements: Vec<Element>,
}

impl ComposedShape for Mobius {
    fn material(&self) -> &dyn Material {
        return self.material.as_ref();
    }
    fn elements(&self) -> &Vec<Element> {
        return &self.elements();
    }
    fn elements_as_mut(&mut self) -> &mut Vec<Element> {
        return &mut self.elements;
    }
    fn as_mobius(&self) -> Option<&self::Mobius> {
        return Some(self);
    }
}

impl Mobius {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn half_width(&self) -> f64 {
        self.half_width
    }
    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }
    pub fn color(&self) -> &Vec3 {
        &self.color
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.update();
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
        self.update();
    }
    pub fn set_half_width(&mut self, half_width: f64) {
        self.half_width = half_width;
        self.update();
    }
    pub fn set_elements(&mut self, elements: Vec<Element>) {
        self.elements = elements;
        self.update();
    }
    pub fn set_color(&mut self, color: Vec3) {
        self.color = color;
        self.material.set_color(Texture::Value(color, TextureType::Color));
        self.update();
    }

    // Constructor
    pub fn new(pos: Vec3, radius: f64, half_width: f64, color: Vec3) -> Mobius {
        let mut elements: Vec<Element> = Vec::new();
        let mut material: Box<dyn Material> = <dyn Material>::default();
        material.set_color(Texture::Value(color, TextureType::Color));
        material.set_opacity(Texture::Value(Vec3::from_value(1.0), TextureType::Float));

        let step = 0.1;
        let mut v = 0.0;
        while v < PI {
            v = v.min(PI);
            let mut t = -half_width;

            while t < half_width {
                t = t.min(half_width);

                let p1 = compute_position(v, t, pos);
                let p2 = compute_position(v + step, t, pos);
                let p3 = compute_position(v, t + step, pos);
                let p4 = compute_position(v + step, t + step, pos);

                let triangle1 = Triangle::new(p1, p2, p3);
                let triangle2 = Triangle::new(p3, p2, p4);

                let element1 = Element::new(Box::new(triangle1), material.copy());
                let element2 = Element::new(Box::new(triangle2), material.copy());

                elements.push(element1);
                elements.push(element2);

                t += step;
            }

            v += step;
        }

        Mobius {
            pos,
            radius,
            half_width,
            color,
            material,
            elements,
        }
    }

    // Methods
    pub fn update(&mut self) {
        let pos = self.pos;
        let radius = self.radius;
        let half_width = self.half_width;
        let color = self.color;

        *self = Mobius::new(pos, radius, half_width, color);
    }
}

fn compute_position(v: f64, t: f64, pos: Vec3) -> Vec3 {
    let cdv = (v * 2.0).cos();
    let sdv = (v * 2.0).sin();
    let ctv = v.cos();
    let stv = v.sin();
    let c = 2.0 + t * ctv;

    Vec3::new(
        c * cdv,
        c * sdv,
        t * stv,
    ) + pos
}