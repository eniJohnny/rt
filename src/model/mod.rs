use materials::material::Material;

use self::{shapes::Shape};

pub mod materials;
pub mod shapes;
pub mod maths;
pub mod objects;
pub mod scene;

#[derive(Debug)]
pub struct Element {
    id: u32,
    material: Box<dyn Sync + Material>,
    shape: Box<dyn Sync + Shape>,
}

impl Element {
    pub fn new(shape: Box<dyn Shape + Sync>, material: Box<dyn Material + Sync>) -> Self {
        Self {
            shape,
            material,
            id: 0
        }
    }

    pub fn material(&self) -> &dyn Material {
        self.material.as_ref()
    }

    pub fn material_mut(&mut self)-> &mut dyn Material {
        self.material.as_mut()
    }

    pub fn shape(&self) -> &dyn Shape {
        self.shape.as_ref()
    }
    pub fn shape_mut(&mut self) -> &mut dyn Shape {
        self.shape.as_mut()
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn set_material(&mut self, material: Box<dyn Sync + Material>) {
        self.material = material;
    }

    pub fn set_shape(&mut self, shape: Box<dyn Sync + Shape>) {
        self.shape = shape;
    }

    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
}
