use super::Shape;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::shapes::plane::Plane;

#[derive(Debug)]
pub struct Triangle {
    pub(crate) a: Vec3,
    pub(crate) b: Vec3,
    pub(crate) c: Vec3,
}

#[derive(Debug)]
pub struct Polygon {
    triangles: Vec<Triangle>,
    dir: Vec3,
    plane: Plane,
}

unsafe impl Send for Polygon {}

impl Shape for Polygon {
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
        if self.inside_triangles(&p) {
            Some(Vec::from([intersection]))
        } else {
            None
        }
    }

    fn projection(&self, hit: &Hit) -> (i32, i32) {
        unimplemented!()
    }
    fn norm(&self, hit_position: &Vec3, ray_dir: &Vec3) -> Vec3 {
        return self.plane.norm(hit_position, ray_dir);
    }
}

impl Polygon {
    // Accessors
    pub fn plane(&self) -> &Plane { &self.plane }

    // Mutators
    pub fn set_plane(&mut self, plane: Plane) { self.plane = plane }

    // Constructor
    pub fn new(triangles: Vec<Triangle>) -> Polygon {
        let triangle_ref = &triangles[0];
        let dir =(triangle_ref.b.clone() - &triangle_ref.a).cross(&(triangle_ref.c.clone() - &triangle_ref.a)).normalize();
        let plane = Plane::new(triangle_ref.a.clone(), dir.clone());
        Polygon {
            triangles,
            dir,
            plane,
        }
    }

    pub fn inside_triangles(&self, p: &Vec3) -> bool {

        let mut is_inside = false;

        self.triangles.iter().for_each(|triangle| {
            let pa = (&triangle.b - &triangle.a).cross(&(p - &triangle.a)).dot(&-&self.dir);
            let pb = (&triangle.c - &triangle.b).cross(&(p - &triangle.b)).dot(&-&self.dir);
            let pc = (&triangle.a - &triangle.c).cross(&(p - &triangle.c)).dot(&-&self.dir);
            if pa < 0. && pb < 0. && pc < 0. {
                is_inside = true;
            }
        });

        return is_inside;
    }
}


#[cfg(test)]
mod tests {
    use crate::model::maths::ray::Ray;
    use crate::model::maths::vec3::Vec3;
    use crate::model::shapes::polygon::{Polygon, Triangle};
    use crate::model::shapes::Shape;

    #[test]
    fn test_intersect1() {
        let s1: Polygon = Polygon::new(Vec::from([Triangle{
            a: Vec3::new(0., 2., -2.),
            b: Vec3::new(2., -2., -2.),
            c: Vec3::new(-2., -2., -2.)
        }]));
        let r1: Ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., -1.), 5);
        assert_eq!(s1.intersect(&r1), Some(Vec::from([2.])));
    }

    #[test]
    fn test_intersect2() {
        let s1: Polygon = Polygon::new(Vec::from([Triangle{
            a: Vec3::new(0., 2., 2.),
            b: Vec3::new(2., -2., 2.),
            c: Vec3::new(-2., -2., 2.)
        }]));
        let r1: Ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.), 5);
        assert_eq!(s1.intersect(&r1), Some(Vec::from([2.])));
    }
}
