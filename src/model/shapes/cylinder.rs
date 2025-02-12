use super::{shape::Shape, utils::get_cross_axis};
use std::{f64::{consts::PI, EPSILON}, sync::{Arc, RwLock}};
use crate::{
    model::{
        materials::material::Projection,
        maths::{hit::Hit, ray::Ray, vec3::Vec3},
        scene::Scene,
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
}

impl Shape for Cylinder {
    fn distance(&self, _vec: &Vec3) -> f64 {
        unimplemented!()
    }

    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        let a = r.get_dir().dot(&r.get_dir()) - (r.get_dir().dot(&self.dir) * r.get_dir().dot(&self.dir));
        let b = 2. * (r.get_dir().dot(&(r.get_pos() - &self.pos)) - (r.get_dir().dot(&self.dir) * (r.get_pos() - &self.pos).dot(&self.dir)));
        let c = (r.get_pos() - &self.pos).dot(&(r.get_pos() - &self.pos)) - ((r.get_pos() - &self.pos).dot(&self.dir) * (r.get_pos() - &self.pos).dot(&self.dir)) - self.radius * self.radius;

        let delta = b * b - 4. * a * c;

        if delta < 0. {
            return None;
        }

        let t1 = (-b - delta.sqrt()) / (2. * a);
        let t2 = (-b + delta.sqrt()) / (2. * a);

        Some(vec![t1.min(t2), t1.max(t2)])
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

        let constant_axis = get_cross_axis(&self.dir);

        let i = self.dir().cross(&constant_axis).normalize();
        let j = self.dir().cross(&i).normalize();
        let i_component: f64 = cam_hit.dot(&i);
        let j_component: f64 = cam_hit.dot(&j);
        let ij_hit: Vec3 = (i_component * &i + j_component * &j).normalize();

        projection.u = 0.5 + i_component.atan2(j_component) / (2. * PI);
        projection.u = (projection.u * hit.element().material().u_scale() - hit.element().material().u_shift()).rem_euclid(1.);
        projection.i = (&ij_hit).cross(self.dir()).normalize();
        projection.k = hit.norm().clone();

        projection.j = self.dir().clone();
        projection.v = (level + self.radius) / (2. * self.radius * PI);
        projection.v = (projection.v * hit.element().material().v_scale() - hit.element().material().v_shift()).rem_euclid(1.);
        projection
    }

    fn norm(&self, hit: &Vec3) -> Vec3 {
        let mut norm = *hit - self.pos;
        norm -= self.dir * self.dir.dot(&norm);
        norm.normalize()
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

    fn get_ui(&self, element: &Element, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Cylinder", "cylinder", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(cylinder) = element.shape().as_cylinder() {
            let id = element.id().clone();
            category.add_element(get_vector_ui(cylinder.pos.clone(), "Position", "pos", &ui.uisettings_mut(), 
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cylinder) = elem.shape_mut().as_cylinder_mut() {
                        if let Value::Float(value) = value {
                            cylinder.pos.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cylinder) = elem.shape_mut().as_cylinder_mut() {
                        if let Value::Float(value) = value {
                            cylinder.pos.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
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
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cylinder) = elem.shape_mut().as_cylinder_mut() {
                        if let Value::Float(value) = value {
                            cylinder.dir.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cylinder) = elem.shape_mut().as_cylinder_mut() {
                        if let Value::Float(value) = value {
                            cylinder.dir.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
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
                    Box::new(move |_, value, context, _| {
                        let scene = match context.active_scene {
                            Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                            None => return,
                        };
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
        }

        category
    }
}

impl Cylinder {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn radius(&self) -> f64 { self.radius }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { 
        self.pos = pos;
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64) -> Cylinder {
        let dir = dir.normalize();
        self::Cylinder { pos, dir, radius }
    }
}