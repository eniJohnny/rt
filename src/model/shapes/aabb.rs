use std::default;

use super::Shape;
use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::scene::Scene;
use crate::{ERROR_MARGIN, WIREFRAME_THICKNESS, AABB_STEPS_NB};


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

    // Get the AABB of a list of AABBs
    pub fn from_aabbs(aabbs: &Vec<&Aabb>) -> Aabb {
        let mut x_min = f64::MAX;
        let mut x_max = f64::MIN;
        let mut y_min = f64::MAX;
        let mut y_max = f64::MIN;
        let mut z_min = f64::MAX;
        let mut z_max = f64::MIN;

        for aabb in aabbs {
            x_min = x_min.min(aabb.x_min());
            x_max = x_max.max(aabb.x_max());
            y_min = y_min.min(aabb.y_min());
            y_max = y_max.max(aabb.y_max());
            z_min = z_min.min(aabb.z_min());
            z_max = z_max.max(aabb.z_max());
        }

        Aabb::new(x_min, x_max, y_min, y_max, z_min, z_max)
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
        self.x_min = x_min;
        self.update_pos();
    }
    pub fn set_x_max(&mut self, x_max: f64) {
        self.x_max = x_max;
        self.update_pos();
    }
    pub fn set_y_min(&mut self, y_min: f64) {
        self.y_min = y_min;
        self.update_pos();
    }
    pub fn set_y_max(&mut self, y_max: f64) {
        self.y_max = y_max;
        self.update_pos();
    }
    pub fn set_z_min(&mut self, z_min: f64) {
        self.z_min = z_min;
        self.update_pos();
    }
    pub fn set_z_max(&mut self, z_max: f64) {
        self.z_max = z_max;
        self.update_pos();
    }
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos
    }

    // Methods
    pub fn update_pos(&mut self) {
        self.set_pos(Vec3::new(
            (self.x_min + self.x_max) / 2.0,
            (self.y_min + self.y_max) / 2.0,
            (self.z_min + self.z_max) / 2.0,
        ));
    }

    pub fn intersection(&self, aabb: &Aabb) -> Option<Aabb> {
        let x_min = self.x_min().max(aabb.x_min());
        let x_max = self.x_max().min(aabb.x_max());
        let y_min = self.y_min().max(aabb.y_min());
        let y_max = self.y_max().min(aabb.y_max());
        let z_min = self.z_min().max(aabb.z_min());
        let z_max = self.z_max().min(aabb.z_max());

        if x_min < x_max && y_min < y_max && z_min < z_max {
            Some(Aabb::new(x_min, x_max, y_min, y_max, z_min, z_max))
        } else {
            None
        }
    }

    pub fn volume(&self) -> f64 {
        let x = self.x_max - self.x_min;
        let y = self.y_max - self.y_min;
        let z = self.z_max - self.z_min;

        x * y * z
    }

    pub fn min(&self) -> Vec<f64> {
        vec![self.x_min, self.y_min, self.z_min]
    }

    pub fn max(&self) -> Vec<f64> {
        vec![self.x_max, self.y_max, self.z_max]
    }

    pub fn is_wireframe_point(&self, point: &Vec3) -> bool {
        let x = *point.x();
        let y = *point.y();
        let z = *point.z();

        // Two dimensions are close to the same edge = point is part of the wireframe
        let x_min_near_edge = (x - self.x_min).abs() < WIREFRAME_THICKNESS;
        let x_max_near_edge = (x - self.x_max).abs() < WIREFRAME_THICKNESS;
        let y_min_near_edge = (y - self.y_min).abs() < WIREFRAME_THICKNESS;
        let y_max_near_edge = (y - self.y_max).abs() < WIREFRAME_THICKNESS;
        let z_min_near_edge = (z - self.z_min).abs() < WIREFRAME_THICKNESS;
        let z_max_near_edge = (z - self.z_max).abs() < WIREFRAME_THICKNESS;

        let near_edge_count = x_min_near_edge as i32 + x_max_near_edge as i32 + y_min_near_edge as i32 + y_max_near_edge as i32 + z_min_near_edge as i32 + z_max_near_edge as i32;

        return near_edge_count >= 2;
    }

    pub fn surface_area(&self) -> f64 {
        let x = self.x_max - self.x_min;
        let y = self.y_max - self.y_min;
        let z = self.z_max - self.z_min;
        
        2.0 * (x * y + y * z + z * x)
    }

    pub fn calculate_sah_cost(&self, aabb1: &Aabb, aabb2: &Aabb, scene: &Scene) -> f64 {
        let total_surface_area = self.surface_area();
        let aabb1_surface_area = aabb1.surface_area();
        let aabb2_surface_area = aabb2.surface_area();

        let P1 = aabb1_surface_area / total_surface_area;
        let P2 = aabb2_surface_area / total_surface_area;

        let C1 = aabb1_surface_area * aabb1.get_children_elements(scene).len() as f64; // Can be optimized
        let C2 = aabb2_surface_area * aabb2.get_children_elements(scene).len() as f64; // Can be optimized
        let Ct = 1.0; // Predefined constant in SAH, typically 1

        Ct + P1 * C1 + P2 * C2
    }

    pub fn better_split(&mut self, scene: &Scene) -> (Aabb, Aabb) {
        let mut aabb1 = self.clone();
        let mut aabb2 = self.clone();
        let mut costs: Vec<Cost> = vec![];

        let original_cost = self.surface_area() * self.get_children_elements(scene).len() as f64;
        let t_vec = get_t_vec(AABB_STEPS_NB);

        // Default cost is 1.0 (worst case scenario, original traversal cost)
        let mut best_cost = Cost {
            cost: original_cost,
            axis: 0,
            t: 0.0,
        };

        for axis in 0..2 {
            for t in &t_vec {
                let (aabb1_tmp, aabb2_tmp) = self.split(axis, *t);
                let cost = self.calculate_sah_cost(&aabb1_tmp, &aabb2_tmp, scene);

                if cost < best_cost.cost {
                    best_cost = Cost { cost, axis, t: *t };
                }

                costs.push(Cost { cost, axis, t: *t });
            }
        }

        if best_cost.cost < original_cost {
            let (aabb1_tmp, aabb2_tmp) = self.split(best_cost.axis, best_cost.t);
            aabb1 = aabb1_tmp;
            aabb2 = aabb2_tmp;
        }

        // dbg!(original_cost, costs, aabb1 == aabb2);

        (aabb1, aabb2)
    }

    pub fn split(&mut self, axis: usize, t: f64) -> (Aabb, Aabb) {
        // Split AABB into two parts along the axis at distance t


        let mut aabb1 = self.clone();
        let mut aabb2 = self.clone();

        if axis == 0 {
            let new_x = self.x_min + (self.x_max - self.x_min) * t;
            aabb1.set_x_max(new_x);
            aabb2.set_x_min(new_x);
        } else if axis == 1 {
            let new_y = self.y_min + (self.y_max - self.y_min) * t;
            aabb1.set_y_max(new_y);
            aabb2.set_y_min(new_y);
        } else if axis == 2 {
            let new_z = self.z_min + (self.z_max - self.z_min) * t;
            aabb1.set_z_max(new_z);
            aabb2.set_z_min(new_z);
        }

        aabb1.update_pos();
        aabb2.update_pos();

        (aabb1, aabb2)
    }

    pub fn get_children_aabbs(&self, scene: &Scene) -> Vec<Aabb> {
        let aabbs = scene.all_aabb();
        let mut children = vec![];

        for aabb in aabbs {
            if self.is_child(&aabb) {
                children.push(aabb.clone());
            }
        }

        children
    }

    pub fn get_children_aabbs_id(&self, scene: &Scene) -> Vec<usize> {
        let elements = scene.elements();
        let mut children = vec![];

        for (i, element) in elements.iter().enumerate() {
            if element.shape().as_aabb().is_some() && self.is_child(element.shape().as_aabb().unwrap()){
                children.push(i);
            }
        }

        children
    }

    pub fn get_children_elements(&self, scene: &Scene) -> Vec<usize> {
        let elements = scene.elements();
        let mut children = vec![];

        for (i, element) in elements.iter().enumerate() {
            let mut aabb_shape = None;

            if let Some(aabb) = element.shape().as_aabb() {
                aabb_shape = Some(aabb);
            } else if let Some(aabb) = element.shape().aabb() {
                aabb_shape = Some(aabb);
            }

            if aabb_shape.is_some() && self.is_child(aabb_shape.unwrap()) {
                children.push(i);
            }
        }

        children
    }

    pub fn get_children_number(&self, scene: &Scene) -> usize {
        let elements = scene.elements();
        let mut children = 0;

        for element in elements {
            if element.shape().as_aabb().is_some() || element.shape().aabb().is_some() {
                children += 1;
            }
        }

        children
    }

    pub fn get_children_elements_only(&self, scene: &Scene) -> Vec<usize> {
        // Does not return the children aabbs

        let elements = scene.elements();
        let mut children = vec![];

        for (i, element) in elements.iter().enumerate() {
            let is_aabb = element.shape().as_aabb().is_some();
            let aabb = element.shape().aabb();
            let has_aabb = aabb.is_some();
            
            if !is_aabb && has_aabb && self.is_child(aabb.unwrap()) {
                children.push(i);
            }
        }

        children
    }
    
    pub fn get_children_non_bvh_elements(&self, scene: &Scene) -> Vec<usize> {
        let elements = scene.elements();
        let mut children = vec![];

        for (i, element) in elements.iter().enumerate() {
            let is_aabb = element.shape().as_aabb().is_some();
            let aabb = element.shape().aabb();
            let has_aabb = aabb.is_some();
            
            if !is_aabb && !has_aabb {
                children.push(i);
            }
        }

        children
    }

    pub fn is_child(&self, aabb: &Aabb) -> bool {
        if self == aabb {
            return false;
        }
        
        let x_overlap = self.x_min() < aabb.x_max() && self.x_max() > aabb.x_min();
        let y_overlap = self.y_min() < aabb.y_max() && self.y_max() > aabb.y_min();
        let z_overlap = self.z_min() < aabb.z_max() && self.z_max() > aabb.z_min();

        x_overlap && y_overlap && z_overlap
    }

    pub fn contains_point(&self, point: &Vec3) -> bool {
        let x = *point.x();
        let y = *point.y();
        let z = *point.z();

        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max && z >= self.z_min && z <= self.z_max
    }
}

