use std::sync::{Arc, RwLock};

use super::{shape::Shape, utils::get_cross_axis};
use nalgebra::Matrix3;
use crate::{model::{
    element::Element, materials::material::Projection, maths::{hit::Hit, ray::Ray, vec3::Vec3}, scene::Scene
}, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}};

#[derive(Debug)]
pub struct Ellipse {
    pos: Vec3,
    dir: Vec3,
    major_axis: Vec3,
    major_half_len: f64,
    minor_axis: Vec3,
    minor_half_len: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
    plane: super::plane::Plane,
    aabb: super::aabb::Aabb,
}

impl Shape for Ellipse {
    fn distance(&self, _vec: &Vec3) -> f64 {
        unimplemented!()
    }

    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        let t = self.plane.intersect(r);
        if t.is_none() {
            return None;
        }

        let mut t_array: Vec<f64> = Vec::new();

        for t in t.unwrap() {
            if t > 0. && self.is_inside(r.get_pos() + r.get_dir() * t) {
                t_array.push(t);
            }
        }

        match t_array.is_empty() {
            false => {
                t_array.sort_by(|a, b| a.partial_cmp(b).unwrap());
                Some(t_array)
            }
            true => None
        }
    }

    fn projection(&self, hit: &Hit) -> Projection {
        self.plane.projection(hit)
    }

    fn norm(&self, hit_position: &Vec3) -> Vec3 {
        self.plane.norm(hit_position)
    }

    fn as_ellipse(&self) -> Option<&Ellipse> {
        Some(self)
    }

    fn as_ellipse_mut(&mut self) -> Option<&mut Ellipse> {
        Some(self)
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn aabb(&self) -> Option<&super::aabb::Aabb> {
        Some(&self.aabb)
    }

    fn outer_intersect(&self, ray: &Ray, _displaced_factor: f64) -> Option<Vec<f64>> {
        self.intersect(ray)
    }

    fn intersect_displacement(&self, ray: &Ray, _element: &Element, _scene: &Scene) -> Option<Vec<f64>> {
        self.intersect(ray)
    }

    fn get_ui(&self, element: &Element, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Ellipse", "ellipse", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(ellipse) = element.shape().as_ellipse() {
            let id = element.id().clone();
            category.add_element(get_vector_ui(ellipse.pos.clone(), "Position", "pos", &ui.uisettings_mut(), 
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                        if let Value::Float(value) = value {
                            ellipse.pos.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                        if let Value::Float(value) = value {
                            ellipse.pos.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                        if let Value::Float(value) = value {
                            ellipse.pos.set_z(value);
                        }
                    }
                }),
                false, None, None));
            category.add_element(get_vector_ui(ellipse.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
                Box::new(move |_, value, scene, _ui| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                        if let Value::Float(value) = value {
                            ellipse.dir.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                        if let Value::Float(value) = value {
                            ellipse.dir.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, ui| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                        if let Value::Float(value) = value {
                            ellipse.dir.set_z(value);
                            ellipse.set_dir(ellipse.dir.normalize());
                            ui.set_dirty();
                        }
                    }
                }),
                false, Some(-1.), Some(1.)));

                category.add_element(UIElement::new(
                    "Major Half Length",
                    "major_half_len", 
                    ElemType::Property(Property::new(
                        Value::Float(ellipse.major_half_len), 
                        Box::new(move |_, value, scene, _: &mut UI| {
                            let mut scene = scene.write().unwrap();
                            let elem = scene.element_mut_by_id(id.clone()).unwrap();
                            if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                                if let Value::Float(value) = value {
                                    ellipse.set_major_half_len(value);
                                }
                            }
                            scene.set_dirty(true);
                        }),
                        Box::new(|_, _, _| Ok(())),
                        ui.uisettings())),
                    ui.uisettings()));

                    category.add_element(UIElement::new(
                        "Minor Half Length",
                        "minor_half_len", 
                        ElemType::Property(Property::new(
                            Value::Float(ellipse.minor_half_len), 
                            Box::new(move |_, value, scene, _: &mut UI| {
                                let mut scene = scene.write().unwrap();
                                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                                if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                                    if let Value::Float(value) = value {
                                        ellipse.set_minor_half_len(value);
                                    }
                                }
                                scene.set_dirty(true);
                            }),
                            Box::new(|_, _, _| Ok(())),
                            ui.uisettings())),
                        ui.uisettings()));
        }
        category
    }
}

