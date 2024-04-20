use crate::{
    model::maths::{quaternion::Quaternion, ray::Ray, vec3::Vec3},
    SCREEN_HEIGHT, SCREEN_WIDTH,
};

const STEP: f64 = 0.2;

#[derive(Debug)]
pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    fov: f64,
    vfov: f64,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn fov(&self) -> f64 {
        self.fov
    }
    pub fn vfov(&self) -> f64 {
        self.vfov
    }
    pub fn u(&self) -> &Vec3 {
        &self.u
    }
    pub fn v(&self) -> &Vec3 {
        &self.v
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.u = Vec3::new(*dir.z(), 0., -*dir.x()).normalize();
        self.v = dir.cross(&self.u).normalize();
        self.dir = dir;
    }
    pub fn set_fov(&mut self, fov: f64) {
        self.fov = fov;
        self.vfov = fov * SCREEN_HEIGHT as f64 / SCREEN_WIDTH as f64;
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, fov: f64) -> Camera {
        let u = Vec3::new(*dir.z(), 0., -*dir.x()).normalize();
        let v = dir.cross(&u).normalize();
        let vfov = fov * SCREEN_HEIGHT as f64 / SCREEN_WIDTH as f64;
        self::Camera {
            pos,
            dir,
            fov,
            u,
            v,
            vfov,
        }
    }

    pub fn default() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 0.0, 0.0),
            fov: 0.,
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            vfov: 0.,
        }
    }

    // Movement methods
    pub fn move_forward(&mut self) {
        self.pos += self.dir() * STEP;
    }
    pub fn move_backward(&mut self) {
        self.pos -= self.dir() * STEP;
    }
    pub fn move_left(&mut self) {
        self.pos -= self.u() * STEP;
    }
    pub fn move_right(&mut self) {
        self.pos += self.u() * STEP;
    }
    pub fn move_up(&mut self) {
        self.pos -= self.v() * STEP;
    }
    pub fn move_down(&mut self) {
        self.pos += self.v() * STEP;
    }
}
