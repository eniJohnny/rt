use std::sync::{Arc, RwLock};
use crate::{model::{materials::color::Color, maths::{hit::Hit, ray::Ray, vec3::Vec3}, scene::Scene}, render::raycasting::get_closest_hit, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}, BOUNCE_OFFSET};
use super::light::{AnyLight, Light};

#[derive(Debug)]
pub struct PointLight {
    pos: Vec3,
    intensity: f64,
    color: Color,
}

impl PointLight {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn intensity(&self) -> f64 {
        self.intensity
    }
    pub fn color(&self) -> &Color {
        &self.color
    }

    // Constructor
    pub fn new(pos: Vec3, intensity: f64, color: Color) -> Self {
        self::PointLight {
            pos,
            intensity,
            color,
        }
    }
}

impl Light for PointLight {
    fn get_diffuse(&self, hit: &Hit) -> Color {
        let to_light = (self.pos() - hit.pos()).normalize();
        let mut ratio = to_light.dot(hit.norm());
        if ratio < 0. {
            return Color::new(0., 0., 0.);
        }
        ratio *= 0_f64
            .max(1. - (self.pos() - hit.pos()).length().powf(2.) / (self.intensity().powf(2.)));
        ratio * self.color()
    }

    fn get_specular(&self, hit: &Hit, ray: &Ray) -> Color {
        let to_light = (self.pos() - hit.pos()).normalize();
        let reflected = (-(&to_light) - hit.norm().dot(&-to_light) * 2. * hit.norm()).normalize();
        let mut ratio = (-ray.get_dir()).normalize().dot(&reflected);
        if ratio < 0. {
            return Color::new(0., 0., 0.);
        }
        ratio = ratio.powf(25.);
        ratio *= 0_f64
            .max(1. - (self.pos() - hit.pos()).length().powf(2.) / (self.intensity().powf(2.)));
        ratio * self.color()
    }

    fn throughput(&self, scene: &Scene, hit: &Hit) -> Vec3 {
        let to_light = (self.pos() - hit.pos()).normalize();
        let shadow_ray = Ray::new(hit.pos() + hit.norm() * BOUNCE_OFFSET, to_light, 0);
        let mut throughput = Vec3::from_value(1.);
        if let Some(light_hit) = get_closest_hit(scene, &shadow_ray) {
            for (_, t_list) in light_hit.t_list() {
                for t in t_list {
                    if t > &0. && *t < (hit.pos() - self.pos()).length() {
                        if light_hit.opacity() > (1. - f64::EPSILON) {
                            return Vec3::from_value(0.);
                        } else {
                            throughput = throughput * (1. - light_hit.opacity()) * light_hit.color().to_vec3();
                        }
                    }
                }
            }
        }
        throughput
    }
                    
    fn as_point_light(&self) -> Option<&PointLight> {
        Some(self)
    }

    fn as_point_light_mut(&mut self) -> Option<&mut PointLight> {
        Some(self)
    }

    fn get_ui(&self, light: &AnyLight, ui: &mut UI, _: &Arc<RwLock<Scene>>) -> UIElement {
        let id = light.id().clone();
        let mut category = UIElement::new(format!("Point light {}", id).as_str(), format!("light{}", id).as_str(), ElemType::Category(Category::collapsed()), ui.uisettings());
        let pos = get_vector_ui(*light.light().as_point_light().unwrap().pos(), "Position", "pos", ui.uisettings(), 
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_point_light_mut() {
                    if let Value::Float(value) = value {
                        light.pos.set_x(value);
                    }
                }
            }
        }),
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_point_light_mut() {
                    if let Value::Float(value) = value {
                        light.pos.set_y(value);
                    }
                }
            }
        }),
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_point_light_mut() {
                    if let Value::Float(value) = value {
                        light.pos.set_z(value);
                    }
                }
            }
        }), false, None, None);

        let color = get_vector_ui(light.light().as_point_light().unwrap().color().to_vec3(), "Color", "color", ui.uisettings(), 
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_point_light_mut() {
                    if let Value::Float(value) = value {
                        light.color = Color::new(value, light.color.g(), light.color.b());
                    }
                }
            }
        }),
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_point_light_mut() {
                    if let Value::Float(value) = value {
                        light.color = Color::new(light.color.r(), value, light.color.b());
                    }
                }
            }
        }),
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_point_light_mut() {
                    if let Value::Float(value) = value {
                        light.color = Color::new(light.color.r(), light.color.g(), value);
                    }
                }
            }
        }), true, Some(0.), Some(1.));

        let intensity = UIElement::new("Intensity", "intensity", ElemType::Property(Property::new(Value::Float(light.light().as_point_light().unwrap().intensity()),
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_point_light_mut() {
                    if let Value::Float(value) = value {
                        light.intensity = value;
                    }
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
        }), ui.uisettings())), ui.uisettings());

        category.add_element(pos);
        category.add_element(color);
        category.add_element(intensity);
        category
    }
}
