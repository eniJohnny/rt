use self::sphere::Sphere;

use super::maths::{hit::Hit, ray::Ray};

pub mod sphere;

pub enum Shape{
    Sphere(Sphere)
}

pub trait HasShape {
    fn distance(&self) -> f64;
    fn intersect(&self, vector: &Ray) -> Option<Hit>;
    fn projection(&self, hit: &Hit) -> Option<(i32, i32)>;
}