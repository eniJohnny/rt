use std::collections::HashMap;

use image::{RgbImage, RgbaImage};

use crate::{
    gui::Gui,
    model::objects::light::{AmbientLight, Light},
};

use super::{
    materials::{material::Material, texture::Texture},
    objects::camera::Camera,
    Element,
};

#[derive(Debug)]
pub struct Scene {
    elements: Vec<Element>,
    camera: Camera,
    lights: Vec<Box<dyn Light + Sync + Send>>,
    ambient_light: AmbientLight,
    pub gui: Gui,
    indirect_lightning: bool,
    imperfect_reflections: bool,
    textures: HashMap<String, image::RgbaImage>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            camera: Camera::default(),
            lights: Vec::new(),
            ambient_light: AmbientLight::default(),
            gui: Gui::new(),
            indirect_lightning: true,
            imperfect_reflections: true,
            textures: HashMap::new(),
        }
    }

    // Adders
    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }

    pub fn add_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn add_light(&mut self, light: Box<dyn Light + Sync + Send>) {
        self.lights.push(light);
    }

    pub fn add_ambient_light(&mut self, ambient_light: AmbientLight) {
        self.ambient_light = ambient_light;
    }

    pub fn add_texture(&mut self, name: String, texture: RgbaImage) {
        self.textures.insert(name, texture);
    }

    pub fn add_textures(&mut self, material: &Box<dyn Material + Sync + Send>) {
        let textures = [
            material.color(),
            material.roughness(),
            material.metalness(),
            material.norm(),
        ];
        for texture in textures.iter() {
            match texture {
                Texture::Value(_) => {}
                Texture::Texture(path) => {
                    if path == "" || path.contains("..") {
                        panic!("Textures should only be stored in the textures folder.");
                    }
                    if !self.textures.contains_key(path) {
                        let pathStr = String::from("./textures/") + path;
                        self.add_texture(
                            path.clone(),
                            match image::open(&pathStr) {
                                Ok(img) => img.to_rgba8(),
                                Err(_) => panic!("Error opening texture file {}", path),
                            },
                        );
                    }
                }
            }
        }
    }

    // Accessors
    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }

    pub fn elements_as_mut(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn lights(&self) -> &Vec<Box<dyn Light + Sync + Send>> {
        &self.lights
    }

    pub fn ambient_light(&self) -> &AmbientLight {
        &self.ambient_light
    }

    pub fn indirect_lightning(&self) -> bool {
        self.indirect_lightning
    }

    pub fn imperfect_reflections(&self) -> bool {
        self.imperfect_reflections
    }

    pub fn textures(&self) -> &HashMap<String, image::RgbaImage> {
        &self.textures
    }

    // Mutators

    pub fn set_elements(&mut self, elements: Vec<Element>) {
        self.elements = elements;
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn set_lights(&mut self, lights: Vec<Box<dyn Light + Sync + Send>>) {
        self.lights = lights;
    }

    pub fn set_ambient_light(&mut self, ambient_light: AmbientLight) {
        self.ambient_light = ambient_light;
    }
}
