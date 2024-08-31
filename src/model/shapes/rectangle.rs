use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::shapes::plane::Plane;
use crate::model::shapes::triangle::Triangle;
use super::aabb::Aabb;
use super::Shape;

#[derive(Debug)]
pub struct Rectangle {
    pos: Vec3,
    length: f64,
    width: f64,
    dir_l : Vec3,
    dir_w : Vec3,
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
    plane: Plane,
    aabb: super::aabb::Aabb,
}

impl Shape for Rectangle {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {

        let mut intersection: f64;
        match self.plane.intersect(r) {
            Some(intersections) => {
                intersection = intersections[0];
            },
            _ => {
                return None;
            }
        }

        let p = intersection * r.get_dir() + r.get_pos();

        if Triangle::inside_triangle(&p, &self.d, &self.b, &self.c, &self.plane.dir()) || Triangle::inside_triangle(&p, &self.a, &self.b, &self.c, &self.plane.dir()) {
            return Some(Vec::from([intersection]));
        }
        None
    }
    fn projection(&self, hit: &Hit) -> Projection {
        Projection::default()
    }
    fn norm(&self, hit: &Vec3, ray_dir: &Vec3) -> Vec3 {
        self.plane.norm(hit, ray_dir)
    }
    fn pos(&self) -> &Vec3 { &self.pos }
    fn as_rectangle(&self) -> Option<&Rectangle> { Some(self) }
}

impl Rectangle {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn length(&self) -> &f64 { &self.length }
    pub fn width(&self) -> &f64 { &self.width }
    pub fn dir_l(&self) -> &Vec3 { &self.dir_l }
    pub fn dir_w(&self) -> &Vec3 { &self.dir_w }
    pub fn aabb(&self) -> &Aabb { &self.aabb }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.update_aabb();
    }

    pub fn set_length(&mut self, length: f64) {
        self.length = length;
        self.update_aabb();
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
        self.update_aabb();
    }

    pub fn set_dir_l(&mut self, dir_l: Vec3) {
        self.dir_l = dir_l;
        self.update_aabb();
    }

    pub fn set_dir_w(&mut self, dir_w: Vec3) {
        self.dir_w = dir_w;
        self.update_aabb();
    }

    pub fn set_aabb(&mut self, aabb: Aabb) {
        self.aabb = aabb;
    }

    // Constructor
    pub fn new(pos: Vec3, length: f64, width: f64, dir_l: Vec3, dir_w: Vec3) -> Rectangle {
        let l_gap = (&dir_l.clone().normalize() * length) / 2.;
        let w_gap = (&dir_w.clone().normalize() * width) / 2.;
        let a = pos.clone() + &l_gap + &w_gap;
        let b= pos.clone() - &l_gap + &w_gap;
        let c= pos.clone() + &l_gap - &w_gap;
        let d= pos.clone() - &l_gap - &w_gap;
        let plane = Plane::new(a.clone(), dir_l.clone().cross(&dir_w).normalize());
        let aabb = self::Rectangle::compute_aabb(&a, &d);

        Rectangle { pos, length, width, dir_l, dir_w, a, b, c, d, plane, aabb }
    }

    fn update_aabb(&mut self) {
        self.set_aabb(self::Rectangle::compute_aabb(&self.a, &self.d));
    }

    pub fn compute_aabb(a: &Vec3, d: &Vec3) -> super::aabb::Aabb {
        let x_min = a.x().min(*d.x());
        let x_max = a.x().max(*d.x());
        let y_min = a.y().min(*d.y());
        let y_max = a.y().max(*d.y());
        let z_min = a.z().min(*d.z());
        let z_max = a.z().max(*d.z());
        Aabb::new(x_min, x_max, y_min, y_max, z_min, z_max)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::maths::ray::Ray;

    #[test]
    fn test_rectangle_intersect() {
        let r = Rectangle::new(Vec3::new(0., 0., 1.), 2., 2., Vec3::new(1., 0., 0.), Vec3::new(0., 1., 0.));
        let ray = Ray::new(Vec3::new(-0.1, 0., 0.), Vec3::new(0., 0., 1.), 5);
        let intersections = r.intersect(&ray);
        assert_eq!(intersections, Some(vec![1.]));
    }
}