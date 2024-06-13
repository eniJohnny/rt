use std::default;

use super::Shape;
use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::ERROR_MARGIN;


#[derive(Debug, Clone)]
pub struct Aabb {
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    z_min: f64,
    z_max: f64,
    pos: Vec3,
}

impl Aabb {
    // Constructor
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64, z_min: f64, z_max: f64) -> Aabb {
        let pos = Vec3::new(
            (x_min + x_max) / 2.0,
            (y_min + y_max) / 2.0,
            (z_min + z_max) / 2.0,
        );
        self::Aabb {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
            pos,
        }
    }

    // Accessors
    pub fn x_min(&self) -> f64 {
        self.x_min
    }
    pub fn x_max(&self) -> f64 {
        self.x_max
    }
    pub fn y_min(&self) -> f64 {
        self.y_min
    }
    pub fn y_max(&self) -> f64 {
        self.y_max
    }
    pub fn z_min(&self) -> f64 {
        self.z_min
    }
    pub fn z_max(&self) -> f64 {
        self.z_max
    }
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    // Mutators
    pub fn set_x_min(&mut self, x_min: f64) {
        self.x_min = x_min
    }
    pub fn set_x_max(&mut self, x_max: f64) {
        self.x_max = x_max
    }
    pub fn set_y_min(&mut self, y_min: f64) {
        self.y_min = y_min
    }
    pub fn set_y_max(&mut self, y_max: f64) {
        self.y_max = y_max
    }
    pub fn set_z_min(&mut self, z_min: f64) {
        self.z_min = z_min
    }
    pub fn set_z_max(&mut self, z_max: f64) {
        self.z_max = z_max
    }
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos
    }

}

impl Shape for Aabb {
    fn distance(&self, vec: &Vec3) -> f64 {
        let dx = vec.x().max(self.x_min()).min(self.x_max()) - vec.x();
        let dy = vec.y().max(self.y_min()).min(self.y_max()) - vec.y();
        let dz = vec.z().max(self.z_min()).min(self.z_max()) - vec.z();

        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>> {
        let tmin_x = (self.x_min() - ray.get_pos().x()) / ray.get_dir().x();
        let tmax_x = (self.x_max() - ray.get_pos().x()) / ray.get_dir().x();
        let tmin_y = (self.y_min() - ray.get_pos().y()) / ray.get_dir().y();
        let tmax_y = (self.y_max() - ray.get_pos().y()) / ray.get_dir().y();
        let tmin_z = (self.z_min() - ray.get_pos().z()) / ray.get_dir().z();
        let tmax_z = (self.z_max() - ray.get_pos().z()) / ray.get_dir().z();

        let tmin = get_tmin(tmin_x, tmax_x, tmin_y, tmax_y, tmin_z, tmax_z);
        let tmax = get_tmax(tmin_x, tmax_x, tmin_y, tmax_y, tmin_z, tmax_z);

        if tmin > 0.0 && tmax > 0.0 && tmin < tmax {
            return Some(Vec::from([tmin, tmax]));
        }

        None
    }

    fn projection(&self, hit: &Hit) -> Projection {
        todo!()
    }

    fn norm(&self, hit_position: &Vec3, ray_dir: &Vec3) -> Vec3 {
        let x = *hit_position.x();
        let y = *hit_position.y();
        let z = *hit_position.z();
    
        if (x - self.x_min()).abs() < ERROR_MARGIN {
            return Vec3::new(-1.0, 0.0, 0.0);
        } else if (x - self.x_max()).abs() < ERROR_MARGIN {
            return Vec3::new(1.0, 0.0, 0.0);
        } else if (y - self.y_min()).abs() < ERROR_MARGIN {
            return Vec3::new(0.0, -1.0, 0.0);
        } else if (y - self.y_max()).abs() < ERROR_MARGIN {
            return Vec3::new(0.0, 1.0, 0.0);
        } else if (z - self.z_min()).abs() < ERROR_MARGIN {
            return Vec3::new(0.0, 0.0, -1.0);
        } else if (z - self.z_max()).abs() < ERROR_MARGIN {
            return Vec3::new(0.0, 0.0, 1.0);
        } else {
            // DEBUG - print all the diffs
            // let xmin_diff = (x - self.x_min()).abs();
            // let xmax_diff = (x - self.x_max()).abs();
            // let ymin_diff = (y - self.y_min()).abs();
            // let ymax_diff = (y - self.y_max()).abs();
            // let zmin_diff = (z - self.z_min()).abs();
            // let zmax_diff = (z - self.z_max()).abs();
            // println!("----------------------------------------------------");
            // println!("xmin_diff: {} - {}", xmin_diff, xmin_diff < ERROR_MARGIN);
            // println!("xmax_diff: {} - {}", xmax_diff, xmax_diff < ERROR_MARGIN);
            // println!("ymin_diff: {} - {}", ymin_diff, ymin_diff < ERROR_MARGIN);
            // println!("ymax_diff: {} - {}", ymax_diff, ymax_diff < ERROR_MARGIN);
            // println!("zmin_diff: {} - {}", zmin_diff, zmin_diff < ERROR_MARGIN);
            // println!("zmax_diff: {} - {}", zmax_diff, zmax_diff < ERROR_MARGIN);
            // println!("----------------------------------------------------");

            panic!("Error: hit_position is not on the AABB.\nThe problem certainly comes from the error margin.\nYou can use the debug print right above this message (src/model/shapes/aabb.rs:151 atm) to see why it didn't trigger.\nAdjust ERROR_MARGIN (src/lib.rs) if needed.");
        }
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
    }
    fn as_aabb(&self) -> Option<&Aabb> {
        Some(self)
    }
}

fn get_tmin(tmin_x: f64, tmax_x: f64, tmin_y: f64, tmax_y: f64, tmin_z: f64, tmax_z: f64) -> f64 {
    let xmin = tmin_x.min(tmax_x);
    let ymin = tmin_y.min(tmax_y);
    let zmin = tmin_z.min(tmax_z);

    let tmin = xmin.max(ymin).max(zmin);
    tmin
}

fn get_tmax(tmin_x: f64, tmax_x: f64, tmin_y: f64, tmax_y: f64, tmin_z: f64, tmax_z: f64) -> f64 {
    let xmax = tmin_x.max(tmax_x);
    let ymax = tmin_y.max(tmax_y);
    let zmax = tmin_z.max(tmax_z);

    let tmax = xmax.min(ymax).min(zmax);
    tmax
}
