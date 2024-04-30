use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use super::Shape;

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
        let dot_product = r.get_dir().dot(&self.dir);
        if dot_product >0.  {
            return None;
        }
        let t = dist.dot(&self.dir) / dot_product;
        if (t > 0.0) {
            return Some(Vec::from([t]));
        }
        return None;
    }
    fn projection(&self, hit: &Hit) -> (i32, i32) {
        unimplemented!()
    }
    fn norm(&self, ray: &Vec3) -> Vec3 {
        // On doit aussi prendre on compte quand on tape de l'autre cote du plane
        if ray.dot(&self.dir) > 0. {
            return - self.dir.clone();
        }
		self.dir.clone()
	}
    fn as_plane(&self) -> Option<&Plane> { Some(self) }
}

impl Plane {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos }
    pub fn set_dir(&mut self, dir: Vec3) { self.dir = dir }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3) -> Plane{
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
        assert_eq!(plane.intersect(&ray), None);
    }

    #[test]
    fn test_plane_intersect2() {
        let plane = super::Plane::new(Vec3::new(0., 0., 1.), Vec3::new(0., 0., -1.));
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 5);
        assert_eq!(plane.intersect(&ray), Some(Vec::from([1.0])));
    }
}