impl PartialEq for Aabb {
    fn eq(&self, other: &Self) -> bool {
        self.x_min == other.x_min
            && self.x_max == other.x_max
            && self.y_min == other.y_min
            && self.y_max == other.y_max
            && self.z_min == other.z_min
            && self.z_max == other.z_max
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
        Projection::default()
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

#[derive(Debug, Clone, Copy)]
struct Cost {
    cost: f64,
    axis: usize,
    t: f64,
}

pub fn get_t_vec(steps: usize) -> Vec<f64> {
    let mut t_vec = vec![];

    for i in 0..steps {
        t_vec.push((i as f64 + 1.0) / (steps as f64 + 1.0));
    }

    t_vec
}

// Tests

pub fn aabb_split_test() {
    let mut aabb = Aabb::new(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);
    let axis = 0;
    let t = 0.75;

    let (aabb1, aabb2) = aabb.split(axis, t);

    dbg!(aabb, aabb1, aabb2);
}

pub fn aabb_better_split_test(scene: &Scene) {
    let aabbs = scene.all_aabb();
    let mut biggest_aabb = Aabb::from_aabbs(&aabbs);

    let (aabb1, aabb2) = biggest_aabb.better_split(scene);

    dbg!(biggest_aabb, aabb1, aabb2);
}

pub fn aabb_get_children_test(scene: &Scene) {
    let aabbs = scene.all_aabb();
    let aabb = aabbs.first().expect("No AABBs in scene");
    let biggest_aabb = Aabb::from_aabbs(&aabbs);

    dbg!(aabb, aabb.get_children_aabbs(scene));
    dbg!(&biggest_aabb, biggest_aabb.get_children_aabbs(scene));
}
