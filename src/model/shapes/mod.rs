use self::cone::Cone;
use self::cylinder::Cylinder;
use self::plane::Plane;
use self::sphere::Sphere;
use std::fmt::Debug;

use super::{
    materials::material::Projection, maths::{hit::Hit, ray::Ray, vec3::Vec3}, scene::{self, Scene}, Element
};

pub mod cone;
pub mod cylinder;
pub mod plane;
pub mod sphere;

pub trait Shape: Debug + Sync + Send {
    fn distance(&self, vec: &Vec3) -> f64;
    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>>;
    fn outer_intersect(&self, ray: &Ray, f: f64, displaced_factor: f64) -> Option<Vec<f64>>;
    fn intersect_displacement(&self, ray: &Ray, element: &Element, scene: &Scene) -> Option<Vec<f64>>;
    fn projection(&self, hit: &Hit) -> Projection;
    fn norm(&self, hit_position: &Vec3, ray_dir: &Vec3) -> Vec3;
    fn pos(&self) -> &Vec3;

    fn as_sphere(&self) -> Option<&Sphere> {
        None
    }
    fn as_plane(&self) -> Option<&Plane> {
        None
    }
    fn as_cylinder(&self) -> Option<&Cylinder> {
        None
    }
    fn as_cone(&self) -> Option<&Cone> {
        None
    }
}
