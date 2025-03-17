use super::shape::Shape;
use std::{
    f64::consts::PI, sync::{Arc, RwLock}
};
use crate::{
    model::{
        element::Element, materials::material::Projection, maths::{hit::Hit, ray::Ray, vec3::Vec3}, scene::Scene
    },
    ui::{
        prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}
    }
};

#[derive(Debug)]
pub struct Cone {
    pos: Vec3,
    dir: Vec3,
    cos_powed: f64,
    angle: f64,
}

unsafe impl Send for Cone {}

impl Shape for Cone {
    fn distance(&self, _vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        //d:    direction du rayon
        //co:   vecteur entre la postion du cone et le point d'origine du rayon
        //v:    vecteur directeur du cone
        //abc:  les coefficients
        let dv = r.get_dir().dot(&self.dir);
        let co = r.get_pos() - &self.pos;
        let cov = co.dot(&self.dir);

        let a = dv.powi(2) - &self.cos_powed;
        let b = 2.0 * ((dv * cov) - co.dot(&r.get_dir()) * &self.cos_powed);
        let c = cov.powi(2) - co.dot(&(co)) * &self.cos_powed;

        let mut delta = b.powi(2) - 4.0 * a * c;

        if delta < 0.0 {
            return None;
        }
        delta = delta.sqrt();
        let mut intersections;
        if delta == 0. {
            intersections = Vec::from([(-b) / (2.0 * a)]);
        } else {
            intersections = Vec::from([(-b - delta) / (2.0 * a), (-b + delta) / (2.0 * a)]);
        }
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        return Some(intersections);
    }

    fn outer_intersect(&self, r: &Ray, _displaced_factor: f64) -> Option<Vec<f64>> {
        self.intersect(r)
    }

    fn intersect_displacement(&self, ray: &Ray, _element: &Element, _scene: &Scene) -> Option<Vec<f64>> {
        self.intersect(ray)
    }

    fn projection(&self, hit: &Hit) -> Projection {
        let mut projection: Projection = Projection::default();
        let constant_axis: Vec3;
        if *hit.norm() == Vec3::new(0., 0., 1.) || *hit.norm() == Vec3::new(0., 0., -1.) {
            constant_axis = Vec3::new(0., 1., 0.);
        } else {
            constant_axis = Vec3::new(0., 0., 1.);
        }
        projection.i = hit.norm().cross(&constant_axis).normalize();
        projection.j = hit.norm().cross(&projection.i).normalize();
        projection.k = hit.norm().clone();

        let point_to_hit = hit.pos() - &self.pos;
        projection.v = point_to_hit.dot(&self.dir).abs();
        
        let constant_axis: Vec3;
        if *self.dir() == Vec3::new(0., 0., 1.) || *self.dir() == Vec3::new(0., 0., -1.) {
            constant_axis = Vec3::new(0., 1., 0.);
        } else {
            constant_axis = Vec3::new(0., 0., 1.);
        }
        let i = self.dir().cross(&constant_axis).normalize();
        let j = self.dir().cross(&i).normalize();
        let i_component: f64 = hit.norm().dot(&i);
        let j_component: f64 = hit.norm().dot(&j);
        projection.u = (f64::atan2(i_component, j_component) + PI) / (2. * PI);
        projection
    }

    fn norm(&self, hit_position: &Vec3) -> Vec3 {
        let pc = hit_position - &self.pos;
        let u = self.dir.cross(&pc);
        let v = u.cross(&pc);
        v.normalize()
    }
    fn as_cone(&self) -> Option<&Cone> {
        Some(self)
    }
    fn as_cone_mut(&mut self) -> Option<&mut Cone> {
        Some(self)
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn get_ui(&self, element: &Element, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Cone", "cone", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(cone) = element.shape().as_cone() {
            let id = element.id().clone();
            category.add_element(get_vector_ui(cone.pos.clone(), "Position", "pos", &ui.uisettings_mut(), 
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cone) = elem.shape_mut().as_cone_mut() {
                        if let Value::Float(value) = value {
                            cone.pos.set_x(value);
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
                    if let Some(cone) = elem.shape_mut().as_cone_mut() {
                        if let Value::Float(value) = value {
                            cone.pos.set_y(value);
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
                    if let Some(cone) = elem.shape_mut().as_cone_mut() {
                        if let Value::Float(value) = value {
                            cone.pos.set_z(value);
                        }
                    }
                }),
                true, None, None));
            category.add_element(get_vector_ui(cone.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cone) = elem.shape_mut().as_cone_mut() {
                        if let Value::Float(value) = value {
                            cone.dir.set_x(value);
                            cone.set_dir(cone.dir.normalize());
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
                    if let Some(cone) = elem.shape_mut().as_cone_mut() {
                        if let Value::Float(value) = value {
                            cone.dir.set_y(value);
                            cone.set_dir(cone.dir.normalize());
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
                    if let Some(cone) = elem.shape_mut().as_cone_mut() {
                        if let Value::Float(value) = value {
                            cone.dir.set_z(value);
                            cone.set_dir(cone.dir.normalize());
                        }
                    }
                }),
                false, Some(-1.), Some(1.)));

            category.add_element(UIElement::new(
                "Angle",
                "angle", 
                ElemType::Property(Property::new(
                    Value::Float(cone.angle), 
                    Box::new(move |_, value, context, _| {
                        let scene = match context.active_scene {
                            Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                            None => return,
                        };
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(cone) = elem.shape_mut().as_cone_mut() {
                            if let Value::Float(value) = value {
                                cone.set_angle(value);
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

impl Cone {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir.normalize();
    }
    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle;
        self.cos_powed = (angle * PI / 360.).cos().powi(2);
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, angle: f64) -> Cone {
        let cos_powed = (angle * PI / 360.).cos().powi(2);
        self::Cone {
            pos,
            dir,
            cos_powed,
            angle
        }
    }
}