use super::{composed_shape::ComposedShape, sphere::Sphere, utils::get_cross_axis};
use std::f64::consts::PI;
use crate::{
    model::{
        materials::
            material::Material
        , maths::vec3::Vec3, composed_element::ComposedElement, element::Element
    },
    ui::{
        prefabs::vector_ui::get_vector_ui,
        ui::UI,
        uielement::{Category, UIElement},
        utils::misc::{ElemType, Property, Value}
    },
};

#[derive(Debug)]
pub struct Torusphere {
    pub pos: Vec3,
    pub dir: Vec3,
    pub radius: f64,
    pub steps: usize,
    pub sphere_radius: f64
}

impl ComposedShape for Torusphere {
    fn generate_elements(&self, material: Box<dyn Material + Send +Sync>) -> Vec<Element> {
        let mut sph_vec = vec![];
        let dir_y = self.dir.clone().normalize();
        let dir_x = get_cross_axis(&dir_y);

        for i in 1..self.steps + 1 {
            // let factor = (i as i32 - steps as i32 / 2) as f64 * 2.0;
            let factor = (i * 2) as f64;
            sph_vec.push((PI * factor / self.steps as f64).sin() * dir_y + (PI * factor / self.steps as f64).cos() * dir_x);
        }

        let mut elements = vec![];
        for i in 0..self.steps {
            let sphere = Sphere::new(self.pos + sph_vec[i] * self.radius, self.dir.normalize(), self.sphere_radius);
            let element = Element::new(Box::new(sphere), material.clone());
            elements.push(element);
        }
        elements
    }
    fn as_torusphere(&self) -> Option<&self::Torusphere> {
        return Some(self);
    }
    fn as_torusphere_mut(&mut self) -> Option<&mut self::Torusphere> {
        return Some(self);
    }

    fn get_ui(&self, element: &ComposedElement, ui: &mut UI, _scene: &std::sync::Arc<std::sync::RwLock<crate::model::scene::Scene>>) -> UIElement {
        let mut category = UIElement::new("Torusphere", "torusphere", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(torusphere) = self.as_torusphere() {
            let id = element.id();

            // pos
            category.add_element(get_vector_ui(torusphere.pos.clone(), "Position", "pos", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(torusphere) = elem.composed_shape_mut().as_torusphere_mut() {
                    if let Value::Float(value) = value {
                        torusphere.pos.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(torusphere) = elem.composed_shape_mut().as_torusphere_mut() {
                    if let Value::Float(value) = value {
                        torusphere.pos.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(torusphere) = elem.composed_shape_mut().as_torusphere_mut() {
                    if let Value::Float(value) = value {
                        torusphere.pos.set_z(value);
                    }
                }
            }),
            false, None, None));

            // dir
            category.add_element(get_vector_ui(torusphere.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(torusphere) = elem.composed_shape_mut().as_torusphere_mut() {
                    if let Value::Float(value) = value {
                        torusphere.dir.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(torusphere) = elem.composed_shape_mut().as_torusphere_mut() {
                    if let Value::Float(value) = value {
                        torusphere.dir.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(torusphere) = elem.composed_shape_mut().as_torusphere_mut() {
                    if let Value::Float(value) = value {
                        torusphere.dir.set_z(value);
                    }
                }
            }),
            false, None, None));

            // radius
            category.add_element(UIElement::new(
                "Radius",
                "radius", 
                ElemType::Property(Property::new(
                    Value::Float(torusphere.radius), 
                    Box::new(move |_, value, scene, _: &mut UI| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(torusphere) = elem.composed_shape_mut().as_torusphere_mut() {
                            if let Value::Float(value) = value {
                                torusphere.set_radius(value);
                            }
                        }
                        scene.set_dirty(true);
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));

            // steps
            category.add_element(UIElement::new(
                "Steps",
                "steps", 
                ElemType::Property(Property::new(
                    Value::Unsigned(torusphere.steps as u32), 
                    Box::new(move |_, value, scene, _: &mut UI| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(torusphere) = elem.composed_shape_mut().as_torusphere_mut() {
                            if let Value::Unsigned(value) = value {
                                torusphere.set_steps(value as usize);
                            }
                        }
                        scene.set_dirty(true);
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));

            // sphere_radius
            category.add_element(UIElement::new(
                "Sphere Radius",
                "sphere_radius", 
                ElemType::Property(Property::new(
                    Value::Float(torusphere.sphere_radius), 
                    Box::new(move |_, value, scene, _: &mut UI| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(torusphere) = elem.composed_shape_mut().as_torusphere_mut() {
                            if let Value::Float(value) = value {
                                torusphere.set_sphere_radius(value);
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

impl Torusphere {
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, steps: usize) -> Torusphere {
        let sphere_radius = 0.2 * radius;

        Torusphere {
            pos,
            dir,
            radius,
            steps,
            sphere_radius
        }
    }

    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn steps(&self) -> usize { self.steps }
    pub fn sphere_radius(&self) -> f64 { self.sphere_radius }

    // Setters
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
    pub fn set_steps(&mut self, steps: usize) {
        self.steps = steps;
    }
    pub fn set_sphere_radius(&mut self, sphere_radius: f64) {
        self.sphere_radius = sphere_radius;
    }
}