use std::f64::consts::PI;

use super::Shape;
use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};

#[derive(Debug)]
pub struct Sphere {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
    aabb: super::aabb::Aabb,
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

    fn projection(&self, hit: &Hit) -> Projection {
        let mut projection: Projection = Projection::default();

        let constant_axis: Vec3;
        if *hit.norm() == Vec3::new(0., 0., 1.) {
            constant_axis = Vec3::new(0., 1., 0.);
        } else {
            constant_axis = Vec3::new(0., 0., 1.);
        }
        let i = self.dir().cross(&constant_axis).normalize();
        let j = self.dir().cross(&i).normalize();
        projection.k = hit.norm().clone();
        let i_component: f64 = hit.norm().dot(&i);
        let j_component: f64 = hit.norm().dot(&j);
        let k_component: f64 = hit.norm().dot(&self.dir);
        projection.u = (f64::atan2(i_component, j_component) + PI) / (2. * PI);
        projection.v = f64::acos(k_component) / PI;
        projection.i = hit.norm().cross(&self.dir).normalize();
        projection.j = hit.norm().cross(&projection.i).normalize();
        projection
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
    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn aabb(&self) -> Option<&super::aabb::Aabb> {
        Some(&self.aabb)
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
    pub fn aabb(&self) -> &super::aabb::Aabb {
        &self.aabb
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.update_aabb();
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
        self.update_aabb();
    }
    pub fn set_aabb(&mut self, aabb: super::aabb::Aabb) {
        self.aabb = aabb
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64) -> Sphere {
        let aabb = self::Sphere::compute_aabb(&pos, radius);
        self::Sphere { pos, dir, radius, aabb }
    }

    // Methods
    pub fn clone(&self) -> Sphere {
        let pos = Vec3::new(*self.pos.x(), *self.pos.y(), *self.pos.z());
        let dir = Vec3::new(*self.dir.x(), *self.dir.y(), *self.dir.z());
        self::Sphere {
            pos: pos,
            dir: dir,
            radius: self.radius,
            aabb: self.aabb.clone(),
        }
    }

    fn update_aabb(&mut self) {
        self.aabb.set_x_min(self.pos.x() - self.radius);
        self.aabb.set_x_max(self.pos.x() + self.radius);
        self.aabb.set_y_min(self.pos.y() - self.radius);
        self.aabb.set_y_max(self.pos.y() + self.radius);
        self.aabb.set_z_min(self.pos.z() - self.radius);
        self.aabb.set_z_max(self.pos.z() + self.radius);
    }

    pub fn compute_aabb(pos: &Vec3, radius: f64) -> super::aabb::Aabb {
        super::aabb::Aabb::new(
            pos.x() - radius,
            pos.x() + radius,
            pos.y() - radius,
            pos.y() + radius,
            pos.z() - radius,
            pos.z() + radius,
        )
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
