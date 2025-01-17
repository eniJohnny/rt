use super::{materials::material::Material, shapes::shape::Shape};

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

    pub fn material(&self) -> &Box<dyn Material + Send +Sync> {
        &self.material
    }

    pub fn material_mut(&mut self)-> &mut Box<dyn Material + Send +Sync> {
        &mut self.material
    }

    pub fn shape(&self) -> &Box<dyn Shape + Sync> {
        &self.shape
    }
    pub fn shape_mut(&mut self) -> &mut Box<dyn Shape + Sync> {
        &mut self.shape
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
