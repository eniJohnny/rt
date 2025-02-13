use std::sync::{Arc, RwLock};

use crate::{model::{materials::color::Color, scene::Scene}, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}};

#[derive(Debug)]
pub struct AmbientLight {
    intensity: f64,
    color: Color,
}

impl AmbientLight {
    // Accessors
    pub fn intensity(&self) -> f64 {
        self.intensity
    }
    pub fn color(&self) -> &Color {
        &self.color
    }

    // Constructor
    pub fn new(intensity: f64, color: Color) -> Self {
        self::AmbientLight { intensity, color }
    }
    pub fn default() -> Self {
        Self {
            intensity: 0.,
            color: Color::new(1., 1., 1.),
        }
    }

    pub fn get_ui(&self, ui: &mut UI, _: &Arc<RwLock<Scene>>) -> UIElement {
        let mut ambient_category = UIElement::new("Ambient light", "ambient", ElemType::Category(Category::collapsed()), ui.uisettings());

        ambient_category.add_element(get_vector_ui(self.color.to_vec3(), "Color", "ambient.color", ui.uisettings(), 
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let ambient = scene.ambient_light_mut();
                if let Value::Float(value) = value {
                    ambient.color = Color::new(value, ambient.color.g(), ambient.color.b());
                }
            }
        }),
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let ambient = scene.ambient_light_mut();
                if let Value::Float(value) = value {
                    ambient.color = Color::new(ambient.color.r(), value, ambient.color.b());
                }
            }
        }),
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let ambient = scene.ambient_light_mut();
                if let Value::Float(value) = value {
                    ambient.color = Color::new(ambient.color.r(), ambient.color.g(), value);
                }
            }
        }), true, Some(0.), Some(1.)));

        ambient_category.add_element(UIElement::new("Intensity", "intensity", ElemType::Property(Property::new(Value::Float(self.intensity),
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let ambient = scene.ambient_light_mut();
                if let Value::Float(value) = value {
                    ambient.intensity = value;
                }
            }
        }),
        Box::new(move |value, _, _| {
            if let Value::Float(value) = value {
                if *value < 0. {
                    return Err("The value should not be inferior to 0".to_string());
                }
            }
            Ok(())
        }), ui.uisettings())), ui.uisettings()));


		ambient_category
    }
}