use super::Shape;
use crate::model::{materials::material::Projection, maths::{hit::Hit, ray::Ray, vec3::Vec3}};

#[derive(Debug)]
pub struct Plane {
    pos: Vec3,
    dir: Vec3,
}

impl Shape for Plane {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        let dist = &self.pos - r.get_pos();
        let mut dir = self.dir.clone();
        let mut dot_product = r.get_dir().dot(&self.dir);
        if dot_product > 0. {
            dir = -dir;
            dot_product = -dot_product;
        }
        let t = dist.dot(&dir) / dot_product;
        if t > 0. {
            return Some(Vec::from([t]));
        }
        return None;
    }

    fn projection(&self, hit: &Hit) -> Projection {
		let mut projection: Projection = Projection::default();

		let constant_axis: Vec3;
		if self.dir == Vec3::new(0., 0., 1.) {
			constant_axis = Vec3::new(0., 1., 0.);
		} else {
			constant_axis = Vec3::new(0., 0., 1.);
		}
		projection.i = self.dir().cross(&constant_axis).normalize();
		projection.j = self.dir().cross(&projection.i).normalize();
		projection.k = hit.norm().clone();
		let dist = hit.pos() - self.pos();
		let i_component = dist.dot(&projection.i);
		let j_component = dist.dot(&projection.j);
		projection.u = &i_component - (i_component as i32) as f64;
		if projection.u < 0. {
			projection.u += 1.;
		}
		projection.v = &j_component - (j_component as i32) as f64;
		if projection.v < 0. {
			projection.v += 1.;
		}
		projection
    }

    fn norm(&self, hit_pos: &Vec3, ray_dir: &Vec3) -> Vec3 {
        // On doit aussi prendre on compte quand on tape de l'autre cote du plane
        if ray_dir.dot(&self.dir) > 0. {
            return - self.dir.clone();
        }
        self.dir.clone()
    }
    fn as_plane(&self) -> Option<&Plane> {
        Some(self)
    }
}

impl Plane {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3) -> Plane {
        self::Plane { pos, dir }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::maths::ray::Ray;
    use crate::model::maths::vec3::Vec3;
    use crate::model::shapes::Shape;

    #[test]
    fn test_plane_intersect1() {
        let plane = super::Plane::new(Vec3::new(0., 0., 1.), Vec3::new(0., 0., 1.));
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 5);
        assert_eq!(plane.intersect(&ray), Some(Vec::from([1.0])));
    }

    #[test]
    fn test_plane_intersect2() {
        let plane = super::Plane::new(Vec3::new(0., 0., 1.), Vec3::new(0., 0., -1.));
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 5);
        assert_eq!(plane.intersect(&ray), Some(Vec::from([1.0])));
    }
}
