use std::{f64::consts::PI, sync::{Arc, RwLock}};
use crate::{model::{materials::color::Color, maths::{hit::Hit, ray::Ray, vec3::Vec3}, scene::Scene}, render::raycasting::get_closest_hit, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}, BOUNCE_OFFSET, ELEMENT};
use super::light::{AnyLight, Light};

#[derive(Debug)]
pub struct SpotLight {
    pos: Vec3,
    dir: Vec3,
    intensity: f64,
    color: Color,
    fov: f64,
    fov_rad: f64,
}

impl SpotLight {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn intensity(&self) -> f64 {
        self.intensity
    }
    pub fn color(&self) -> &Color {
        &self.color
    }
    pub fn fov(&self) -> f64 {
        self.fov
    }
    pub fn fov_rad(&self) -> f64 {
        self.fov_rad
    }

    pub fn set_fov(&mut self, fov: f64) {
        self.fov_rad = fov * PI / 180.;
        self.fov = fov;
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, intensity: f64, color: Color, fov: f64) -> Self {
        self::SpotLight {
            pos,
            dir,
            intensity,
            color,
            fov,
            fov_rad: fov * PI / 180.,
        }
    }
}

impl Light for SpotLight {
    fn get_diffuse(&self, hit: &Hit) -> Color {
        let to_light = (self.pos() - hit.pos()).normalize();
        let angle = self.dir().dot(&-&to_light).acos();
        if angle > self.fov_rad() / 2. {
            return Color::new(0., 0., 0.);
        }
        let mut ratio = to_light.dot(hit.norm());
        ratio *= 0_f64
            .max(1. - (self.pos() - hit.pos()).length().powf(2.) / (self.intensity().powf(2.)));
        if ratio < 0. {
            return Color::new(0., 0., 0.);
        }
        ratio *= 1. - angle / (self.fov_rad() / 2.);
        ratio * self.color()
    }

    fn get_specular(&self, hit: &Hit, ray: &Ray) -> Color {
        let to_light = (self.pos() - hit.pos()).normalize();
        let angle = self.dir().dot(&-&to_light).acos();
        if angle > self.fov_rad() / 2. {
            return Color::new(0., 0., 0.);
        }
        let reflected = (-(&to_light) - hit.norm().dot(&-to_light) * 2. * hit.norm()).normalize();
        let mut ratio = (-ray.get_dir()).normalize().dot(&reflected);
        ratio *= 1. - angle / (self.fov_rad() / 2.);
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

    fn as_spot_light(&self) -> Option<&SpotLight> {
        Some(self)
    }

    fn as_spot_light_mut(&mut self) -> Option<&mut SpotLight> {
        Some(self)
    }

    fn get_ui(&self, light: &AnyLight, ui: &mut UI, _: &Arc<RwLock<Scene>>) -> UIElement {
        let id = light.id().clone();
        let mut category = UIElement::new(format!("Spot light {}", id).as_str(), format!("light{}", id).as_str(), ElemType::Category(Category::collapsed()), ui.uisettings());
        category.on_click = Some(Box::new(move |_element,_scene, ui| {
            ui.destroy_box(ELEMENT);
        }));
        let pos = get_vector_ui(*light.light().as_spot_light().unwrap().pos(), "Position", "pos", ui.uisettings(), 
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_spot_light_mut() {
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
                if let Some(light) = light.light_mut().as_spot_light_mut() {
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
                if let Some(light) = light.light_mut().as_spot_light_mut() {
                    if let Value::Float(value) = value {
                        light.pos.set_z(value);
                    }
                }
            }
        }), false, None, None);

        let dir = get_vector_ui(*light.light().as_spot_light().unwrap().dir(), "Direction", "dir", ui.uisettings(), 
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_spot_light_mut() {
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
                if let Some(light) = light.light_mut().as_spot_light_mut() {
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
                if let Some(light) = light.light_mut().as_spot_light_mut() {
                    if let Value::Float(value) = value {
                        light.dir.set_z(value);
                        light.dir = light.dir.normalize();
                    }
                }
            }
        }), false, None, None);

        let color = get_vector_ui(light.light().as_spot_light().unwrap().color().to_vec3(), "Color", "color", ui.uisettings(), 
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_spot_light_mut() {
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
                if let Some(light) = light.light_mut().as_spot_light_mut() {
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
                if let Some(light) = light.light_mut().as_spot_light_mut() {
                    if let Value::Float(value) = value {
                        light.color = Color::new(light.color.r(), light.color.g(), value);
                    }
                }
            }
        }), true, Some(0.), Some(1.));

        let fov = UIElement::new("FOV", "fov", ElemType::Property(Property::new(Value::Float(light.light().as_spot_light().unwrap().fov()),
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_spot_light_mut() {
                    if let Value::Float(value) = value {
                        light.set_fov(value);
                    }
                }
            }
        }),
        Box::new(move |value, _, _| {
            if let Value::Float(value) = value {
                if *value < 0. {
                    return Err("The value should not be inferior to 0".to_string());
                }
                if *value > 360. {
                    return Err("The value should not be superior to 360".to_string());
                }
            }
            Ok(())
        }), ui.uisettings())), ui.uisettings());

        let intensity = UIElement::new("Intensity", "intensity", ElemType::Property(Property::new(Value::Float(light.light().as_spot_light().unwrap().intensity()),
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let light = scene.light_mut_by_id(id.clone()).unwrap();
                if let Some(light) = light.light_mut().as_spot_light_mut() {
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
        category.add_element(dir);
        category.add_element(color);
        category.add_element(fov);
        category.add_element(intensity);
        category
    }
}
