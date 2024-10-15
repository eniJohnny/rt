use self::wireframe::Wireframe;
use self::aabb::Aabb;
use self::cone::Cone;
use self::cylinder::Cylinder;
use self::plane::Plane;
use self::sphere::Sphere;
use crate::{model::shapes::rectangle::Rectangle, ui::{ui::UI, uielement::UIElement}};

use std::{fmt::Debug, sync::{Arc, RwLock}};
use crate::model::shapes::triangle::Triangle;

use super::{
    materials::material::Projection,
    maths::{hit::Hit, ray::Ray, vec3::Vec3}, scene::Scene, Element,
};

pub mod cone;
pub mod cylinder;
pub mod plane;
pub mod sphere;
pub mod rectangle;
pub mod triangle;
pub mod aabb;
pub mod wireframe;

pub trait Shape: Debug + Sync + Send {
    fn distance(&self, vec: &Vec3) -> f64;
    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>>;
    fn outer_intersect(&self, ray: &Ray, displaced_factor: f64) -> Option<Vec<f64>>;
    fn intersect_displacement(&self, ray: &Ray, element: &Element, scene: &Scene) -> Option<Vec<f64>>;
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

    fn as_sphere_mut(&mut self) -> Option<&mut Sphere> {
        None
    }
    fn as_plane_mut(&mut self) -> Option<&mut Plane> {
        None
    }
    fn as_cylinder_mut(&mut self) -> Option<&mut Cylinder> {
        None
    }
    fn as_cone_mut(&mut self) -> Option<&mut Cone> {
        None
    }
    fn as_rectangle_mut(&mut self) -> Option<&mut Rectangle> {
        None
    }
    fn as_triangle_mut(&mut self) -> Option<&mut Triangle> {
        None
    }
    fn as_aabb_mut(&mut self) -> Option<&mut Aabb> {
        None
    }
    fn as_wireframe_mut(&mut self) -> Option<&mut Wireframe> {
        None
    }

    fn get_ui(&self, element: &Element, ui: &mut UI, scene: &Arc<RwLock<Scene>>) -> UIElement;
}
