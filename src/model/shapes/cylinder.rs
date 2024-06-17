use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::shapes::plane::Plane;
use super::Shape;

#[derive(Debug)]
pub struct Cylinder {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
    height: f64,
    plane: [Plane; 2]
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
        if (delta < 0.0) {
            return None;
        }
        delta = delta.sqrt();

        //On calcule la distance avec les deux intersections
        let mut intersections = Vec::from([(-b - delta) / (2.0 * a), (-b + delta) / (2.0 * a)]);

        //On vérifie si les intersections sont bien sur le cylindre (delimité par la hauteur)
        let projection1 = (intersections[0] * r.get_dir() + r.get_pos() - &self.pos).dot(&self.dir);
        let projection2 = (intersections[1] * r.get_dir() + r.get_pos() - &self.pos).dot(&self.dir);

        if (projection2 < 0.0 || projection2 > self.height) || intersections[1] < 0. || delta == 0.{
            intersections.remove(1);
        }
        if (projection1 < 0.0 ||  projection1 > self.height) || intersections[0] < 0.{
            intersections.remove(0);
        }

        //On vérifie si le rayon intersecte les plans du cylindre
        match self.plane[0].intersect(r) {
            Some(intersection) => {
                let position = intersection[0]  * r.get_dir() + r.get_pos();
                let distance1 = (&position - &self.pos).length();
                let distance2 = (position - (&self.pos + &self.dir * &self.height)).length();
                if distance1 < self.radius || distance2 < self.radius {
                    intersections.push(intersection[0]);
                }
            },
            _ => {
                // Ce bloc sera exécuté pour tous les autres cas, y compris None
            }
        }
        match self.plane[1].intersect(r) {
            Some(intersection) => {
                let position = intersection[0]  * r.get_dir() + r.get_pos();
                let distance1 = (&position - &self.pos).length();
                let distance2 = (position - (&self.pos + &self.dir * &self.height)).length();
                if distance1 < self.radius || distance2 < self.radius {
                    intersections.push(intersection[0]);
                }
            },
            _ => {
                // Ce bloc sera exécuté pour tous les autres cas, y compris None
            }
        }
        if intersections.len() == 0 {
            return None;
        }

        //On retourne les intersections triées
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        return Some(intersections);
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
		let cylinder_i = self.dir.cross(&constant_axis).normalize();
		let cylinder_j = self.dir.cross(&cylinder_i).normalize();
		
		let constant_axis: Vec3;
		if *hit.norm() == Vec3::new(0., 0., 1.) {
			constant_axis = Vec3::new(0., 1., 0.);
		} else {
			constant_axis = Vec3::new(0., 0., 1.);
		}
		projection.i = hit.norm().cross(&constant_axis).normalize();
		projection.j = hit.norm().cross(&projection.i).normalize();
		projection.k = hit.norm().clone();
		let i_component: f64 = cam_hit.dot(&cylinder_i);
		let j_component: f64 = cam_hit.dot(&cylinder_j);
		let ij_hit: Vec3 = (i_component * &cylinder_i + j_component * &cylinder_j).normalize(); 
		let is_front: bool = ij_hit.dot(&cylinder_j) > 0.;
		if is_front {
			projection.u = (ij_hit.dot(&cylinder_i) + 1.) / 4.;
		} else {
			projection.u = 1. - (ij_hit.dot(&cylinder_i) + 1.) / 4.;
		}
        if level > -0.000001 && level < 0.000001 { // Bottom Cap
			projection.v = (hit.pos() - &self.pos).length() / total_height;
        } else if level > self.height - 0.000001 && level < self.height + 0.000001 { // Top Cap
			projection.v = (total_height - (hit.pos() - &self.pos - &self.dir * &self.height).length()) / total_height;
        } else { // Cylinder
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
        if coef > self.height - 0.000001 && coef < self.height + 0.000001{
            return self.plane[1].norm(hit, ray_dir);
        }

        return (hit - (&self.pos + &projection)).normalize();
    }
    fn as_cylinder(&self) -> Option<&Cylinder> { Some(self) }
}

impl Cylinder {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn height(&self) -> f64 { self.height }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos }
    pub fn set_dir(&mut self, dir: Vec3) { self.dir = dir }
    pub fn set_radius(&mut self, radius: f64) { self.radius = radius }
    pub fn set_height(&mut self, height: f64) { self.height = height }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, height: f64) -> Cylinder {
        let plane1 = Plane::new(pos.clone(), -dir.clone());
        let plane2 = Plane::new(pos.clone() + dir.clone() * height, dir.clone());
        self::Cylinder { pos, dir, radius, height, plane: [plane1, plane2] }
    }

}