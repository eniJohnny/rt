use std::f64::consts::PI;

use super::Shape;
use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::scene::Scene;
use crate::model::Element;
use crate::render::raycasting::get_sorted_hit_from_t;

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

	fn outer_intersect(&self, r: &Ray, displaced_factor: f64) -> Option<Vec<f64>> {
		let mut outer_sphere = self.clone();
		outer_sphere.set_radius(outer_sphere.radius() + outer_sphere.radius() * displaced_factor);
		outer_sphere.intersect(r)
	}

    fn intersect_displacement(&self, ray: &Ray, element: &Element, scene: &Scene) -> Option<Vec<f64>> {
		// Size of the displacement proportional to the radius
		let displaced_factor: f64 = 0.05; // Maybe add possibility to change this value ?
		let step_size: f64 = 0.1; // step number ~ 1 / step_size 

		let biggest_sphere_size: f64 = self.radius * displaced_factor;
		let mut t: Option<Vec<f64>> = self.outer_intersect(ray, displaced_factor);
		if let Some(mut hits) = get_sorted_hit_from_t(scene, ray, &t, element) {
			if hits.len() == 1 {
				return None; // Inside the sphere
			}

			let mut hit = hits.remove(0);
			let second_hit = hits.remove(0);

			let mut old_t = *hit.dist();
			while hit.dist() < second_hit.dist() {
				let sphere_to_hit = hit.pos() - self.pos();
				let hit_distance = sphere_to_hit.length() - self.radius;
				let hit_ratio: f64 = hit_distance / biggest_sphere_size;

				let displaced_ratio = hit.map_texture(element.material().displacement(), scene.textures()).to_value();
				if (displaced_ratio - hit_ratio).abs() < 0.01 {
					return Some(vec![*hit.dist()]); // Almost perfect match
				}
				if displaced_ratio >= hit_ratio {
					return Some(vec![(*hit.dist() + old_t) / 2.]); // Passed the displacement
				}

				old_t = *hit.dist();
				let mut displaced_dist = (hit_ratio - displaced_ratio) * biggest_sphere_size;
				if displaced_dist > step_size * biggest_sphere_size {
					displaced_dist = step_size * biggest_sphere_size;
				}
				hit = Hit::new(
					element,
					hit.dist() + displaced_dist,
					hit.pos() + ray.get_dir() * displaced_dist,
					ray.get_dir(),
					scene.textures()
				);
			}
		}
		None
	}

    fn projection(&self, hit: &Hit) -> Projection {
		let mut projection = Projection::default();
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
        projection.u = 2. * (f64::atan2(i_component, j_component) + PI) / (2. * PI);
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
