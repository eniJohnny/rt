use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};

use super::Shape;

pub struct Sphere {
    pos: Vec3,
    dir: Vec3
}

impl Shape for Sphere {
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