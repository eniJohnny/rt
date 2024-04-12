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
