use crate::model::maths::{ray::Ray, vec3::Vec3};

#[derive(Debug)]
pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    fov: f64,
    rays: Vec<Vec<Ray>>
}

impl Camera {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn fov(&self) -> f64 { self.fov }
    pub fn rays(&self) -> &Vec<Vec<Ray>> { &self.rays }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos }
    pub fn set_dir(&mut self, dir: Vec3) { self.dir = dir }
    pub fn set_fov(&mut self, fov: f64) { self.fov = fov }
    pub fn set_rays(&mut self, rays: Vec<Vec<Ray>>) {self.rays = rays}

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, fov: f64) -> Camera {
        self::Camera { pos, dir, fov, rays: vec![] }
    }
    pub fn default() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 0.0, 0.0),
            fov: 0.,
            rays: vec![]
        }
    }
}