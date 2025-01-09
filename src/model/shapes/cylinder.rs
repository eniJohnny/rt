use super::aabb::Aabb;
use super::shape::Shape;
use core::panic;
use std::f64::consts::PI;
use std::sync::{Arc, RwLock};
use crate::{
    model::{
        materials::material::Projection,
        maths::{hit::Hit, ray::Ray, vec3::Vec3},
        scene::Scene,
        shapes::plane::Plane,
        element::Element
    },
    ui::{
        prefabs::vector_ui::get_vector_ui,
        ui::UI,
        uielement::{Category, UIElement},
        utils::misc::{ElemType, Property, Value}
    }
};

#[derive(Debug, Clone)]
pub struct Cylinder {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
    height: f64,
    plane: [Plane; 2],
    aabb: super::aabb::Aabb,
}

impl Shape for Cylinder {
    fn distance(&self, _vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        //d:    direction du rayon
        //co:   vecteur entre la postion du cylindre et le point d'origine du rayon
        //v:    vecteur directeur du cylindre
        //abc:  les coefficients
        let dv = r.get_dir().cross(&self.dir);
        let cov = (r.get_pos() - &self.pos).cross(&self.dir);
        let a = dv.dot(&dv);
        let b = cov.dot(&dv) * 2.0;
        let c = cov.dot(&cov) - (self.radius * self.radius);

        let mut delta = b * b - 4.0 * a * c;

        let mut t = Vec::new();

        if delta > 0.0 {
            delta = delta.sqrt();
            let (t1, t2) = ((-b - delta) / (2.0 * a), (-b + delta) / (2.0 * a));
            t.push(t1.min(t2));
            t.push(t1.max(t2));
        } else if delta == 0.0 {
            t.push(-b / (2.0 * a));
        }
        let mut plane_intersect = false;
        if let Some(t3) = self.plane[0].intersect(r) {
            plane_intersect = true;
            let t3 = t3[0];
            let t4 = self.plane[1]
                .intersect(r)
                .expect("The cylinder's planes should be parrallel to each other.")[0];
            t.push(t3.min(t4));
            t.push(t3.max(t4));
        }
        match t.len() {
            1 => {
                return Some(t);
            }
            2 => {
                if !plane_intersect {
                    return Some(t);
                } else {
                    return None;
                }
            }
            3 => {
                // On ne touche que la tranche du cylindre, on n'intersecte que si le t cylindre est entre les deux plans (inclusif)
                if t[0] >= t[1] && t[0] <= t[1] {
                    t.truncate(1);
                    return Some(t);
                }
            }
            4 => {
                // 99.9% des cas, le classico
                if !(t[2] > t[1] || t[3] < t[0]) {
                    t.sort_by(|a, b| {
                        a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
                    });
                    t.remove(0);
                    t.remove(2);
                    return Some(t);
                }
            }
            _ => panic!("Should never happen"),
        }
        None
    }

	fn outer_intersect(&self, r: &Ray, _displaced_factor: f64) -> Option<Vec<f64>> {
		self.intersect(r)
	}

    fn intersect_displacement(&self, ray: &Ray, _element: &Element, _scene: &Scene) -> Option<Vec<f64>> {
		self.intersect(ray)
	}

    fn projection(&self, hit: &Hit) -> Projection {
        let mut projection: Projection = Projection::default();

        let cam_hit = hit.pos() - &self.pos;
        let level = cam_hit.dot(&self.dir);
        let total_height = self.height + self.radius * 2.0;

        let constant_axis: Vec3;
        if self.dir == Vec3::new(0., 0., 1.) {
            constant_axis = Vec3::new(0., 1., 0.);
        } else {
            constant_axis = Vec3::new(0., 0., 1.);
        }

        let i = self.dir().cross(&constant_axis).normalize();
        let j = self.dir().cross(&i).normalize();
        let i_component: f64 = cam_hit.dot(&i);
        let j_component: f64 = cam_hit.dot(&j);
        let ij_hit: Vec3 = (i_component * &i + j_component * &j).normalize();

        projection.u = 0.5 + i_component.atan2(j_component) / (2. * PI);
        projection.i = (&ij_hit).cross(self.dir()).normalize();
        projection.k = hit.norm().clone();

        if level > -0.000001 && level < 0.000001 {
            // Bottom Cap
            projection.j = ij_hit;
            projection.v = (hit.pos() - &self.pos).length() / total_height;
        } else if level > self.height - 0.000001 && level < self.height + 0.000001 {
            // Top Cap
            projection.j = -ij_hit;
            projection.v = (total_height
                - (hit.pos() - &self.pos - &self.dir * &self.height).length())
                / total_height;
        } else {
            // Cylinder
            projection.j = self.dir().clone();
            projection.v = (level + self.radius) / total_height;
        }
        projection
    }

