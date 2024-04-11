use crate::model::maths::vec3::Vec3;

pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    fov: f64
}

impl Camera {
    // Accessors
    pub fn get_pos(&self) -> Vec3 { Vec3::new(self.pos.x().to_owned(), self.pos.y().to_owned(), self.pos.z().to_owned()) }
    pub fn get_dir(&self) -> Vec3 { Vec3::new(self.dir.x().to_owned(), self.dir.y().to_owned(), self.dir.z().to_owned()) }
    pub fn get_fov(&self) -> f64 { self.fov }

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
}