use self::cone::Cone;
use self::cylinder::Cylinder;
use self::plane::Plane;
use self::sphere::Sphere;
use crate::model::shapes::rectangle::Rectangle;

use std::fmt::Debug;
use crate::model::shapes::triangle::Triangle;

use super::{
    materials::material::Projection,
    maths::{hit::Hit, ray::Ray, vec3::Vec3},
};

pub mod cone;
pub mod cylinder;
pub mod plane;
pub mod sphere;
pub mod rectangle;
pub mod triangle;

pub trait Shape: Debug + Sync + Send {
    fn distance(&self, vec: &Vec3) -> f64;
    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>>;
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
    fn as_rectangle(&self) -> Option<&Rectangle> {
        None
    }
    fn as_triangle(&self) -> Option<&Triangle> {
        None
    }
}
