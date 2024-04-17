use self::{materials::Material, shapes::Shape};

pub mod materials;
pub mod shapes;
pub mod maths;
pub mod objects;
pub mod scene;

#[derive(Debug)]
pub struct Element {
    material: Box<dyn Sync + Material>,
    shape: Box<dyn Sync + Shape>
}

impl Element {
    pub fn new(shape: Box<dyn Shape + Sync>, material: Box<dyn Material + Sync>) -> Self {
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
