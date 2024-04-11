use self::sphere::Sphere;
use self::plane::Plane;
use self::cylinder::Cylinder;
use self::cone::Cone;

use super::maths::{hit::Hit, ray::Ray};

pub mod sphere;
pub mod plane;
pub mod cylinder;
pub mod cone;

pub enum Shape{
    Sphere(Sphere),
    Plane(Plane),
    Cylinder(Cylinder),
    Cone(Cone)
}

pub trait HasShape {
    fn distance(&self) -> f64;
    fn intersect(&self, vector: &Ray) -> Option<Hit>;
    fn projection(&self, hit: &Hit) -> Option<(i32, i32)>;
}