use std::{f64::EPSILON, sync::{Arc, RwLock}};

use super::{shape::Shape, utils::{get_min_max_multiple_vec3, get_u_v_from_normal}};

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
                Box::new(move |_, value, context, _| {
                    if let Some(scene) = context.get_active_scene() {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                            if let Value::Float(value) = value {
                                ellipse.pos.set_x(value);
                            }
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    if let Some(scene) = context.get_active_scene() {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                            if let Value::Float(value) = value {
                                ellipse.pos.set_y(value);
                            }
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    if let Some(scene) = context.get_active_scene() {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                            if let Value::Float(value) = value {
                                ellipse.pos.set_z(value);
                            }
                        }
                    }
                }),
                false, None, None));
            category.add_element(get_vector_ui(ellipse.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
                Box::new(move |_, value, context, _ui| {
                    if let Some(scene) = context.get_active_scene() {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                            if let Value::Float(value) = value {
                                ellipse.dir.set_x(value);
                            }
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    if let Some(scene) = context.get_active_scene() {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                            if let Value::Float(value) = value {
                                ellipse.dir.set_y(value);
                            }
                        }
                    }
                }),
                Box::new(move |_, value, context, ui| {
                    if let Some(scene) = context.get_active_scene() {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                            if let Value::Float(value) = value {
                                ellipse.dir.set_z(value);
                                ellipse.set_dir(ellipse.dir.normalize());
                                ui.set_dirty();
                            }
                        }
                    }
                }),
                false, Some(-1.), Some(1.)));

                category.add_element(UIElement::new(
                    "Major Half Length",
                    "major_half_len", 
                    ElemType::Property(Property::new(
                        Value::Float(ellipse.major_half_len), 
                        Box::new(move |_, value, context, _: &mut UI| {
                            if let Some(scene) = context.get_active_scene() {
                                let mut scene = scene.write().unwrap();
                                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                                if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                                    if let Value::Float(value) = value {
                                        ellipse.set_major_half_len(value);
                                    }
                                }
                                scene.set_dirty(true);
                            }
                        }),
                        Box::new(|_, _, _| Ok(())),
                        ui.uisettings())),
                    ui.uisettings()));

                    category.add_element(UIElement::new(
                        "Minor Half Length",
                        "minor_half_len", 
                        ElemType::Property(Property::new(
                            Value::Float(ellipse.minor_half_len), 
                            Box::new(move |_, value, context, _: &mut UI| {
                                if let Some(scene) = context.get_active_scene() {
                                    let mut scene = scene.write().unwrap();
                                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                                    if let Some(ellipse) = elem.shape_mut().as_ellipse_mut() {
                                        if let Value::Float(value) = value {
                                            ellipse.set_minor_half_len(value);
                                        }
                                    }
                                    scene.set_dirty(true);
                                }
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
    pub fn aabb(&self) -> &super::aabb::Aabb {
        &self.aabb
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.update();
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
        self.update();
    }
    pub fn set_major_axis(&mut self, major_axis: Vec3) {
        self.major_axis = major_axis;
    }
    pub fn set_major_half_len(&mut self, major_half_len: f64) {
        self.major_half_len = major_half_len;
        self.update();
    }
    pub fn set_minor_axis(&mut self, minor_axis: Vec3) {
        self.minor_axis = minor_axis;
    }
    pub fn set_minor_half_len(&mut self, minor_half_len: f64) {
        self.minor_half_len = minor_half_len;
        self.update();
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, u: f64, v: f64) -> Ellipse {
        let plane = super::plane::Plane::new(pos.clone(), dir.clone());

        let (major_axis, minor_axis) = get_u_v_from_normal(&dir);
        let (major_half_len, minor_half_len) = if u > v { (u, v) } else { (v, u) };

        let aabb = Ellipse::compute_aabb(&pos, &dir, minor_half_len, major_half_len);

        self::Ellipse { pos, dir, major_axis, major_half_len, minor_axis, minor_half_len, plane, aabb }
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
            plane,
            aabb: self.aabb.clone(),
        }
    }

    fn update(&mut self) {
        let pos = self.pos.clone();
        let dir = self.dir.clone();
        let plane = super::plane::Plane::new(pos.clone(), dir.clone());

        let (major_half_len, minor_half_len) = if self.major_half_len > self.minor_half_len { (self.major_half_len, self.minor_half_len) } else { (self.minor_half_len, self.major_half_len) };
        let (major_axis, minor_axis) = get_u_v_from_normal(&dir);

        let aabb = Ellipse::compute_aabb(&pos, &dir, minor_half_len, major_half_len);

        *self = self::Ellipse { pos, dir, major_axis, major_half_len, minor_axis, minor_half_len, plane, aabb };
    }

    pub fn compute_aabb(pos: &Vec3, dir: &Vec3, minor_half_len: f64, major_half_len: f64) -> super::aabb::Aabb {
        let (major_axis, minor_axis) = get_u_v_from_normal(&dir);

        let a = major_axis * major_half_len;
        let b = minor_axis * minor_half_len;

        let apexes = vec![
            pos + a + b,
            pos - a + b,
            pos - a - b,
            pos + a - b
        ];

        let (min, max) = get_min_max_multiple_vec3(&apexes);
        super::aabb::Aabb::from_min_max(min, max)
    }

    pub fn is_inside(&self, point: Vec3) -> bool {
        let relative_position = point - self.pos;

        let x = relative_position.dot(&self.major_axis);
        let y = relative_position.dot(&self.minor_axis);

        x.powi(2) / self.major_half_len.powi(2) + y.powi(2) / self.minor_half_len.powi(2) - 1. <= EPSILON
    }

}
