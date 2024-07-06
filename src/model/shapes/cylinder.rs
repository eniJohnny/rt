use core::panic;
use std::f64::consts::PI;
use std::vec;

use super::Shape;
use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::scene::Scene;
use crate::model::shapes::plane::Plane;
use crate::model::Element;

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
                .expect("The cylinder's planes should be parrallel to each other.")[0];
            t.push(t3.min(t4));
            t.push(t3.max(t4));
        }
        match t.len() {
            1 => {
                return Some(t);
            }
            2 => {
                return None;
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
                if !(t[2] > t[1] || t[3] < t[0]) {
                    t.sort_unstable_by(|a, b| {
                        a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
                    });
                    t.remove(0);
                    t.remove(2);
                    return Some(t);
                }
            }
            _ => panic!("Should never happen"),
        }
        None
    }

	fn outer_intersect(&self, r: &Ray, f: f64, displaced_factor: f64) -> Option<Vec<f64>> {
		self.intersect(r)
	}

    fn intersect_displacement(&self, ray: &Ray, element: &Element, scene: &Scene) -> Option<Vec<f64>> {
		self.intersect(ray)
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

        let i = self.dir().cross(&constant_axis).normalize();
        let j = self.dir().cross(&i).normalize();
        let i_component: f64 = cam_hit.dot(&i);
        let j_component: f64 = cam_hit.dot(&j);
        let ij_hit: Vec3 = (i_component * &i + j_component * &j).normalize();

        projection.u = 0.5 + i_component.atan2(j_component) / (2. * PI);
        projection.i = (&ij_hit).cross(self.dir()).normalize();
        projection.k = hit.norm().clone();

        if level > -0.000001 && level < 0.000001 {
            // Bottom Cap
            projection.j = ij_hit;
            projection.v = (hit.pos() - &self.pos).length() / total_height;
        } else if level > self.height - 0.000001 && level < self.height + 0.000001 {
            // Top Cap
            projection.j = -ij_hit;
            projection.v = (total_height
                - (hit.pos() - &self.pos - &self.dir * &self.height).length())
                / total_height;
        } else {
            // Cylinder
            projection.j = self.dir().clone();
            projection.v = (level + self.radius) / total_height;
        }
        projection
    }

    fn norm(&self, hit: &Vec3, ray_dir: &Vec3) -> Vec3 {
        let pc = hit - &self.pos;
        let coef = pc.dot(&self.dir);
        let projection = &self.dir * coef;

        let norm;
        if coef > -0.000001 && coef < 0.000001 {
            norm = self.plane[0].norm(hit, ray_dir);
        } else if coef > self.height - 0.000001 && coef < self.height + 0.000001 {
            norm = self.plane[1].norm(hit, ray_dir);
        } else {
            norm = (pc - &projection).normalize();
        }
        return norm;
    }
    fn as_cylinder(&self) -> Option<&Cylinder> {
        Some(self)
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
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
