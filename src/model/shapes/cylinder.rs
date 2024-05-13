use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use super::Shape;

#[derive(Debug)]
pub struct Cylinder {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
    height: f64
}

impl Shape for Cylinder {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, vector: &Ray) -> Option<Vec<f64>> {
        unimplemented!()
    }
    fn projection(&self, hit: &Hit) -> (i32, i32) {
        unimplemented!()
    }
    fn norm(&self, hit_position: &Vec3, ray_dir: &Vec3) -> Vec3 {
        unimplemented!()
    }
    fn as_cylinder(&self) -> Option<&Cylinder> { Some(self) }
}

impl Cylinder {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn height(&self) -> f64 { self.height }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos }
    pub fn set_dir(&mut self, dir: Vec3) { self.dir = dir }
    pub fn set_radius(&mut self, radius: f64) { self.radius = radius }
    pub fn set_height(&mut self, height: f64) { self.height = height }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, height: f64) -> Cylinder {
        self::Cylinder { pos, dir, radius, height}
    }

}