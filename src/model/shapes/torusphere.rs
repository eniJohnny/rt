use super::{sphere::Sphere, ComposedShape};
use std::f64::consts::PI;
use crate::{
    model::{
        materials::{
            diffuse::Diffuse,
            material::Material,
            texture::{Texture, TextureType}
        }, maths::vec3::Vec3, ComposedElement, Element
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
    pub sphere_radius: f64,
    pub sphere_color: Vec3,
    pub material: Box<dyn Material>,
    pub elements: Vec<Element>,
}

impl ComposedShape for Torusphere {
    fn material(&self) -> &dyn Material {
        return self.material.as_ref();
    }
    fn elements(&self) -> &Vec<Element> {
        return &self.elements();
    }
    fn elements_as_mut(&mut self) -> &mut Vec<Element> {
        return &mut self.elements;
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
                        elem.update();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(torusphere) = elem.composed_shape_mut().as_torusphere_mut() {
                    if let Value::Float(value) = value {
                        torusphere.pos.set_y(value);
                        elem.update();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(torusphere) = elem.composed_shape_mut().as_torusphere_mut() {
                    if let Value::Float(value) = value {
                        torusphere.pos.set_z(value);
                        elem.update();
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
                        let next_id = scene.read().unwrap().get_next_element_id();
                        let mut id_increment = 0;
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(torusphere) = elem.composed_shape_mut().as_torusphere_mut() {
                            if let Value::Unsigned(value) = value {
                                let m = torusphere.elements().len() as u32;
                                torusphere.set_steps(value as usize, next_id);
                                let n = torusphere.elements().len() as u32;
                                id_increment = next_id + n - m;
                            }
                        }
                        scene.set_next_element_id(id_increment);
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

    fn update(&mut self) {
        self.update(0);
    }
}

impl Torusphere {
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, steps: usize, color: Vec3) -> Torusphere {
        let mut sph_vec: Vec<Vec3> = Vec::new();
        let mut elements: Vec<Element> = Vec::new();
        let mut material: Box<Diffuse> = Diffuse::default();
        material.set_color(Texture::Value(color, TextureType::Color));
        material.set_opacity(Texture::Value(Vec3::from_value(1.0), TextureType::Float));

        let sphere_radius = 0.2 * radius;
        let dir_y = dir.normalize();
        let dir_x;
        if dir == Vec3::new(0.0, 1.0, 0.0) {
            dir_x =  Vec3::new(1.0, 0.0, 0.0);
        } else {
            dir_x =  Vec3::new(0.0, 1.0, 0.0);
        }

        for i in 1..steps + 1 {
            // let factor = (i as i32 - steps as i32 / 2) as f64 * 2.0;
            let factor = (i * 2) as f64;
            sph_vec.push((PI * factor / steps as f64).sin() * dir_y + (PI * factor / steps as f64).cos() * dir_x);
        }

        for i in 0..steps {
            let sphere = Sphere::new(pos + sph_vec[i] * radius, dir_y, sphere_radius);
            let element = Element::new(Box::new(sphere), material.clone());
            elements.push(element);
        }

        Torusphere {
            pos,
            dir,
            radius,
            steps,
            sphere_radius,
            sphere_color: color,
            material,
            elements: elements
        }
    }

    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn steps(&self) -> usize { self.steps }
    pub fn sphere_radius(&self) -> f64 { self.sphere_radius }
    pub fn color(&self) -> &Vec3 { &self.sphere_color }
    pub fn material(&self) -> &dyn Material { self.material.as_ref() }
    pub fn elements(&self) -> &Vec<Element> { &self.elements }

    // Setters
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.update(0);
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
        self.update(0);
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
        self.update(0);
    }
    pub fn set_color(&mut self, color: Vec3) {
        self.sphere_color = color;
        self.material.set_color(Texture::Value(color, TextureType::Color));
        self.update(0);
    }
    pub fn set_steps(&mut self, steps: usize, next_id: u32) {
        self.steps = steps;
        self.update(next_id);
    }
    pub fn set_sphere_radius(&mut self, sphere_radius: f64) {
        self.sphere_radius = sphere_radius;
        self.update(0);
    }

    // Methods
    pub fn update(&mut self, next_id: u32) {
        let mut next_id = next_id;
        let mut elem_ids: Vec<u32> = Vec::new();
        for elem in self.elements() {
            elem_ids.push(elem.id());
        }

        let pos = self.pos;
        let dir = self.dir;
        let radius = self.radius;
        let color = self.sphere_color;
        let steps = self.steps;

        *self = Torusphere::new(pos, dir, radius, steps, color);

        for (i, elem) in self.elements.iter_mut().enumerate() {
            if i < elem_ids.len() {
                elem.set_id(elem_ids[i]);
            } else {
                elem.set_id(next_id);
                next_id += 1;
            }
        }
    }
}