use std::fmt::Debug;
use self::sphere::Sphere;
use self::plane::Plane;
use self::cylinder::Cylinder;
use self::cone::Cone;

use super::maths::{hit::Hit, ray::Ray, vec3::Vec3};

pub mod sphere;
pub mod plane;
pub mod cylinder;
pub mod cone;

pub trait Shape: Debug + Sync {
    fn distance(&self, vec : &Vec3) -> f64;
    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>>;
    fn projection(&self, hit: &Hit) -> (i32, i32);
	fn norm(&self, hit_position: &Vec3) -> Vec3;

    fn as_sphere(&self) -> Option<&Sphere> { None }
    fn as_plane(&self) -> Option<&Plane> { None }
    fn as_cylinder(&self) -> Option<&Cylinder> { None }
    fn as_cone(&self) -> Option<&Cone> { None }
}
