use crate::{model::maths::{ray::Ray, vec3::Vec3}, VFOV_RAD, SCREEN_WIDTH, SCREEN_HEIGHT};

#[derive(Debug)]
pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    fov: f64
}

impl Camera {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn fov(&self) -> f64 { self.fov }

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
        let u = Vec3::new(*self.dir.y(), - *self.dir.x(), 0.).normalize();
        let v = - self.dir.cross(&u).normalize();
        let width = (self.fov as f64/2.).tan() * 2.;
        let height = (VFOV_RAD / 2.).tan() * 2.;
        let center: Vec3 = &self.pos + &self.dir;

        let topLeft = center +  &u * - width/2. + &v * height/2.;
        let leftToRight = u * width;
        let topToBot = v * height;

        let mut result: Vec<Vec<Ray>> = vec![];
        for x in 0..SCREEN_WIDTH {
            let mut line: Vec<Ray> = vec![];
            for y in 0..SCREEN_HEIGHT {
                let pos = self.pos.clone();
                let dir = ((&topLeft + &leftToRight * (x as f64 / SCREEN_WIDTH as f64) + &topToBot * (y as f64 / SCREEN_HEIGHT as f64)) - &pos).normalize();
                let ray = Ray::new(pos, dir, 0);
                line.push(ray);
            }
            result.push(line);
        }
        result
    }
}