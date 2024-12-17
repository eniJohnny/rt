use super::{cylinder::Cylinder, sphere::Sphere, ComposedShape};
use std::f64::consts::PI;
use crate::{error, model::{
    materials::{
        diffuse::Diffuse,
        material::Material,
        texture::{Texture, TextureType}
    },
    maths::vec3::Vec3,
    Element
}, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}};

#[derive(Debug)]
pub struct Nagone {
    pub pos: Vec3,
    pub dir: Vec3,
    pub radius: f64,
    pub angles: usize
}

impl ComposedShape for Nagone {
    fn as_nagone(&self) -> Option<&self::Nagone> {
        return Some(self);
    }
    fn as_nagone_mut(&mut self) -> Option<&mut self::Nagone> {
        return Some(self);
    }

    fn generate_elements(&self, material: Box<dyn Material + Send +Sync>) -> Vec<Element> {
        let mut elements: Vec<Element> = Vec::new();

        let dir_y = self.dir.normalize();
        let dir_x;

        if self.dir == Vec3::new(0.0, 1.0, 0.0) {
            dir_x = Vec3::new(1.0, 0.0, 0.0);
        } else {
            dir_x = Vec3::new(0.0, 1.0, 0.0);
        }

        let mut origins_dirs: Vec<Vec3> = Vec::new();
        let sphere_radius = self.radius / self.angles as f64 * 1.3;
        let cylinder_radius = 0.5 * sphere_radius;

        for i in 1..self.angles + 1 {
            let factor = (i * 2) as f64;
            origins_dirs.push((PI * factor / self.angles as f64).sin() * dir_y + (PI * factor / self.angles as f64).cos() * dir_x);
        }

        for i in 0..self.angles {
            let sphere = Sphere::new(self.pos + origins_dirs[i] * self.radius, dir_y, sphere_radius);
            elements.push(Element::new(Box::new(sphere), material.clone()));

            let next_i = (i + 1) % self.angles;
            let cylinder_dir = ((self.pos + origins_dirs[next_i] * self.radius) - (self.pos + origins_dirs[i] * self.radius)).normalize();
            let cylinder_height = ((self.pos + origins_dirs[next_i] * self.radius) - (self.pos + origins_dirs[i] * self.radius)).length();

            let cylinder = Cylinder::new(self.pos + origins_dirs[i] * self.radius, cylinder_dir, cylinder_radius, cylinder_height);
            elements.push(Element::new(Box::new(cylinder), material.clone()));
        }
        elements
    }

    fn get_ui(&self, element: &crate::model::ComposedElement, ui: &mut crate::ui::ui::UI, _scene: &std::sync::Arc<std::sync::RwLock<crate::model::scene::Scene>>) -> crate::ui::uielement::UIElement {
        let mut category = UIElement::new("Nagone", "nagone", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(nagone) = self.as_nagone() {
            let id = element.id();

            // pos
            category.add_element(get_vector_ui(nagone.pos.clone(), "Position", "pos", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.pos.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.pos.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.pos.set_z(value);
                    }
                }
            }),
            false, None, None));

            // dir
            category.add_element(get_vector_ui(nagone.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.dir.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.dir.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.dir.set_z(value);
                    }
                }
            }),
            false, None, None));

            // radius
            category.add_element(UIElement::new(
                "Radius",
                "radius", 
                ElemType::Property(Property::new(
                    Value::Float(nagone.radius), 
                    Box::new(move |_, value, scene, _: &mut UI| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                            if let Value::Float(value) = value {
                                nagone.set_radius(value);
                            }
                        }
                        scene.set_dirty(true);
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));

            // angles
            category.add_element(UIElement::new(
                "Angles",
                "angles", 
                ElemType::Property(Property::new(
                    Value::Unsigned(nagone.angles as u32), 
                    Box::new(move |_, value, scene, _: &mut UI| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                            if let Value::Unsigned(value) = value {
                                nagone.set_angles(value as usize);
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

impl Nagone {
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, angles: usize) -> Nagone {
        if angles < 3 {
            error("Nagone must have at least 3 angles");
        }

        Nagone {
            pos,
            dir,
            radius,
            angles
        }
    }

    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn angles(&self) -> usize { self.angles }

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
    pub fn set_angles(&mut self, angles: usize) {
        self.angles = angles;
    }
}