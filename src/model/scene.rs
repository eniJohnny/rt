use crate::model::objects::light::{AmbientLight, Light};

use super::{objects::camera::Camera, Element};

#[derive(Debug)]
pub struct Scene {
    elements: Vec<Element>,
    camera: Camera,
    lights: Vec<Light>,
    ambient_light: AmbientLight
}

impl Scene {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            camera: Camera::default(),
            lights: Vec::new(),
            ambient_light: AmbientLight::default() 
        }
    }

    // Adders
    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }

    pub fn add_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn add_ambient_light(&mut self, ambient_light: AmbientLight) {
        self.ambient_light = ambient_light;
    }

    // Accessors
    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn ambient_light(&self) -> &AmbientLight {
        &self.ambient_light
    }

    // Mutators

    pub fn set_elements(&mut self, elements: Vec<Element>) {
        self.elements = elements;
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn set_lights(&mut self, lights: Vec<Light>) {
        self.lights = lights;
    }

    pub fn set_ambient_light(&mut self, ambient_light: AmbientLight) {
        self.ambient_light = ambient_light;
    }

}
