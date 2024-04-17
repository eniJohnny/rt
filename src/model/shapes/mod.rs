use std::fmt::Debug;
use super::maths::{hit::Hit, ray::Ray, vec3::Vec3};

pub mod sphere;
pub mod plane;
pub mod cylinder;
pub mod cone;

pub trait Shape: Debug {
    fn distance(&self, vec : &Vec3) -> f64;
    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>>;
    fn projection(&self, hit: &Hit) -> (i32, i32);
	fn norm(&self, hit_position: &Vec3) -> Vec3;
}