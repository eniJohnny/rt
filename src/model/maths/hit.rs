use crate::model::Element;

use super::vec3::Vec3;

pub struct Hit<'a> {
    element: &'a Element,
    dist: f64,
    norm: Vec3,
    pos: Vec3
}