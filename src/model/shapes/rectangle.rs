use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::shapes::polygon::{Polygon, Triangle};
use super::Shape;

#[derive(Debug)]
pub struct Rectangle {
    pos: Vec3,
    length: f64,
    width: f64,
    dir_l : Vec3,
    dir_w : Vec3,
    rectangle: Polygon
}

impl Shape for Rectangle {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        self.rectangle.intersect(r)
    }
    fn projection(&self, hit: &Hit) -> (i32, i32) {
        unimplemented!()
    }
    fn norm(&self, hit: &Vec3, ray_dir: &Vec3) -> Vec3 {
        self.rectangle.norm(hit, ray_dir)
    }
    fn as_rectangle(&self) -> Option<&Rectangle> { Some(self) }
}

impl Rectangle {
    // // Accessors
    // pub fn pos(&self) -> &Vec3 { &self.pos }
    // pub fn dir(&self) -> &Vec3 { &self.dir }
    // pub fn radius(&self) -> f64 { self.radius }
    // pub fn height(&self) -> f64 { self.height }
    //
    // // Mutators
    // pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos }
    // pub fn set_dir(&mut self, dir: Vec3) { self.dir = dir }
    // pub fn set_radius(&mut self, radius: f64) { self.radius = radius }
    // pub fn set_height(&mut self, height: f64) { self.height = height }

    // Constructor
    pub fn new(pos: Vec3, length: f64, width: f64, dir_l: Vec3, dir_w: Vec3) -> Rectangle {
        let l_gap = (&dir_l * length) / 2.;
        let w_gap = (&dir_w * width) / 2.;
        let triangle1 = Triangle{a: pos.clone() + &l_gap + &w_gap, b: pos.clone() - &l_gap + &w_gap, c: pos.clone() + &l_gap - &w_gap};
        let triangle2 = Triangle{a: pos.clone() - &l_gap - &w_gap, b: pos.clone() + &l_gap - &w_gap, c: pos.clone() - &l_gap + &w_gap};
        let rectangle = Polygon::new(Vec::from([triangle1, triangle2]));

        Rectangle { pos, length, width, dir_l, dir_w, rectangle }
    }

}