impl Ellipse {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn major_axis(&self) -> &Vec3 {
        &self.major_axis
    }
    pub fn major_half_len(&self) -> f64 {
        self.major_half_len
    }
    pub fn minor_axis(&self) -> &Vec3 {
        &self.minor_axis
    }
    pub fn minor_half_len(&self) -> f64 {
        self.minor_half_len
    }
    pub fn alpha(&self) -> f64 {
        self.alpha
    }
    pub fn beta(&self) -> f64 {
        self.beta
    }
    pub fn gamma(&self) -> f64 {
        self.gamma
    }
    pub fn aabb(&self) -> &super::aabb::Aabb {
        &self.aabb
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        // self.update_ellipse();
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
        // self.update_ellipse();
    }
    pub fn set_major_axis(&mut self, major_axis: Vec3) {
        self.major_axis = major_axis
    }
    pub fn set_major_half_len(&mut self, major_half_len: f64) {
        self.major_half_len = major_half_len;
        // self.update_ellipse();
    }
    pub fn set_minor_axis(&mut self, minor_axis: Vec3) {
        self.minor_axis = minor_axis
    }
    pub fn set_minor_half_len(&mut self, minor_half_len: f64) {
        self.minor_half_len = minor_half_len;
        self.update();
    }
    pub fn set_alpha(&mut self, alpha: f64) {
        self.alpha = alpha
    }
    pub fn set_beta(&mut self, beta: f64) {
        self.beta = beta
    }
    pub fn set_gamma(&mut self, gamma: f64) {
        self.gamma = gamma
    }
    pub fn set_aabb(&mut self, aabb: super::aabb::Aabb) {
        self.aabb = aabb
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, u: f64, v: f64) -> Ellipse {
        let plane = super::plane::Plane::new(pos.clone(), dir.clone());
        let (major_half_len, minor_half_len) = if u > v { (u, v) } else { (v, u) };
        let aabb = Ellipse::compute_aabb(&pos, &dir, minor_half_len, major_half_len);
        let axis = get_cross_axis(&dir);

        let major_axis = dir.cross(&axis).normalize();
        let minor_axis = major_axis.cross(&dir).normalize();

        let angles = Ellipse::get_angles(&dir);
        self::Ellipse { pos, dir, major_axis, major_half_len, minor_axis, minor_half_len, alpha: *angles.x(), beta: *angles.y(), gamma: *angles.z(), plane, aabb }
    }

    // Methods
    pub fn clone(&self) -> Ellipse {
        let pos = Vec3::new(*self.pos.x(), *self.pos.y(), *self.pos.z());
        let dir = Vec3::new(*self.dir.x(), *self.dir.y(), *self.dir.z());
        let plane = super::plane::Plane::new(pos.clone(), dir.clone());

        self::Ellipse {
            pos,
            dir,
            major_axis: self.major_axis.clone(),
            major_half_len: self.major_half_len,
            minor_axis: self.minor_axis.clone(),
            minor_half_len: self.minor_half_len,
            alpha: self.alpha,
            beta: self.beta,
            gamma: self.gamma,
            plane,
            aabb: self.aabb.clone(),
        }
    }

    fn update(&mut self) {
        let pos = self.pos.clone();
        let dir = self.dir.clone();
        let (major_half_len, minor_half_len) = if self.major_half_len > self.minor_half_len { (self.major_half_len, self.minor_half_len) } else { (self.minor_half_len, self.major_half_len) };
        
        let plane = super::plane::Plane::new(pos.clone(), dir.clone());
        let aabb = Ellipse::compute_aabb(&pos, &dir, minor_half_len, major_half_len);
        let axis = get_cross_axis(&dir);

        let major_axis = dir.cross(&axis).normalize();
        let minor_axis = major_axis.cross(&dir).normalize();

        let angles = Ellipse::get_angles(&dir);
        *self = self::Ellipse { pos, dir, major_axis, major_half_len, minor_axis, minor_half_len, alpha: *angles.x(), beta: *angles.y(), gamma: *angles.z(), plane, aabb };
    }

