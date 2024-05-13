use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};

use super::Shape;

#[derive(Debug)]
pub struct Sphere {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
}

impl Shape for Sphere {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }

    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        // intersection rayon/sphere
        let dist = &self.pos - r.get_pos();
        let dot_product = r.get_dir().dot(&dist);
        let discriminant =
            &dot_product * &dot_product - &dist.dot(&dist) + &self.radius * &self.radius;
        if discriminant < 0.0 {
            return None;
        }
        let intersection1 = &dot_product - &discriminant.sqrt();
        let intersection2 = &dot_product + &discriminant.sqrt();
        if intersection1 > 0.0 {
            return Some(Vec::from([intersection1, intersection2]));
        }
        None
    }

    fn projection(&self, hit: &Hit) -> (i32, i32) {
        unimplemented!()
    }
    fn norm(&self, hit_position: &Vec3, ray_dir: &Vec3) -> Vec3 {
        (hit_position - self.pos()).normalize()
    }

    fn as_sphere(&self) -> Option<&Sphere> {
        Some(self)
    }

    fn as_plane(&self) -> Option<&super::plane::Plane> {
        None
    }

    fn as_cylinder(&self) -> Option<&super::cylinder::Cylinder> {
        None
    }

    fn as_cone(&self) -> Option<&super::cone::Cone> {
        None
    }
}

impl Sphere {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64) -> Sphere {
        self::Sphere { pos, dir, radius }
    }

    // Methods
    pub fn clone(&self) -> Sphere {
        let pos = Vec3::new(*self.pos.x(), *self.pos.y(), *self.pos.z());
        let dir = Vec3::new(*self.dir.x(), *self.dir.y(), *self.dir.z());
        self::Sphere {
            pos: pos,
            dir: dir,
            radius: self.radius,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::maths::ray::Ray;
    use crate::model::maths::vec3::Vec3;
    use crate::model::shapes::sphere::Sphere;
    use crate::model::shapes::Shape;

    #[test]
    fn test_intersect() {
        let s1: Sphere = Sphere::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.), 1.);
        let r1: Ray = Ray::new(Vec3::new(-5., 0., 0.), Vec3::new(1., 0., 0.), 5);
        assert_eq!(s1.intersect(&r1), Some(Vec::from([4., 6.])));
    }

    #[test]
    fn test_intersect2() {
        let s1: Sphere = Sphere::new(Vec3::new(0., 0., 2.), Vec3::new(0., 0., 0.), 1.);
        let r1: Ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.), 5);
        assert_eq!(s1.intersect(&r1), Some(Vec::from([1., 3.])));
    }

    #[test]
    fn test_intersect3() {
        let s1: Sphere = Sphere::new(Vec3::new(0., 0., 2.), Vec3::new(0., 0., 0.), 1.);
        let r1: Ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), 5);
        assert_eq!(s1.intersect(&r1), None);
    }
}
