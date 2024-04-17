use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use super::Shape;

#[derive(Debug)]
pub struct Plane {
    pos: Vec3,
    dir: Vec3,
}

impl Shape for Plane {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, vector: &Ray) -> Option<Vec<f64>> {
        unimplemented!()
    }
    fn projection(&self, hit: &Hit) -> (i32, i32) {
        unimplemented!()
    }
    fn norm(&self, _: &Vec3) -> Vec3 {
		self.dir.clone()
	}
}

impl Plane {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos }
    pub fn set_dir(&mut self, dir: Vec3) { self.dir = dir }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3) -> Plane{
        self::Plane { pos, dir }
    }

}