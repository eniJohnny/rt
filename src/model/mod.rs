use self::{materials::Material, maths::ray::Ray, shapes::Shape};

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

pub struct Scene;
