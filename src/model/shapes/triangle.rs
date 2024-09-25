use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::shapes::plane::Plane;
use super::Shape;

#[derive(Debug)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    dir: Vec3,
    plane: Plane,
    aabb: super::aabb::Aabb,
}

impl Shape for Triangle {
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
        if Triangle::inside_triangle(&p, &self.a, &self.b, &self.c, &self.dir) {
            return Some(Vec::from([intersection]));
        }
        None
    }
    fn projection(&self, hit: &Hit) -> Projection {
        self.plane.projection(hit)
    }
    fn norm(&self, hit: &Vec3, ray_dir: &Vec3) -> Vec3 {
        return self.plane.norm(hit, ray_dir);
    }
    fn pos(&self) -> &Vec3 { &self.a }
    fn as_triangle(&self) -> Option<&Triangle> { Some(self) }
}

impl Triangle {
    // // Accessors
    pub fn get_a(&self) -> &Vec3 { &self.a }
    pub fn get_b(&self) -> &Vec3 { &self.b }
    pub fn get_c(&self) -> &Vec3 { &self.c }
    pub fn aabb(&self) -> &super::aabb::Aabb { &self.aabb }

    // Mutators
    pub fn set_a(&mut self, a: Vec3) {
        self.a = a;
        self.update_aabb();
    }

    pub fn set_b(&mut self, b: Vec3) {
        self.b = b;
        self.update_aabb();
    }

    pub fn set_c(&mut self, c: Vec3) {
        self.c = c;
        self.update_aabb();
    }

    pub fn set_aabb(&mut self, aabb: super::aabb::Aabb) {
        self.aabb = aabb;
    }

    pub fn inside_triangle(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3, dir: &Vec3) -> bool {
        let pa = (b - a).cross(&(p - a)).dot(&dir);
        let pb = (c - b).cross(&(p - b)).dot(&dir);
        let pc = (a - c).cross(&(p - c)).dot(&dir);
        if (pa >= 0. && pb >= 0. && pc >= 0.) || (pa <= 0. && pb <= 0. && pc <= 0.) {
            return true;
        }
        false
    }

    // Constructor
    pub fn new(a: Vec3, b: Vec3, c: Vec3 ) -> Triangle {

        let dir =(b.clone() - &a).cross(&(c.clone() - &a)).normalize();
        let plane = Plane::new(a.clone(), dir.clone());
        let aabb = self::Triangle::compute_aabb(&a, &b, &c);
        Triangle {
            a,
            b,
            c,
            dir,
            plane,
            aabb,
        }
    }

    pub fn update_aabb(&mut self) {
        self.set_aabb(self::Triangle::compute_aabb(&self.a, &self.b, &self.c));
    }

    pub fn compute_aabb(a: &Vec3, b: &Vec3, c: &Vec3) -> super::aabb::Aabb {
        let x_min = a.x().min(*b.x()).min(*c.x());
        let x_max = a.x().max(*b.x()).max(*c.x());
        let y_min = a.y().min(*b.y()).min(*c.y());
        let y_max = a.y().max(*b.y()).max(*c.y());
        let z_min = a.z().min(*b.z()).min(*c.z());
        let z_max = a.z().max(*b.z()).max(*c.z());

        super::aabb::Aabb::new(x_min, x_max, y_min, y_max, z_min, z_max)
    }

}