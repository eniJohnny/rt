use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};

use super::HasShape;

pub struct Sphere {
    pos: Vec3,
    dir: Vec3
}

impl HasShape for Sphere {
    fn distance(&self) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, vector: &Ray) -> Option<Hit> {
        unimplemented!()
    }
    fn projection(&self, hit: &Hit) -> Option<(i32, i32)> {
        unimplemented!()
    }
}