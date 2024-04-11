use self::sphere::Sphere;
use self::plane::Plane;
use self::cylinder::Cylinder;
use self::cone::Cone;

use super::maths::{hit::Hit, ray::Ray, vec3::Vec3};

pub mod sphere;
pub mod plane;
pub mod cylinder;
pub mod cone;

pub trait Shape {
    fn distance(&self, vec : &Vec3) -> f64;
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
    fn projection(&self, hit: &Hit) -> (i32, i32);
}