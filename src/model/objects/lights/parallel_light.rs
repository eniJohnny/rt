use std::sync::{Arc, RwLock};
use crate::{model::{materials::color::Color, maths::{hit::Hit, ray::Ray, vec3::Vec3}, scene::Scene}, render::raycasting::get_closest_hit, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}, BOUNCE_OFFSET};
use super::light::{AnyLight, Light};

#[derive(Debug, Clone)]
pub struct ParallelLight {
    dir: Vec3,
    intensity: f64,
    color: Color,
}

impl ParallelLight {
    // Accessors
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn intensity(&self) -> f64 {
        self.intensity
    }
    pub fn color(&self) -> &Color {
        &self.color
    }

    // Constructor
    pub fn new(dir: Vec3, intensity: f64, color: Color) -> Self {
        self::ParallelLight {
            dir,
            intensity,
            color,
        }
    }
}

impl Light for ParallelLight {
    fn get_diffuse(&self, hit: &Hit) -> Color {
        let mut ratio = (-self.dir()).dot(hit.norm());
        if ratio < f64::EPSILON {
            return Color::new(0., 0., 0.);
        }
        ratio *= 0_f64.max(self.intensity());
        (ratio * self.color()).clamp(0., 1.)
    }

    fn get_specular(&self, hit: &Hit, ray: &Ray) -> Color {
        let to_light = -self.dir();
        let reflected = (-(&to_light) - hit.norm().dot(&-to_light) * 2. * hit.norm()).normalize();
        let mut ratio = (-ray.get_dir()).normalize().dot(&reflected);
        if ratio < f64::EPSILON {
            return Color::new(0., 0., 0.);
        }
        ratio = ratio.powf(50.);
        ratio *= self.intensity().powi(2);
        (ratio * self.color()).clamp(0., 1.)
    }

    fn throughput(&self, scene: &Scene, hit: &Hit) -> Vec3 {
        let mut shadow_ray = Ray::new(hit.pos() + hit.norm() * BOUNCE_OFFSET, -self.dir(), 0);
        let mut throughput = Vec3::from_value(1.);
        while throughput.length() > f64::EPSILON {
            if let Some(light_hit) = get_closest_hit(scene, &shadow_ray) {
                if light_hit.opacity() > (1. - f64::EPSILON) {
                    return Vec3::from_value(0.);
                } else {
                    throughput = throughput * (1. - light_hit.opacity()) * light_hit.color().to_vec3();
                    shadow_ray.set_pos(*light_hit.pos() + *shadow_ray.get_dir() * BOUNCE_OFFSET);
                }
            } else {
                return throughput;
            }
        }
        
        throughput
    }

    fn as_parallel_light(&self) -> Option<&ParallelLight> {
        Some(self)
    }

    fn as_parallel_light_mut(&mut self) -> Option<&mut ParallelLight> {
        Some(self)
    }

	fn get_ui(&self, light: &AnyLight, ui: &mut UI, _: &Arc<RwLock<Scene>>) -> UIElement {
		let id = light.id().clone();
		let mut category = UIElement::new(format!("Parallel light {}", id).as_str(), format!("light{}", id).as_str(), ElemType::Category(Category::collapsed()), ui.uisettings());

		let dir = get_vector_ui(*light.light().as_parallel_light().unwrap().dir(), "Direction", "dir", ui.uisettings(), 
		Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
			    let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_parallel_light_mut() {
                    if let Value::Float(value) = value {
                        light.dir.set_x(value);
                        light.dir = light.dir.normalize();
                    }
                }
            }
		}),
		Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_parallel_light_mut() {
                    if let Value::Float(value) = value {
                        light.dir.set_y(value);
                        light.dir = light.dir.normalize();
                    }
                }
            }
		}),
		Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_parallel_light_mut() {
                    if let Value::Float(value) = value {
                        light.dir.set_z(value);
                        light.dir = light.dir.normalize();
                    }
                }
            }
		}), false, None, None);

		let color = get_vector_ui(light.light().as_parallel_light().unwrap().color().to_vec3(), "Color", "color", ui.uisettings(), 
		Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_parallel_light_mut() {
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
                if let Some(light) = light.light_mut().as_parallel_light_mut() {
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
                if let Some(light) = light.light_mut().as_parallel_light_mut() {
                    if let Value::Float(value) = value {
                        light.color = Color::new(light.color.r(), light.color.g(), value);
                    }
                }
            }
		}), true, Some(0.), Some(1.));

		let intensity = UIElement::new("Intensity", "intensity", ElemType::Property(Property::new(Value::Float(light.light().as_parallel_light().unwrap().intensity()),
		Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_parallel_light_mut() {
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

		category.add_element(dir);
		category.add_element(color);
		category.add_element(intensity);
		category
	}
}