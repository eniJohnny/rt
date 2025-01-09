use super::{materials::material::Material, shapes::composed_shape::ComposedShape};

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

    pub fn elements_index(&self) -> &Vec<usize> {
        &self.elements_index
    }
    pub fn elements_index_mut(&mut self) -> &mut Vec<usize> {
        &mut self.elements_index
    }
    pub fn set_elements_index(&mut self, elements_index: Vec<usize>) {
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