use self::{materials::Material, shapes::Shape};
use crate::render::{camera::Camera, light::AmbientLight};
use crate::render::light::Light;

pub mod materials;
pub mod shapes;
pub mod maths;

pub struct Element {
    shape: Shape,
    material: Material
}

impl Element {
    pub fn new(shape: Shape, material: Material) -> Self {
        Self {
            shape,
            material
        }
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

}