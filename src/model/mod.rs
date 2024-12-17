use materials::material::Material;
use scene::Scene;
use shapes::{Shape, ComposedShape};

pub mod materials;
pub mod shapes;
pub mod maths;
pub mod objects;
pub mod scene;

#[derive(Debug)]
pub struct Element {
    id: usize,
    material: Box<dyn Material + Send +Sync>,
    shape: Box<dyn Sync + Shape>,
    composed_id: Option<usize>
}

impl Element {
    pub fn new(shape: Box<dyn Shape + Sync>, material: Box<dyn Material + Send + Sync>) -> Self {
        Self {
            shape,
            material,
            id: 0,
            composed_id: None
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

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn composed_id(&self) -> Option<usize> {
        self.composed_id
    }

    pub fn set_material(&mut self, material: Box<dyn Material + Send + Sync>) {
        self.material = material;
    }

    pub fn set_shape(&mut self, shape: Box<dyn Sync + Shape>) {
        self.shape = shape;
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn set_composed_id(&mut self, id: usize) {
        self.composed_id = Some(id);
    }
}

#[derive(Debug)]
pub struct ComposedElement {
    composed_shape: Box<dyn Sync + ComposedShape>,
    elements_index: Vec<usize>,
    material: Box<dyn Material + Send +Sync>,
    id: usize,
}

impl ComposedElement {
    pub fn new(composed_shape: Box<dyn Sync + ComposedShape>, material: Box<dyn Material + Send +Sync>) -> Self {
        Self {
            composed_shape,
            elements_index: vec![],
            material,
            id: 0
        }
    }

    fn elements_index(&self) -> &Vec<usize> {
        &self.elements_index
    }
    fn elements_index_mut(&mut self) -> &mut Vec<usize> {
        &mut self.elements_index
    }
    fn set_elements_index(&mut self, elements_index: Vec<usize>) {
        self.elements_index = elements_index;
    }

    pub fn composed_shape(&self) -> &Box<dyn Sync + ComposedShape> {
        &self.composed_shape
    }

    pub fn composed_shape_mut(&mut self) -> &mut Box<dyn Sync + ComposedShape> {
        &mut self.composed_shape
    }

    pub fn material(&self) -> &Box<dyn Material + Send +Sync> {
        &self.material
    }

    pub fn material_mut(&mut self) -> &mut Box<dyn Material + Send +Sync> {
        &mut self.material
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn id(&self) -> usize {
        self.id
    }
}