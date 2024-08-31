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
    aabb: super::aabb::Aabb,
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
    fn as_triangle_mut(&mut self) -> Option<&mut Triangle> { Some(self) }

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
            true, None, None));
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
            true, Some(-1.), Some(1.)));
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
            true, Some(-1.), Some(1.)));
        }
        category
    }
}

impl Triangle {
    // // Accessors
    pub fn get_a(&self) -> &Vec3 { &self.a }
    pub fn get_b(&self) -> &Vec3 { &self.b }
    pub fn get_c(&self) -> &Vec3 { &self.c }
    pub fn aabb(&self) -> &super::aabb::Aabb { &self.aabb }

    // Mutators
    pub fn set_a(&mut self, a: Vec3) {
        self.a = a;
        self.update_aabb();
    }

    pub fn set_b(&mut self, b: Vec3) {
        self.b = b;
        self.update_aabb();
    }

    pub fn set_c(&mut self, c: Vec3) {
        self.c = c;
        self.update_aabb();
    }

    pub fn set_aabb(&mut self, aabb: super::aabb::Aabb) {
        self.aabb = aabb;
    }

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
        let aabb = self::Triangle::compute_aabb(&a, &b, &c);
        Triangle {
            a,
            b,
            c,
            dir,
            plane,
            aabb,
        }
    }

    pub fn update_aabb(&mut self) {
        self.set_aabb(self::Triangle::compute_aabb(&self.a, &self.b, &self.c));
    }

    pub fn compute_aabb(a: &Vec3, b: &Vec3, c: &Vec3) -> super::aabb::Aabb {
        let x_min = a.x().min(*b.x()).min(*c.x());
        let x_max = a.x().max(*b.x()).max(*c.x());
        let y_min = a.y().min(*b.y()).min(*c.y());
        let y_max = a.y().max(*b.y()).max(*c.y());
        let z_min = a.z().min(*b.z()).min(*c.z());
        let z_max = a.z().max(*b.z()).max(*c.z());

        super::aabb::Aabb::new(x_min, x_max, y_min, y_max, z_min, z_max)
    }

}