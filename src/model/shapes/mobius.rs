use core::f64;
use std::f64::consts::PI;
use std::f64::EPSILON;

use crate::model::materials::material::Projection;
use crate::model::{maths::{hit::Hit, ray::Ray, vec3::Vec3}, Element};
use crate::model::scene::Scene;
use crate::ERROR_MARGIN;
use super::Shape;
use roots::{find_roots_cubic, Roots};

#[derive(Debug)]
pub struct Mobius {
    pos: Vec3,
    dir: Vec3,
    a: f64,
}

#[derive(Debug)]
struct Term {
    pub c0: f64,
    pub c1: f64,
    pub c2: f64,
    pub c3: f64,
}

impl Shape for Mobius {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }

    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>> {// Transform ray origin to Möbius strip local space
        let transformed_origin = ray.get_pos() - self.pos;
    
        // Define parameters for the Möbius strip
        const U_STEPS: usize = 200; // Number of u steps
        const V_STEPS: usize = 50;   // Number of v steps
    
        // Iterate over the Möbius strip's parameters
        for i in 0..U_STEPS {
            let u = (i as f64) * (2.0 * PI / U_STEPS as f64);
            for j in 0..V_STEPS {
                let v = (j as f64) / (V_STEPS as f64) - 0.5; // Scale v from -0.5 to 0.5
    
                // Calculate point on Möbius strip
                let p = uv_to_xyz(u, v) * self.a;
    
                // Calculate vector from ray origin to point on the Möbius strip
                let delta = p - transformed_origin;
    
                // Calculate intersection parameter t along the ray direction
                let denom = ray.get_dir().dot(&ray.get_dir());
                let t = delta.dot(&ray.get_dir()) / denom;
    
                // Check if t is non-negative (ray points towards the strip)
                if t >= 0.0 {
                    // Calculate intersection point
                    let intersection_point = ray.get_pos() + ray.get_dir() * t;
    
                    // Check if the intersection point is close to the point on the strip
                    if (intersection_point - p).length_squared() < 1e-3 { // Tolerance
                        return Some(vec![t]); // Intersection found
                    }
                }
            }
        }
        
        None // No intersection found
    }

	fn outer_intersect(&self, r: &Ray, displaced_factor: f64) -> Option<Vec<f64>> {
		self.intersect(r)
	}

    fn intersect_displacement(&self, ray: &Ray, element: &Element, scene: &Scene) -> Option<Vec<f64>> {
		self.intersect(ray)
	}

    fn projection(&self, hit: &Hit) -> Projection {
        let mut closest_u = 0.0;
        let mut closest_v = 0.0;
        let mut min_distance = f64::MAX;

        const U_STEPS: usize = 200; // Number of u steps
        const V_STEPS: usize = 50;   // Number of v steps

        // Search for the closest parameters u and v
        for i in 0..U_STEPS {
            let u = (i as f64) * (2.0 * PI / U_STEPS as f64);
            for j in 0..V_STEPS {
                let v = (j as f64) / (V_STEPS as f64) - 0.5; // Scale v from -0.5 to 0.5
                let p = uv_to_xyz(u, v) * self.a + self.pos;

                let distance = (p - hit.pos()).length_squared();
                if distance < min_distance {
                    min_distance = distance;
                    closest_u = u;
                    closest_v = v;
                }
            }
        }

        let (u, v) = (closest_u, closest_v);
        let k: Vec3 = hit.norm().clone();
        let i: Vec3 = hit.norm().cross(&self.dir).normalize();
        let j: Vec3 = hit.norm().cross(&i).normalize();

        Projection { u, v, i, j, k }
    }

    fn norm(&self, hit: &Vec3, ray_dir: &Vec3) -> Vec3 {
        let mut closest_u = 0.0;
        let mut closest_v = 0.0;
        let mut min_distance = f64::MAX;

        const U_STEPS: usize = 200; // Number of u steps
        const V_STEPS: usize = 50;   // Number of v steps

        // Search for the closest parameters u and v
        for i in 0..U_STEPS {
            let u = (i as f64) * (2.0 * PI / U_STEPS as f64);
            for j in 0..V_STEPS {
                let v = (j as f64) / (V_STEPS as f64) - 0.5; // Scale v from -0.5 to 0.5
                let p = uv_to_xyz(u, v) * self.a + self.pos;

                let distance = (p - hit).length_squared();
                if distance < min_distance {
                    min_distance = distance;
                    closest_u = u;
                    closest_v = v;
                }
            }
        }

        // let (u, v) = xyz_to_uv(*hit.x(), *hit.y(), *hit.z());
        let (u, v) = (closest_u, closest_v);
        let du = 1e-5;
        let dv = 1e-5;

        let p_u = uv_to_xyz(u + dv, v) - uv_to_xyz(u, v);
        let p_v = uv_to_xyz(u, v + dv) - uv_to_xyz(u, v);

        let tangent_u = p_u / du;
        let tangent_v = p_v / dv;

        let norm = tangent_u.cross(&tangent_v).normalize();
        
        norm
    }

    fn pos(&self) -> &Vec3 { &self.pos }

    fn as_mobius(&self) -> Option<&Mobius> { Some(self) }
}

impl Mobius {
    // // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn a(&self) -> f64 { self.a }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos; }
    pub fn set_dir(&mut self, dir: Vec3) { self.dir = dir; }
    pub fn set_a(&mut self, a: f64) { self.a = a; }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, a: f64 ) -> Mobius {
        Mobius { pos, dir, a }
    }

}

fn uv_to_xyz(u: f64, v: f64) -> Vec3 {
    let r = 1.0 + (v / 2.0).cos();
    let x = r * u.cos();
    let z = r * u.sin();
    let y = v;

    Vec3::new(x, y, z)
}

fn xyz_to_uv(x: f64, y: f64, z: f64) -> (f64, f64) {
    let u = (x / (1.0 + (z / 2.0).cos()).cos()).atan2(y);
    let v = z;

    (u, v)
}