use crate::model::maths::{ray::Ray, vec3::Vec3};

pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    fov: f64
}

impl Camera {
    // Accessors
    pub fn get_pos(&self) -> &Vec3 { &self.pos }
    pub fn get_dir(&self) -> &Vec3 { &self.dir }
    pub fn get_fov(&self) -> f64 { self.fov }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos }
    pub fn set_dir(&mut self, dir: Vec3) { self.dir = dir }
    pub fn set_fov(&mut self, fov: f64) { self.fov = fov }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, fov: f64) -> Camera {
        self::Camera { pos, dir, fov }
    }
    pub fn default() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 0.0, 0.0),
            fov: 0.
        }
    }

    pub fn get_rays(&self) -> Vec<Vec<Ray>> {
        unimplemented!()
    }
}