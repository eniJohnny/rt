use super::{composed_shape::ComposedShape, cylinder::Cylinder, sphere::Sphere, utils::get_cross_axis};
use std::{f64::consts::PI, sync::{Arc, RwLock}};
use crate::{model::{
    composed_element::ComposedElement, element::Element, materials::{
        material::Material, texture::Texture
    }, maths::vec3::Vec3, scene::Scene
}, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}};

#[derive(Debug)]
pub struct Helix {
    pub pos: Vec3,
    pub dir: Vec3,
    pub height: f64
}

impl ComposedShape for Helix {
    fn as_helix(&self) -> Option<&self::Helix> {
        return Some(self);
    }
    fn as_helix_mut(&mut self) -> Option<&mut self::Helix> {
        return Some(self);
    }

    fn generate_elements(&self, material: Box<dyn Material + Send +Sync>) -> Vec<Element> {
        let mut elements: Vec<Element> = Vec::new();
        let mut sphere_material = material.clone();
        
        let link_color = material.color().clone();
        let sphere_color = match &link_color {
            Texture::Texture(_, _) => link_color,
            Texture::Value(value, texture_type) => Texture::Value(Vec3::from_value(1.) - value, texture_type.clone())
        };
        sphere_material.set_color(sphere_color);

        // Elements
        let steps = 20;
        let link_length = 0.3 * self.height;
        let link_radius = 0.25 * self.height / steps as f64;
        let sphere_radius = 2.0 * link_radius;

        let cross_vector = get_cross_axis(&self.dir);

        let rotation_ratio = 2.0 * PI / steps as f64;

        for i in 1..steps + 1 {
            let current_dir = cross_vector.rotate_from_axis_angle(i as f64 * rotation_ratio, &self.dir);
            let mut origin = self.pos - current_dir * link_length / 2.0;
            origin = origin + self.dir * self.height / steps as f64 * i as f64;

            let link = Cylinder::new(origin, current_dir, link_radius, link_length);
            let sphere1 = Sphere::new(origin, current_dir, sphere_radius);
            let sphere2 = Sphere::new(origin + current_dir * link_length, current_dir, sphere_radius);

            let link_element = Element::new(Box::new(link), material.clone());
            let sphere1_element = Element::new(Box::new(sphere1), sphere_material.clone());
            let sphere2_element = Element::new(Box::new(sphere2), sphere_material.clone());

            elements.push(link_element);
            elements.push(sphere1_element);
            elements.push(sphere2_element);
        }
        elements
    }

    fn get_ui(&self, element: &ComposedElement, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Helix", "helix", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(helix) = self.as_helix() {
            let id = element.id();

            // pos
            category.add_element(get_vector_ui(helix.pos.clone(), "Position", "pos", &ui.uisettings_mut(),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(helix) = elem.composed_shape_mut().as_helix_mut() {
                    if let Value::Float(value) = value {
                        helix.pos.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(helix) = elem.composed_shape_mut().as_helix_mut() {
                    if let Value::Float(value) = value {
                        helix.pos.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(helix) = elem.composed_shape_mut().as_helix_mut() {
                    if let Value::Float(value) = value {
                        helix.pos.set_z(value);
                    }
                }
            }),
            false, None, None));

            // dir
            category.add_element(get_vector_ui(helix.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(helix) = elem.composed_shape_mut().as_helix_mut() {
                    if let Value::Float(value) = value {
                        helix.dir.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(helix) = elem.composed_shape_mut().as_helix_mut() {
                    if let Value::Float(value) = value {
                        helix.dir.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(helix) = elem.composed_shape_mut().as_helix_mut() {
                    if let Value::Float(value) = value {
                        helix.dir.set_z(value);
                    }
                }
            }),
            false, None, None));

            // height
            category.add_element(UIElement::new(
                "Height",
                "height", 
                ElemType::Property(Property::new(
                    Value::Float(helix.height), 
                    Box::new(move |_, value, context, _| {
                        let scene = match context.active_scene {
                            Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                            None => return,
                        };
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(helix) = elem.composed_shape_mut().as_helix_mut() {
                            if let Value::Float(value) = value {
                                helix.set_height(value);
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

impl Helix {
    pub fn new(pos: Vec3, dir: Vec3, height: f64) -> Helix {
        Helix {
            pos: pos - dir * height / 2.0,
            dir,
            height
        }
    }

    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn height(&self) -> f64 { self.height }

    // Setters
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
    }
    pub fn set_height(&mut self, radius: f64) {
        self.height = radius;
    }
}