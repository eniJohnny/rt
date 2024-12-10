use crate::ui::{ui::UI, uielement::UIElement};
use self::{wireframe::Wireframe, aabb::Aabb, cone::Cone, cylinder::Cylinder, plane::Plane, sphere::Sphere, rectangle::Rectangle, triangle::Triangle};
use std::{fmt::Debug, sync::{Arc, RwLock}};
use super::{
    materials::material::{Material, Projection}, maths::{hit::Hit, ray::Ray, vec3::Vec3}, scene::Scene, ComposedElement, Element
};

pub mod cone;
pub mod cylinder;
pub mod plane;
pub mod sphere;
pub mod rectangle;
pub mod triangle;
pub mod brick;
pub mod aabb;
pub mod wireframe;
pub mod torusphere;
pub mod helix;
pub mod nagone;
pub mod mobius;
pub mod cubehole;
pub mod ellipse;
pub mod cube;
pub mod any;
pub mod hyperboloid;

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
        } else if self.as_ellipse().is_some() {
            return "Ellipse".to_string();
        } else if self.as_cube().is_some() {
            return "Cube".to_string();
        } else if self.as_cubehole().is_some() {
            return "Cubehole".to_string();
        } else if self.as_hyperboloid().is_some() {
            return "Hyperboloid".to_string();
        } else if self.as_any().is_some() {
            return "Any".to_string();
        } else {
            return "Unknown".to_string();
        }
    }

    fn as_sphere(&self) -> Option<&Sphere> { None }
    fn as_plane(&self) -> Option<&Plane> { None }
    fn as_cylinder(&self) -> Option<&Cylinder> { None }
    fn as_cone(&self) -> Option<&Cone> { None }
    fn as_rectangle(&self) -> Option<&Rectangle> { None }
    fn as_triangle(&self) -> Option<&Triangle> { None }
    fn as_aabb(&self) -> Option<&Aabb> { None }
    fn as_wireframe(&self) -> Option<&Wireframe> { None }
    fn as_ellipse(&self) -> Option<&ellipse::Ellipse> { None }
    fn as_cube(&self) -> Option<&cube::Cube> { None }
    fn as_cubehole(&self) -> Option<&cubehole::Cubehole> { None }
    fn as_hyperboloid(&self) -> Option<&hyperboloid::Hyperboloid> { None }
    fn as_any(&self) -> Option<&any::Any> { None }
    fn aabb(&self) -> Option<&Aabb> { None }

    fn as_sphere_mut(&mut self) -> Option<&mut Sphere> { None }
    fn as_plane_mut(&mut self) -> Option<&mut Plane> { None }
    fn as_cylinder_mut(&mut self) -> Option<&mut Cylinder> { None }
    fn as_cone_mut(&mut self) -> Option<&mut Cone> { None }
    fn as_rectangle_mut(&mut self) -> Option<&mut Rectangle> { None }
    fn as_triangle_mut(&mut self) -> Option<&mut Triangle> { None }
    fn as_aabb_mut(&mut self) -> Option<&mut Aabb> { None }
    fn as_wireframe_mut(&mut self) -> Option<&mut Wireframe> { None }
    fn as_ellipse_mut(&mut self) -> Option<&mut ellipse::Ellipse> { None }
    fn as_cube_mut(&mut self) -> Option<&mut cube::Cube> { None }
    fn as_cubehole_mut(&mut self) -> Option<&mut cubehole::Cubehole> { None }
    fn as_hyperboloid_mut(&mut self) -> Option<&mut hyperboloid::Hyperboloid> { None }
    fn as_any_mut(&mut self) -> Option<&mut any::Any> { None }

    fn get_ui(&self, element: &Element, ui: &mut UI, scene: &Arc<RwLock<Scene>>) -> UIElement;
}

pub trait ComposedShape: Debug + Sync + Send {
    fn material(&self) -> &dyn Material;
    fn elements(&self) -> &Vec<Element>;
    fn elements_as_mut(&mut self) -> &mut Vec<Element>;

    fn as_torusphere(&self) -> Option<&torusphere::Torusphere> { None }
    fn as_helix(&self) -> Option<&helix::Helix> { None }
    fn as_brick(&self) -> Option<&brick::Brick> { None }
    fn as_nagone(&self) -> Option<&nagone::Nagone> { None }
    fn as_mobius(&self) -> Option<&mobius::Mobius> { None }

    fn as_torusphere_mut(&mut self) -> Option<&mut torusphere::Torusphere> { None }
    fn as_helix_mut(&mut self) -> Option<&mut helix::Helix> { None }
    fn as_brick_mut(&mut self) -> Option<&mut brick::Brick> { None }
    fn as_nagone_mut(&mut self) -> Option<&mut nagone::Nagone> { None }
    fn as_mobius_mut(&mut self) -> Option<&mut mobius::Mobius> { None }

    fn get_ui(&self, element: &ComposedElement, ui: &mut UI, scene: &Arc<RwLock<Scene>>) -> UIElement;
    fn update(&mut self);
}