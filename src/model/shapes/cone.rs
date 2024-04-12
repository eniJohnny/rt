use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use super::Shape;

pub struct Cone {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
    height: f64
}

impl Shape for Cone {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, vector: &Ray) -> Option<Hit> {
        unimplemented!()
    }
    fn projection(&self, hit: &Hit) -> (i32, i32) {
        unimplemented!()
    }
}

impl Cone {
    // Accessors
    pub fn get_pos(&self) -> &Vec3 { &self.pos }
    pub fn get_dir(&self) -> &Vec3 { &self.dir }
    pub fn get_radius(&self) -> f64 { self.radius }
    pub fn get_height(&self) -> f64 { self.height }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos }
    pub fn set_dir(&mut self, dir: Vec3) { self.dir = dir }
    pub fn set_radius(&mut self, radius: f64) { self.radius = radius }
    pub fn set_height(&mut self, height: f64) { self.height = height }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, height: f64) -> Cone {
        self::Cone { pos, dir, radius, height }
    }

}