    fn norm(&self, hit: &Vec3) -> Vec3 {
        let pc = hit - &self.pos;
        let coef = pc.dot(&self.dir);
        let projection = &self.dir * coef;

        let norm;
        if coef > -0.000001 && coef < 0.000001 {
            norm = self.plane[0].norm(hit);
        } else if coef > self.height - 0.000001 && coef < self.height + 0.000001 {
            norm = self.plane[1].norm(hit);
        } else {
            norm = (pc - &projection).normalize();
        }
        return norm;
    }
    fn as_cylinder(&self) -> Option<&Cylinder> {
        Some(self)
    }
    fn as_cylinder_mut(&mut self) -> Option<&mut Cylinder> {
        Some(self)
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn aabb(&self) -> Option<&Aabb> {
        Some(&self.aabb)
    }

    fn get_ui(&self, element: &Element, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Cylinder", "cylinder", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(cylinder) = element.shape().as_cylinder() {
            let id = element.id().clone();
            category.add_element(get_vector_ui(cylinder.pos.clone(), "Position", "pos", &ui.uisettings_mut(), 
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cylinder) = elem.shape_mut().as_cylinder_mut() {
                        if let Value::Float(value) = value {
                            cylinder.pos.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cylinder) = elem.shape_mut().as_cylinder_mut() {
                        if let Value::Float(value) = value {
                            cylinder.pos.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cylinder) = elem.shape_mut().as_cylinder_mut() {
                        if let Value::Float(value) = value {
                            cylinder.pos.set_z(value);
                        }
                    }
                }),
                true, None, None));
            category.add_element(get_vector_ui(cylinder.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
                Box::new(move |_, value, scene, _ui| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cylinder) = elem.shape_mut().as_cylinder_mut() {
                        if let Value::Float(value) = value {
                            cylinder.dir.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cylinder) = elem.shape_mut().as_cylinder_mut() {
                        if let Value::Float(value) = value {
                            cylinder.dir.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cylinder) = elem.shape_mut().as_cylinder_mut() {
                        if let Value::Float(value) = value {
                            cylinder.dir.set_z(value);
                        }
                    }
                }),
                true, Some(-1.), Some(1.)));
            category.add_element(UIElement::new(
                "Radius",
                "radius", 
                ElemType::Property(Property::new(
                    Value::Float(cylinder.radius), 
                    Box::new(move |_, value, scene, _| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(cylinder) = elem.shape_mut().as_cylinder_mut() {
                            if let Value::Float(value) = value {
                                cylinder.set_radius(value);
                            }
                        }
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));

            category.add_element(UIElement::new(
                "Height",
                "height", 
                ElemType::Property(Property::new(
                    Value::Float(cylinder.height), 
                    Box::new(move |_, value, scene, _| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(cylinder) = elem.shape_mut().as_cylinder_mut() {
                            if let Value::Float(value) = value {
                                cylinder.set_height(value);
                            }
                        }
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));
        }

        category
    }
}

impl Cylinder {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn height(&self) -> f64 { self.height }
    pub fn aabb(&self) -> &super::aabb::Aabb { &self.aabb }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { 
        self.pos = pos;
        self.update_aabb();
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
        self.update_aabb();
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
        self.update_aabb();
    }
    pub fn set_height(&mut self, height: f64) {
        self.height = height;
        self.update_aabb();
    }
    pub fn set_aabb(&mut self, aabb: super::aabb::Aabb) {
        self.aabb = aabb;
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, height: f64) -> Cylinder {
        let plane1 = Plane::new(pos.clone(), -dir.clone());
        let plane2 = Plane::new(pos.clone() + dir.clone() * height, dir.clone());
        let aabb = Cylinder::compute_aabb(pos.clone(), dir.clone(), height, radius);
        self::Cylinder { pos, dir, radius, height, plane: [plane1, plane2], aabb }
    }

    fn update_aabb(&mut self) {
        self.aabb = Cylinder::compute_aabb(self.pos.clone(), self.dir.clone(), self.height, self.radius);
    }

    pub fn compute_aabb(pos:Vec3, dir: Vec3, height: f64, radius: f64) -> Aabb {
        let a = pos;
        let b = a + dir * height;
        let tmp = Vec3::new(radius, radius, radius);
        let min = a.min(b) - tmp;
        let max = a.max(b) + tmp;
        Aabb::new(*min.x(), *max.x(), *min.y(), *max.y(), *min.z(), *max.z())
    }
}