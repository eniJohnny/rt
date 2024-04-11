use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use super::HasShape;

pub struct Cylinder {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
    height: f64
}

impl HasShape for Cylinder {
    fn distance(&self) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, vector: &Ray) -> Option<Hit> {
        unimplemented!()
    }
    fn projection(&self, hit: &Hit) -> Option<(i32, i32)> {
        unimplemented!()
    }
}

impl Cylinder {
    // Accessors
    pub fn get_pos(&self) -> Vec3 { Vec3::new(self.pos.x().to_owned(), self.pos.y().to_owned(), self.pos.z().to_owned()) }
    pub fn get_dir(&self) -> Vec3 { Vec3::new(self.dir.x().to_owned(), self.dir.y().to_owned(), self.dir.z().to_owned()) }
    pub fn get_radius(&self) -> f64 { self.radius.to_owned() }
    pub fn get_height(&self) -> f64 { self.height.to_owned() }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, height: f64) -> Cylinder {
        self::Cylinder { pos, dir, radius, height}
    }

}