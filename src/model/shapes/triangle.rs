use std::sync::{Arc, RwLock};

use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::scene::Scene;
use crate::model::shapes::plane::Plane;
use crate::model::Element;
use crate::ui::prefabs::shape_ui::ShapeUI;
use crate::ui::prefabs::vector_ui::get_vector_ui;
use crate::ui::ui::UI;
use crate::ui::uielement::{Category, UIElement};
use crate::ui::utils::misc::{ElemType, Value};
use super::Shape;

#[derive(Debug)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    dir: Vec3,
    plane: Plane,
}

impl Shape for Triangle {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        let mut intersection: f64;
        match self.plane.intersect(r) {
            Some(intersections) => {
                intersection = intersections[0];
            },
            _ => {
                return None;
            }
        }

        let p = intersection * r.get_dir() + r.get_pos();
        if Triangle::inside_triangle(&p, &self.a, &self.b, &self.c, &self.dir) {
            return Some(Vec::from([intersection]));
        }
        None
    }

	fn outer_intersect(&self, r: &Ray, displaced_factor: f64) -> Option<Vec<f64>> {
		self.intersect(r)
	}

    fn intersect_displacement(&self, ray: &Ray, element: &Element, scene: &Scene) -> Option<Vec<f64>> {
		self.intersect(ray)
	}

    fn projection(&self, hit: &Hit) -> Projection {
        Projection::default()
    }
    fn norm(&self, hit: &Vec3, ray_dir: &Vec3) -> Vec3 {
        return self.plane.norm(hit, ray_dir);
    }
    fn pos(&self) -> &Vec3 { &self.a }
    fn as_triangle(&self) -> Option<&Triangle> { Some(self) }

    fn get_ui(&self, element: &Element, ui: &mut UI, scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Triangle", "triangle", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(triangle) = element.shape().as_triangle() {
            let id = element.id().clone();
            category.add_element(get_vector_ui(triangle.a.clone(), "Point A", "pA", &ui.uisettings_mut(), 
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.a.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.a.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.a.set_z(value);
                    }
                }
            }),
            true));
            category.add_element(get_vector_ui(triangle.b.clone(), "Point B", "pB", &ui.uisettings_mut(), 
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.b.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.b.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.b.set_z(value);
                    }
                }
            }),
            true));
            category.add_element(get_vector_ui(triangle.c.clone(), "Point C", "pC", &ui.uisettings_mut(), 
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.c.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.c.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.c.set_z(value);
                    }
                }
            }),
            true));
        }
        category
    }
}

impl Triangle {
    // // Accessors
    pub fn get_a(&self) -> &Vec3 { &self.a }
    pub fn get_b(&self) -> &Vec3 { &self.b }
    pub fn get_c(&self) -> &Vec3 { &self.c }

    // Mutators
    pub fn set_a(&mut self, a: Vec3) { self.a = a }
    pub fn set_b(&mut self, b: Vec3) { self.b = b }
    pub fn set_c(&mut self, c: Vec3) { self.c = c }

    pub fn inside_triangle(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3, dir: &Vec3) -> bool {
        let pa = (b - a).cross(&(p - a)).dot(&dir);
        let pb = (c - b).cross(&(p - b)).dot(&dir);
        let pc = (a - c).cross(&(p - c)).dot(&dir);
        if (pa >= 0. && pb >= 0. && pc >= 0.) || (pa <= 0. && pb <= 0. && pc <= 0.) {
            return true;
        }
        false
    }

    // Constructor
    pub fn new(a: Vec3, b: Vec3, c: Vec3 ) -> Triangle {

        let dir =(b.clone() - &a).cross(&(c.clone() - &a)).normalize();
        let plane = Plane::new(a.clone(), dir.clone());
        Triangle {
            a,
            b,
            c,
            dir,
            plane
        }
    }

}