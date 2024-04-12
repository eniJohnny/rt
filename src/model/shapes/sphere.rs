use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};

use super::Shape;

#[derive(Debug)]
pub struct Sphere {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
}

impl Shape for Sphere {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }
    
    fn intersect(&self, r: &Ray) -> Option<f64> {
        // intersection rayon/sphere
        let dist = &self.pos - r.get_pos();
        let dot_product = r.get_dir() * &dist;
        let discriminant = &dot_product *&dot_product - &dist * &dist + &self.radius * &self.radius;
        if (discriminant < 0.0) {
            return None;
        }
        let intersection1 = &dot_product - &discriminant.sqrt();
        let intersection2 = &dot_product + &discriminant.sqrt();
        if (intersection1 > 0.1) {
            return Some(intersection1);
        }
        if (intersection2 > 0.1) {
            return Some(intersection2);
        }
        return None;

    }

    fn projection(&self, hit: &Hit) -> (i32, i32) {
        unimplemented!()
    }
}

impl Sphere {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn radius(&self) -> f64 { self.radius }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos }
    pub fn set_dir(&mut self, dir: Vec3) { self.dir = dir }
    pub fn set_radius(&mut self, radius: f64) { self.radius = radius }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64) -> Sphere{
        self::Sphere { pos, dir, radius }
    }

}