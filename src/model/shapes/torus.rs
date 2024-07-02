use super::Shape;
use crate::model::materials::material::Projection;
use crate::model::maths::ray;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use roots::{find_roots_quartic, Roots};

#[derive(Debug)]
pub struct Torus {
    pos: Vec3,
    dir: Vec3,
    radius: f64, // Distance from center of torus to center of its tube
    radius2: f64, // Radius of the tube
}


unsafe impl Send for Torus {}

impl Shape for Torus {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }

    fn intersect(&self, ray: &Ray) -> Option<Vec<f64>> {
        /*
            (x² + y² + z² + R² − r²)² − 4 * R² * (x² + y²) = 0
            P(t) = O + t * D where O is the origin of the ray and D is the direction of the ray
            ((Oˣ ​+ tDˣ​)² + (Oʸ​ + tDʸ​)² + (Oᶻ​ + tDᶻ​)² + R² − r²)² − 4 * R² * ((Oˣ​ + tDˣ​)² + (Oʸ​ + tDʸ​)²) = 0
            At⁴ + Bt³ + Ct² + Dt + E = 0
        */
        let ray_origin = ray.get_pos();
        let ray_direction = ray.get_dir().normalize();
        let torus_center = self.pos();
        let torus_direction = self.dir().normalize();
        let major_radius = self.radius;
        let minor_radius = self.radius2;
        
        let rotation_axis = ray_direction.cross(&torus_direction);
        let rotation_angle = ray_direction.angle(&torus_direction);

        let rotated_ray_origin = ray_origin;
        let rotated_ray_direction = ray_direction.rotate_from_axis_angle(rotation_angle, &rotation_axis);

        let ox = *rotated_ray_origin.x() - torus_center.x();
        let oy = *rotated_ray_origin.y() - torus_center.y();
        let oz = *rotated_ray_origin.z() - torus_center.z();
        let dx = *rotated_ray_direction.x();
        let dy = *rotated_ray_direction.y();
        let dz = *rotated_ray_direction.z();
        let r = major_radius;
        let R = minor_radius;
        let r2 = r * r;
        let R2 = R * R;

        let a4 = dx * dx + dy * dy + dz * dz;
        let a3 = 4.0 * (ox * dx + oy * dy + oz * dz);
        let a2 = 2.0 * (ox * ox + oy * oy + oz * oz + r2 - R2 + r * (dx * dx + dy * dy + dz * dz));
        let a1 = 4.0 * r * (ox * dx + oy * dy + oz * dz);
        let a0 = ox * ox + oy * oy + oz * oz + r2 - R2;

        let roots = find_roots_quartic(a4, a3, a2, a1, a0);

        // println!("\n\nRay Origin: {:?}", ray.get_pos());
        // println!("Ray Direction: {:?}", ray.get_dir());
        // println!("Torus Center: {:?}", self.pos());
        // println!("Torus Direction: {:?}", self.dir());
        // println!("Rotation Axis: {:?}", rotation_axis);
        // println!("Rotation Angle: {:?}", rotation_angle);
        // println!("Rotated Ray Origin: {:?}", rotated_ray_origin);
        // println!("Rotated Ray Direction: {:?}", rotated_ray_direction);
        // println!("Coefficients: a={}, b={}, c={}, d={}, e={}", a, b, c, d, e);
        // println!("Roots: {:?}", roots);

        return match roots {
            Roots::No(_) => None,
            Roots::One([t]) => Some(vec![t]),
            Roots::Two([t1, t2]) => Some(vec![t1, t2]),
            Roots::Three([t1, t2, t3]) => Some(vec![t1, t2, t3]),
            Roots::Four([t1, t2, t3, t4]) => Some(vec![t1, t2, t3, t4]),
        }
    }

    fn projection(&self, hit: &Hit) -> Projection {
        Projection::default()
    }

    fn norm(&self, hit_position: &Vec3, ray_dir: &Vec3) -> Vec3 {
        let theta = hit_position.y().atan2(*hit_position.x());
        let center = Vec3::new(
            self.radius * theta.cos(),
            self.radius * theta.sin(),
            0.0,
        );

        let normal = hit_position - center;
        normal.normalize()
    }

    fn as_torus(&self) -> Option<&Torus> {
        Some(self)
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
    }
}

impl Torus {
    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, radius2: f64) -> Torus {
        Torus {
            pos,
            dir,
            radius,
            radius2,
        }
    }

    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn radius2(&self) -> f64 { self.radius2 }

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
    pub fn set_radius2(&mut self, radius2: f64) {
        self.radius2 = radius2
    }

    // Methods
}