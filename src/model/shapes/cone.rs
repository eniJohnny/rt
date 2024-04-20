use super::Shape;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};

#[derive(Debug)]
pub struct Cone {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
    height: f64,
}

unsafe impl Send for Cone {}

impl Shape for Cone {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, vector: &Ray) -> Option<Vec<f64>> {
        unimplemented!()
    }
    fn projection(&self, hit: &Hit) -> (i32, i32) {
        unimplemented!()
    }
    fn norm(&self, hit_position: &Vec3) -> Vec3 {
        unimplemented!()
    }
    fn as_cone(&self) -> Option<&Cone> {
        Some(self)
    }
}

impl Cone {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn height(&self) -> f64 {
        self.height
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius
    }
    pub fn set_height(&mut self, height: f64) {
        self.height = height
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, height: f64) -> Cone {
        self::Cone {
            pos,
            dir,
            radius,
            height,
        }
    }
}