    pub fn compute_aabb(pos: &Vec3, dir: &Vec3, minor_half_len: f64, major_half_len: f64) -> super::aabb::Aabb {
        let axis = get_cross_axis(dir);
        let major_axis = dir.cross(&axis).normalize();
        let minor_axis = major_axis.cross(&dir).normalize();

        let top = pos + major_axis * major_half_len;
        let right = pos + minor_axis * minor_half_len;
        let bottom = pos - major_axis * major_half_len;
        let left = pos - minor_axis * minor_half_len;

        let min = get_min(&top, &right, &bottom, &left);
        let max = get_max(&top, &right, &bottom, &left);

        super::aabb::Aabb::new(*min.x(), *max.x(), *min.y(), *max.y(), *min.z(), *max.z())
    }

    pub fn get_angles(dir: &Vec3) -> Vec3 {
        let axis;

        if dir == &Vec3::new(0.0, 1.0, 0.0) || dir == &Vec3::new(0.0, -1.0, 0.0) {
            axis = Vec3::new(1.0, 0.0, 0.0);
        } else {
            axis = Vec3::new(0.0, 1.0, 0.0);
        }

        let a_unit = dir.cross(&axis).normalize();
        let b_unit = a_unit.cross(&dir).normalize();
        let gamma = a_unit.y().atan2(*a_unit.x());
        let gamma_proj = (a_unit.x().powi(2) + a_unit.y().powi(2)).sqrt();
        let beta = a_unit.z().atan2(gamma_proj);
        let normal = a_unit.cross(&b_unit).normalize();
        let alpha = normal.y().atan2(*normal.z());

        Vec3::new(alpha, beta, gamma)
    }

    pub fn is_inside(&self, point: Vec3) -> bool {
        let rotation_matrix = rotation_z(self.gamma) * rotation_y(self.beta) * rotation_x(self.alpha);
        let inverse_rotation = rotation_matrix.try_inverse().unwrap();
        let projected_point = matrix3_vec3_mult(inverse_rotation, point - self.pos);
        let x = projected_point.x();
        let y = projected_point.y();

        (x.powi(2) / self.major_half_len.powi(2)) + (y.powi(2) / self.minor_half_len.powi(2)) <= 1.0
    }

}

fn get_min (v1: &Vec3, v2: &Vec3, v3: &Vec3, v4: &Vec3) -> Vec3 {
    let x_min = v1.x().min(v2.x().min(v3.x().min(*v4.x())));
    let y_min = v1.y().min(v2.y().min(v3.y().min(*v4.y())));
    let z_min = v1.z().min(v2.z().min(v3.z().min(*v4.z())));

    Vec3::new(x_min, y_min, z_min)
}

fn get_max (v1: &Vec3, v2: &Vec3, v3: &Vec3, v4: &Vec3) -> Vec3 {
    let x_max = v1.x().max(v2.x().max(v3.x().max(*v4.x())));
    let y_max = v1.y().max(v2.y().max(v3.y().max(*v4.y())));
    let z_max = v1.z().max(v2.z().max(v3.z().max(*v4.z())));

    Vec3::new(x_max, y_max, z_max)
}

fn rotation_x (angle: f64) -> Matrix3<f64> {
    Matrix3::new(
        1.0, 0.0, 0.0,
        0.0, angle.cos(), -angle.sin(),
        0.0, angle.sin(), angle.cos()
    )
}

fn rotation_y (angle: f64) -> Matrix3<f64> {
    Matrix3::new(
        angle.cos(), 0.0, angle.sin(),
        0.0, 1.0, 0.0,
        -angle.sin(), 0.0, angle.cos()
    )
}

fn rotation_z (angle: f64) -> Matrix3<f64> {
    Matrix3::new(
        angle.cos(), -angle.sin(), 0.0,
        angle.sin(), angle.cos(), 0.0,
        0.0, 0.0, 1.0
    )
}

fn matrix3_vec3_mult(rotation: Matrix3<f64>, vec: Vec3) -> Vec3 {
    Vec3::new(
        rotation[(0, 0)] * vec.x() + rotation[(0, 1)] * vec.y() + rotation[(0, 2)] * vec.z(),
        rotation[(1, 0)] * vec.x() + rotation[(1, 1)] * vec.y() + rotation[(1, 2)] * vec.z(),
        rotation[(2, 0)] * vec.x() + rotation[(2, 1)] * vec.y() + rotation[(2, 2)] * vec.z()
    )
}
