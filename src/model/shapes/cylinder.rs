use core::panic;
use std::vec;

use super::Shape;
use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::shapes::plane::Plane;

#[derive(Debug)]
pub struct Cylinder {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
    height: f64,
    plane: [Plane; 2],
}

impl Shape for Cylinder {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        //d:    direction du rayon
        //co:   vecteur entre la postion du cylindre et le point d'origine du rayon
        //v:    vecteur directeur du cylindre
        //abc:  les coefficients
        let dv = r.get_dir().cross(&self.dir);
        let cov = (r.get_pos() - &self.pos).cross(&self.dir);
        let a = dv.dot(&dv);
        let b = cov.dot(&dv) * 2.0;
        let c = cov.dot(&cov) - (self.radius * self.radius);

        let mut delta = b * b - 4.0 * a * c;

        let mut t = Vec::new();

        if delta > 0.0 {
            delta = delta.sqrt();
            let (t1, t2) = ((-b - delta) / (2.0 * a), (-b + delta) / (2.0 * a));
            t.push(t1.min(t2));
            t.push(t1.max(t2));
        } else if delta == 0.0 {
            t.push(-b / (2.0 * a));
        }

        if let Some(t3) = self.plane[0].intersect(r) {
            let t3 = t3[0];
            let t4 = self.plane[1]
                .intersect(r)
                .expect("The cylinder's planes should be parrallel to each other")[0];
            t.push(t3.min(t4));
            t.push(t3.max(t4));
        }
        match t.len() {
            2 => {
                if delta < 0.0 {
                    // On ne touche que les deux plans, on n'intersecte donc que si on est a l'interieur du cylindre
                    let mut base = Vec3::new(0.3, 0.8, 0.6);
                    if base == self.dir {
                        base = Vec3::new(0.4, -0.5, 0.3);
                    }
                    if (r.get_pos() - &self.pos).dot(&self.dir.cross(&base)) < self.radius {
                        return Some(t);
                    }
                } else {
                    let dot_hit_dir = (r.get_pos() - &self.pos).dot(&self.dir);
                    if dot_hit_dir > 0. && dot_hit_dir < self.height {
                        return Some(t);
                    }
                }
            }
            3 => {
                // On ne touche que la tranche du cylindre, on n'intersecte que si le t cylindre est entre les deux plans (inclusif)
                if t[0] >= t[1] && t[0] <= t[1] {
                    t.truncate(1);
                    return Some(t);
                }
            }
            4 => {
                // 99.9% des cas, le classico
                if t[2] > t[1] && t[0] < t[3] {
                    t.remove(0);
                    t.remove(2);
                    t.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                    return Some(t);
                }
            }
            _ => panic!("Should never happen"),
        }
        None
    }

    fn projection(&self, hit: &Hit) -> Projection {
        let mut projection: Projection = Projection::default();

        let cam_hit = hit.pos() - &self.pos;
        let level = cam_hit.dot(&self.dir);
        let total_height = self.height + self.radius * 2.0;

        let constant_axis: Vec3;
        if self.dir == Vec3::new(0., 0., 1.) {
            constant_axis = Vec3::new(0., 1., 0.);
        } else {
            constant_axis = Vec3::new(0., 0., 1.);
        }

        projection.i = self.dir().cross(&constant_axis).normalize();
        projection.j = self.dir().cross(&projection.i).normalize();
        projection.k = hit.norm().clone();
        let i_component: f64 = cam_hit.dot(&projection.i);
        let j_component: f64 = cam_hit.dot(&projection.j);
        let ij_hit: Vec3 = (i_component * &projection.i + j_component * &projection.j).normalize();
        let is_front: bool = ij_hit.dot(&projection.j) > 0.;
        if is_front {
            projection.u = (ij_hit.dot(&projection.i) + 1.) / 4.;
        } else {
            projection.u = 1. - (ij_hit.dot(&projection.i) + 1.) / 4.;
        }
        if level > -0.000001 && level < 0.000001 {
            // Bottom Cap
            projection.v = (hit.pos() - &self.pos).length() / total_height;
        } else if level > self.height - 0.000001 && level < self.height + 0.000001 {
            // Top Cap
            projection.v = (total_height
                - (hit.pos() - &self.pos - &self.dir * &self.height).length())
                / total_height;
        } else {
            // Cylinder
            projection.v = (level + self.radius) / total_height;
        }
        projection
    }

    fn norm(&self, hit: &Vec3, ray_dir: &Vec3) -> Vec3 {
        let pc = hit - &self.pos;
        let coef = pc.dot(&self.dir);
        let projection = &self.dir * coef;

        if coef > -0.000001 && coef < 0.000001 {
            return self.plane[0].norm(hit, ray_dir);
        }
        if coef > self.height - 0.000001 && coef < self.height + 0.000001 {
            return self.plane[1].norm(hit, ray_dir);
        }

        return (hit - (&self.pos + &projection)).normalize();
    }
    fn as_cylinder(&self) -> Option<&Cylinder> {
        Some(self)
    }
}

impl Cylinder {
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
    pub fn height(&self) -> f64 {
        self.height
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
    pub fn set_height(&mut self, height: f64) {
        self.height = height
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, height: f64) -> Cylinder {
        let plane1 = Plane::new(pos.clone(), -dir.clone());
        let plane2 = Plane::new(pos.clone() + dir.clone() * height, dir.clone());
        self::Cylinder {
            pos,
            dir,
            radius,
            height,
            plane: [plane1, plane2],
        }
    }
}
