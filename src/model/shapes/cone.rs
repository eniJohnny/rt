use super::Shape;
use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::shapes::plane::Plane;

#[derive(Debug)]
pub struct Cone {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
    height: f64,
    cos_powed: f64,
    plane: Plane,
}

unsafe impl Send for Cone {}

impl Shape for Cone {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        //d:    direction du rayon
        //co:   vecteur entre la postion du cone et le point d'origine du rayon
        //v:    vecteur directeur du cone
        //abc:  les coefficients
        let dv = r.get_dir().dot(&self.dir);
        let co = r.get_pos() - &self.pos;
        let cov = co.dot(&self.dir);
        let a = dv.powi(2) - &self.cos_powed;
        let b = 2.0 * ((dv * cov) - co.dot(&r.get_dir()) * &self.cos_powed);
        let c = cov.powi(2) - co.dot(&(co)) * &self.cos_powed;

        let mut delta = b.powi(2) - 4.0 * a * c;

        if delta < 0.0 {
            return None;
        }
        delta = delta.sqrt();

        //On calcule la distance avec les deux intersections
        let mut intersections = Vec::from([(-b - delta) / (2.0 * a), (-b + delta) / (2.0 * a)]);

        //On vérifie si les intersections sont bien sur le cone (delimité par la hauteur)
        let projection1 = (intersections[0] * r.get_dir() + r.get_pos() - &self.pos).dot(&self.dir);
        let projection2 = (intersections[1] * r.get_dir() + r.get_pos() - &self.pos).dot(&self.dir);

        if (projection2 < 0.0 || projection2 > self.height) || intersections[1] < 0. || delta == 0.{
            intersections.remove(1);
        }
        if (projection1 < 0.0 ||  projection1 > self.height) || intersections[0] < 0.{
            intersections.remove(0);
        }

        //On vérifie si le rayon intersecte le plan du cone
        match self.plane.intersect(r) {
            Some(intersection) => {
                let position = intersection[0]  * r.get_dir() + r.get_pos();
                let distance = (position - (&self.pos + &self.dir * &self.height)).length();
                if distance < self.radius{
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

        //On trie et on retourne les intersections
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        return Some(intersections);
    }

    fn projection(&self, hit: &Hit) -> Projection {
		let mut projection: Projection = Projection::default();
		let constant_axis: Vec3;
		if *hit.norm() == Vec3::new(0., 0., 1.) {
			constant_axis = Vec3::new(0., 1., 0.);
		} else {
			constant_axis = Vec3::new(0., 0., 1.);
		}
		projection.i = hit.norm().cross(&constant_axis).normalize();
		projection.j = hit.norm().cross(&projection.i).normalize();
		projection.k = hit.norm().clone();

        let point_to_hit = hit.pos() - &self.pos;
        let level = point_to_hit.dot(&self.dir);
		
		let slope_lenght = (self.height.powi(2) + self.radius.powi(2)).sqrt();
		let total_height = slope_lenght + self.radius;

		let constant_axis: Vec3;
		if self.dir == Vec3::new(0., 0., 1.) {
			constant_axis = Vec3::new(0., 1., 0.);
		} else {
			constant_axis = Vec3::new(0., 0., 1.);
		}
		let cylinder_i = self.dir.cross(&constant_axis).normalize();
		let cylinder_j = self.dir.cross(&cylinder_i).normalize();
		
		let i_component: f64 = point_to_hit.dot(&cylinder_i);
		let j_component: f64 = point_to_hit.dot(&cylinder_j);
		let ij_hit: Vec3 = (i_component * &cylinder_i + j_component * &cylinder_j).normalize(); 
		let is_front: bool = ij_hit.dot(&cylinder_j) > 0.;
		if is_front {
			projection.u = (ij_hit.dot(&cylinder_i) + 1.) / 4.;
		} else {
			projection.u = 1. - (ij_hit.dot(&cylinder_i) + 1.) / 4.;
		}
        if level > self.height - 0.000001 && level < self.height + 0.000001 { // Cap
			projection.v = (total_height - (point_to_hit - &self.dir * &self.height).length()) / total_height;
        } else { // Cone
			projection.v = point_to_hit.length() / total_height;
		}
		projection
	}

    fn norm(&self, hit_position: &Vec3, ray_dir: &Vec3) -> Vec3 {
        let pc = hit_position - &self.pos;
        let coef = pc.dot(&self.dir) / pc.dot(&pc);
        let projection = &pc * coef;

        if pc.dot(&self.dir) == self.height {
            return self.plane.norm(hit_position, ray_dir);
        }

        return ((&self.pos + &projection) -(&self.pos + &self.dir * &self.height)).normalize();
    }
    fn as_cone(&self) -> Option<&Cone> {
        Some(self)
    }
}

impl Cone {
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
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, height: f64) -> Cone {
        let cos_powed = (radius / height).atan().cos().powi(2);
        let plane = Plane::new(&pos + &dir * height, dir.clone());
        self::Cone {
            pos,
            dir,
            radius,
            height,
            cos_powed,
            plane
        }
    }
}
