use std::collections::HashMap;

use image::{RgbImage, RgbaImage};

use crate::{
    model::objects::light::{AmbientLight, Light},
    render::settings::Settings,
    bvh
};

use super::{
    materials::{diffuse::{self, Diffuse},
    material::{self, Material},
    texture::Texture},
    objects::camera::Camera,
    shapes::{self, aabb::Aabb},
    Element,
};

#[derive(Debug)]
pub struct Scene {
    elements: Vec<Element>,
    camera: Camera,
    lights: Vec<Box<dyn Light + Sync + Send>>,
    ambient_light: AmbientLight,
    settings: Settings,
    textures: HashMap<String, image::RgbaImage>,
    dirty: bool,
    bvh: Option<bvh::node::Node>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            camera: Camera::default(),
            lights: Vec::new(),
            ambient_light: AmbientLight::default(),
            settings: Settings::default(),
            textures: HashMap::new(),
            dirty: true,
            bvh: None,
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

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
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
            material.emissive(),
            material.refraction(),
            material.opacity(),
        ];
        for texture in textures.iter() {
            match texture {
                Texture::Value(..) => {}
                Texture::Texture(path, _) => {
                    if path == "" || path.contains("..") {
                        panic!("Textures should only be stored in the textures folder.");
                    }
                    if !self.textures.contains_key(path) {
                        let path_str = String::from("./textures/") + path;
                        self.add_texture(
                            path.clone(),
                            match image::open(&path_str) {
                                Ok(img) => img.to_rgba8(),
                                Err(_) => panic!("Error opening texture file {}", path),
                            },
                        );
                    }
                }
            }
        }
    }

    pub fn add_skysphere_texture(&mut self, path: &str) {
        let key = "skysphere";

        if path == "" || path.contains("..") {
            panic!("Textures should only be stored in the textures folder.");
        }
        if !self.textures.contains_key(key) {
            let path_str = String::from("./textures/") + path;
            
            self.add_texture(
                key.to_string(),
                match image::open(&path_str) {
                    Ok(img) => img.to_rgba8(),
                    Err(_) => panic!("Error opening texture file {}", path),
                },
            );
        }
    }
    
    pub fn add_wireframes(&mut self) {
        let aabbs = self.all_aabb();
        let mut new_elements = vec![];
        for aabb in aabbs {
            let new_material = Diffuse::default();
            let new_shape = shapes::wireframe::Wireframe::from_aabb(aabb);
            let new_element = Element::new(Box::new(new_shape), new_material);

            new_elements.push(new_element);
        }
        self.elements.append(&mut new_elements);
    }

    pub fn update_bvh(&mut self) {
        let aabbs = self.all_aabb();
        let biggest_aabb = Aabb::from_aabbs(&aabbs);
        let mut node = bvh::node::Node::new(&biggest_aabb);
        node.split(self);

        self.bvh = Some(node);
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

    pub fn textures(&self) -> &HashMap<String, image::RgbaImage> {
        &self.textures
    }

    pub fn get_texture(&self, name: &str) -> Option<&image::RgbaImage> {
        self.textures.get(name)
    }
    
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }
    
    pub fn get_element(&self, index: usize) -> &Element {
        &self.elements[index]
    }

    pub fn all_aabb(&self) -> Vec<&crate::model::shapes::aabb::Aabb> {
        self.elements
            .iter()
            .filter_map(|element| element.shape().as_aabb())
            .collect()
    }

    pub fn bvh(&self) -> &Option<bvh::node::Node> {
        &self.bvh
    }

    pub fn non_bvh_elements(&self) -> Vec<&crate::model::Element> {
        self.elements
            .iter()
            .filter(|element| element.shape().aabb().is_none())
            .collect()
    }

    pub fn test_all_elements(&self) -> Vec<&crate::model::Element> {
        self.elements
            .iter()
            .collect()
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

    pub fn set_bvh(&mut self, bvh: Option<bvh::node::Node>) {
        self.bvh = bvh;
    }
}
