use crate::model::Element;

use super::vec3::Vec3;

pub struct Hit<'a> {
    element: &'a Element,
    dist: f64,
    pos: Vec3,
    norm: Option<Vec3>,
    projected_pos: Option<(i32, i32)>
}

impl<'a> Hit<'a> {
    pub fn new(element: &'a Element, dist: f64, pos: Vec3) -> Self {
        Hit {
            element, 
            dist,
            pos,
            norm :None,
            projected_pos: None
        }
    }

    pub fn element(&self) -> &'a Element {
        self.element
    }

    pub fn dist(&self) -> &f64 {
        &self.dist
    }

    pub fn norm(&self) -> &Vec3 {
        match &self.norm {
            Some(vec) => vec,
            None => unimplemented!()
        }
    }

}