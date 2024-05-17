use crate::model::Element;

use super::vec3::Vec3;

#[derive(Debug, Clone)]
pub struct Hit<'a> {
    element: &'a Element,
    dist: f64,
    pos: Vec3,
    norm: Vec3,
    projected_pos: Option<(i32, i32)>
}

impl<'a> Hit<'a> {
    pub fn new(element: &'a Element, dist: f64, pos: Vec3, ray_dir: &Vec3) -> Self {
        Hit {
            element, 
            dist,
            norm: element.shape().norm(&pos, ray_dir),
            pos,
            projected_pos: None
        }
    }

    pub fn element(&self) -> &'a Element {
        self.element
    }

    pub fn dist(&self) -> &f64 {
        &self.dist
    }

    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub fn norm(&self) -> &Vec3 {
		&self.norm
    }

}