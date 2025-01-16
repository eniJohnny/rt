use super::shape::Shape;
use core::f64;
use std::sync::{Arc, RwLock};
use crate::{
    model::{
        materials::material::Projection,
        maths::{hit::Hit, ray::Ray, vec3::Vec3},
        scene::Scene,
        element::Element
    }, ui::{
        ui::UI,
        uielement::UIElement,
        utils::misc::ElemType
    }, ERROR_MARGIN, WIREFRAME_THICKNESS
};

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

    pub fn from_min_max(min: Vec3, max: Vec3) -> Aabb {
        Aabb::new(*min.x(), *max.x(), *min.y(), *max.y(), *min.z(), *max.z())
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
            // println!("Aabb : x[{},{}], y[{}, {}], z[{}, {}]", aabb.x_min, aabb.x_max, aabb.y_min, aabb.y_max, aabb.z_min, aabb.z_max);
            x_min = x_min.min(aabb.x_min());
            x_max = x_max.max(aabb.x_max());
            y_min = y_min.min(aabb.y_min());
            y_max = y_max.max(aabb.y_max());
            z_min = z_min.min(aabb.z_min());
            z_max = z_max.max(aabb.z_max());
        }

        // println!("Aabb : x[{},{}], y[{}, {}], z[{}, {}]", x_min, x_max, y_min, y_max, z_min, z_max);
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
        let x = (self.x_max - self.x_min).abs();
        let y = (self.y_max - self.y_min).abs();
        let z = (self.z_max - self.z_min).abs();

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
        
        2.0 * ((x * y) + (y * z) + (x * z))
    }

    pub fn split_aabb(&mut self, axis: usize, t: f64) -> (Aabb, Aabb) {
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

    fn grow_to_include(&mut self, aabb: &Aabb) {
        
        if aabb.x_min < self.x_min {
            self.x_min = aabb.x_min;
        }
        if aabb.z_min < self.z_min {
            self.z_min = aabb.z_min;
        }
        if aabb.y_min < self.y_min {
            self.y_min = aabb.y_min;
        }
        if aabb.x_max > self.x_max {
            self.x_max = aabb.x_max;
        }
        if aabb.y_max > self.y_max {
            self.y_max = aabb.y_max;
        }
        if aabb.z_max > self.z_max {
            self.z_max = aabb.z_max;
        }
    }

    pub fn get_children_and_shrink(&mut self, scene: &Scene, parent_vec: &Vec<usize>) -> Vec<usize> {
        let mut children = vec![];

        let mut new_aabb = Aabb::new(f64::MAX, f64::MIN, f64::MAX, f64::MIN, f64::MAX, f64::MIN);

        for i in parent_vec {
            let element = scene.elements().get(*i).unwrap();
            let mut aabb_shape = None;

            if let Some(aabb) = element.shape().as_aabb() {
                aabb_shape = Some(aabb);
            } else if let Some(aabb) = element.shape().aabb() {
                aabb_shape = Some(aabb);
            }

            if let Some(aabb_shape) = aabb_shape {
                if aabb_shape.is_child_of(self) {
                    children.push(*i);
                    new_aabb.grow_to_include(aabb_shape);
                }
            }
        }
        if children.len() > 0 {
            if new_aabb.x_min > self.x_min {
                self.x_min = new_aabb.x_min;
            }
            if new_aabb.x_max < self.x_max {
                self.x_max = new_aabb.x_max;
            }
            if new_aabb.y_min > self.y_min {
                self.y_min = new_aabb.y_min;
            }
            if new_aabb.y_max < self.y_max {
                self.y_max = new_aabb.y_max;
            }
            if new_aabb.z_min > self.z_min {
                self.z_min = new_aabb.z_min;
            }
            if new_aabb.z_max < self.z_max {
                self.z_max = new_aabb.z_max;
            }
        }
        children
    }

    pub fn is_child_of(&self, aabb: &Aabb) -> bool {
        self.x_min >= aabb.x_min && self.x_max <= aabb.x_max &&
        self.y_min >= aabb.y_min && self.y_max <= aabb.y_max &&
        self.z_min >= aabb.z_min && self.z_max <= aabb.z_max
    }

    pub fn overlaps(&self, aabb: &Aabb) -> bool {
        if self == aabb {
            return false;
        }
        
        let x_overlap = self.x_min() < aabb.x_max() && self.x_max() > aabb.x_min();
        let y_overlap = self.y_min() < aabb.y_max() && self.y_max() > aabb.y_min();
        let z_overlap = self.z_min() < aabb.z_max() && self.z_max() > aabb.z_min();

        x_overlap && y_overlap && z_overlap
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

        if tmin <= tmax {
            return Some(Vec::from([tmin, tmax]));
        }

        None
    }

	fn outer_intersect(&self, r: &Ray, _displaced_factor: f64) -> Option<Vec<f64>> {
		self.intersect(r)
	}

    fn intersect_displacement(&self, ray: &Ray, _element: &Element, _scene: &Scene) -> Option<Vec<f64>> {
		self.intersect(ray)
	}

    fn projection(&self, _hit: &Hit) -> Projection {
        Projection::default()
    }

    fn norm(&self, hit_position: &Vec3) -> Vec3 {
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
            return Vec3::new(0.0, 0.0, 0.0);
        }
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
    }
    fn as_aabb(&self) -> Option<&Aabb> {
        Some(self)
    }
    fn as_aabb_mut(&mut self) -> Option<&mut Aabb> {
        Some(self)
    }

    fn get_ui(&self, _element: &Element, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
        UIElement::new("UI not defined for AABBs", "notdefined", ElemType::Text, ui.uisettings())
    }

    fn aabb(&self) -> Option<&Aabb> {
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