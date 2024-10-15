use std::f64::consts::PI;
use std::ops::Div;

use super::ellipse::Ellipse;
use super::rectangle::Rectangle;
use super::{ComposedShape, Shape};
use crate::model::materials::material::Material;
use crate::model::materials::texture::{Texture, TextureType};
use crate::model::maths::vec3::Vec3;
use crate::model::Element;

#[derive(Debug)]
pub struct CubeHole {
    pub pos: Vec3,
    pub dir: Vec3,
    pub dimensions: Vec3,
    pub color: Vec3,
    pub material: Box<dyn Material>,
    pub elements: Vec<Element>,
    pub caps: Vec<Ellipse>,
}

impl ComposedShape for CubeHole {
    fn material(&self) -> &dyn Material {
        return self.material.as_ref();
    }
    fn elements(&self) -> &Vec<Element> {
        return &self.elements();
    }
    fn elements_as_mut(&mut self) -> &mut Vec<Element> {
        return &mut self.elements;
    }
    fn as_cubehole(&self) -> Option<&self::CubeHole> {
        return Some(self);
    }
}

impl CubeHole {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn dimensions(&self) -> &Vec3 {
        &self.dimensions
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
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
    }
    pub fn set_dimensions(&mut self, dimensions: Vec3) {
        self.dimensions = dimensions;
    }
    pub fn set_elements(&mut self, elements: Vec<Element>) {
        self.elements = elements;
    }
    pub fn set_color(&mut self, color: Vec3) {
        self.color = color;
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, dimensions: Vec3, color: Vec3) -> CubeHole {
        let mut elements: Vec<Element> = Vec::new();
        let mut material: Box<dyn Material> = <dyn Material>::default();
        material.set_color(Texture::Value(color, TextureType::Color));
        material.set_opacity(Texture::Value(Vec3::from_value(1.0), TextureType::Float));

        let h = *dimensions.x();
        let w = *dimensions.y();
        let l = *dimensions.z();

        let dir_h = dir.clone().normalize();
        let dir_w;
        if dir_h == Vec3::new(0.0, 1.0, 0.0) {
            dir_w = dir_h.cross(&Vec3::new(0.0, 0.0, 1.0)).normalize();
        } else {
            dir_w = dir_h.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize();
        }
        let dir_l = dir_h.cross(&-dir_w).normalize();


        // Create the 6 rectangles of the cubehole in this order:
        // TOP, BOTTOM, FRONT, BACK, LEFT, RIGHT

        // The positions of the 6 rectangles (center)
        let rectangle_positions = [
            pos.clone() + dir_h.clone() * h / 2.,
            pos.clone() - dir_h.clone() * h / 2.,
            pos.clone() + dir_l.clone() * l / 2.,
            pos.clone() - dir_l.clone() * l / 2.,
            pos.clone() + dir_w.clone() * w / 2.,
            pos.clone() - dir_w.clone() * w / 2.,
        ];

        // The directions of the 6 rectangles (length, width)
        let rectangle_dirs = [
            (dir_l, dir_w),
            (dir_l, dir_w),
            (dir_w, dir_h),
            (dir_w, dir_h),
            (dir_l, dir_h),
            (dir_l, dir_h),
        ];

        // The dimensions of the 6 rectangles (length, width)
        let rectangle_dims = [
            (l, w),
            (l, w),
            (w, h),
            (w, h),
            (l, h),
            (l, h),
        ];

        // Create the 6 rectangles and add them to the elements vector
        for i in 0..rectangle_positions.len() {
            let rectangle = Rectangle::new(
                rectangle_positions[i],
                rectangle_dims[i].0,
                rectangle_dims[i].1,
                rectangle_dirs[i].0,
                rectangle_dirs[i].1,
            );

            material.set_color(Texture::Value(color, TextureType::Color));
            elements.push(Element::new(Box::new(rectangle), material.copy()));
        }

        // Create and return the cubehole
        CubeHole {
            pos,
            dir,
            dimensions,
            color,
            material,
            elements,
            caps: Vec::new(),
        }
    }
}
