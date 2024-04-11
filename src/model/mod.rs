use self::{materials::Material, shapes::Shape};
use crate::model::objects::{camera::Camera, light::AmbientLight, light::Light};

pub mod materials;
pub mod shapes;
pub mod maths;
pub mod objects;
pub mod scene;

pub struct Element {
    material: Box<dyn Material>,
    shape: Box<dyn Shape>
}

impl Element {
    pub fn new(shape: Box<dyn Shape>, material: Box<dyn Material>) -> Self {
        Self {
            shape,
            material
        }
    }

    pub fn material(&self) -> &dyn Material {
        self.material.as_ref()
    }

    pub fn shape(&self) -> &dyn Shape {
        self.shape.as_ref()
    }
}
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

    // Add a new object to the scene
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

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

}
