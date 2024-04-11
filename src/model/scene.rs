use super::{objects::camera::Camera, Element};


pub struct Scene {
    camera: Camera,
    elements: Vec<Element>
}

impl Scene {
    pub fn new(camera: Camera, elements: Vec<Element>) -> Self {
        Self{
            camera,
            elements
        }
    }

    pub fn camera(&self) -> &Camera {
       &self.camera
    }

    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }
}