use std::sync::{Arc, RwLock};

use crate::{model::{composed_element::ComposedElement, element::Element, materials::material::Material, maths::vec3::Vec3, scene::Scene}, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}};

use super::composed_shape::ComposedShape;

#[derive(Debug)]
pub struct CustomComposed {
    pub elements: Vec<Element>,
    pub pos: Vec3,
    pub dir: Vec3,
    pub scale: f64,
}

impl CustomComposed {
    pub fn new(elements: Vec<Element>, pos: Vec3, dir: Vec3, scale: f64) -> Self {
        Self {
            elements,
            pos,
            dir: dir.normalize(),
            scale,
        }
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }
}

impl ComposedShape for CustomComposed {
    fn generate_elements(&self, material: Box<dyn Material + Send +Sync>) -> Vec<Element> {

        self.elements
    }

    fn get_ui(&self, element: &ComposedElement, ui: &mut UI, scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Custom composed", "custom", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(custom) = self.as_custom_composed() {
            let id = element.id();

            category.add_element(get_vector_ui(custom.pos.clone(), "Position", "pos", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(custom) = elem.composed_shape_mut().as_custom_composed_mut() {
                    if let Value::Float(value) = value {
                        custom.pos.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(custom) = elem.composed_shape_mut().as_custom_composed_mut() {
                    if let Value::Float(value) = value {
                        custom.pos.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(custom) = elem.composed_shape_mut().as_custom_composed_mut() {
                    if let Value::Float(value) = value {
                        custom.pos.set_z(value);
                    }
                }
            }),
            false, None, None));

            category.add_element(get_vector_ui(custom.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(custom) = elem.composed_shape_mut().as_custom_composed_mut() {
                    if let Value::Float(value) = value {
                        let mut new_dir = custom.dir.clone();
                        new_dir.set_x(value);
                        custom.dir = new_dir.normalize();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(custom) = elem.composed_shape_mut().as_custom_composed_mut() {
                    if let Value::Float(value) = value {
                        let mut new_dir = custom.dir.clone();
                        new_dir.set_y(value);
                        custom.dir = new_dir.normalize();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(custom) = elem.composed_shape_mut().as_custom_composed_mut() {
                    if let Value::Float(value) = value {
                        let mut new_dir = custom.dir.clone();
                        new_dir.set_z(value);
                        custom.dir = new_dir.normalize();
                    }
                }
            }),
            false, None, None));

            category.add_element(UIElement::new(
                "Scale",
                "scale", 
                ElemType::Property(Property::new(
                    Value::Float(custom.scale), 
                    Box::new(move |_, value, scene, _: &mut UI| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(obj) = elem.composed_shape_mut().as_custom_composed_mut() {
                            if let Value::Float(value) = value {
                                obj.set_scale(value);
                            }
                        }
                        scene.set_dirty(true);
                    }), Box::new(|value, _, _| {
                        if let Value::Float(value) = value {
                            if *value <= 0.0 {
                                Err("Scale must be greater than 0.".to_string())
                            } else {
                                Ok(())
                            }
                        } else {
                            Err("Scale must be a valid float.".to_string())
                        }
                    }),
                    ui.uisettings())),
                ui.uisettings()));
        }
        category
    }
}