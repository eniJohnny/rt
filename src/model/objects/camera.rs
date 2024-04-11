use crate::{model::maths::{ray::Ray, vec3::Vec3}, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    fov: i32
}

impl Camera {
    pub fn get_rays(&self) -> Vec<Vec<Ray>> {
        unimplemented!()
    }
}