use self::wireframe::Wireframe;
use self::aabb::Aabb;
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
pub mod aabb;
pub mod wireframe;
pub mod torusphere;

pub trait Shape: Debug + Sync + Send {
    fn distance(&self, vec: &Vec3) -> f64;
    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>>;
    fn projection(&self, hit: &Hit) -> Projection;
    fn norm(&self, hit_position: &Vec3, ray_dir: &Vec3) -> Vec3;
    fn pos(&self) -> &Vec3;
    fn shape_name(&self) -> String {
        if self.as_aabb().is_some() {
            return "AABB".to_string();
        } else if self.as_sphere().is_some() {
            return "Sphere".to_string();
        } else if self.as_triangle().is_some() {
            return "Triangle".to_string();
        } else if self.as_cone().is_some() {
            return "Cone".to_string();
        } else if self.as_cylinder().is_some() {
            return "Cylinder".to_string();
        } else if self.as_plane().is_some() {
            return "Plane".to_string();
        } else if self.as_rectangle().is_some() {
            return "Rectangle".to_string();
        } else if self.as_wireframe().is_some() {
            return "Wireframe".to_string();
        } else {
            return "Unknown".to_string();
        }
    }

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
    fn as_aabb(&self) -> Option<&Aabb> {
        None
    }
    fn as_wireframe(&self) -> Option<&Wireframe> {
        None
    }
    fn aabb(&self) -> Option<&Aabb> {
        None
    }
}
