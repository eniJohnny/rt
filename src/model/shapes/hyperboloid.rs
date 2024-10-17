use super::Shape;
use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};

#[derive(Debug)]
pub struct Hyperboloid {
    pos: Vec3,
    z_shift: f64,
}

impl Shape for Hyperboloid {
    fn distance(&self, vec: &Vec3) -> f64 {
        unimplemented!()
    }

    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        // intersection rayon/hyperboloid
        // a = D.z*D.z-D.x*D.x-D.y*D.y
        // b = 2.0*(C.z*D.z - C.x*D.x - C.y*D.y)
        // c = C.z*C.z + k - C.x*C.x - C.y*C.y
        // f(P) = P.z^2 + k - P.x^2 - P.y^2
        // z and y are inverted because of the coordinate system

        let (cx, cy, cz) = (r.get_pos().x() - self.pos.x(), r.get_pos().z() - self.pos.z(), r.get_pos().y() - self.pos.y());
        let (dx, dy, dz) = (*r.get_dir().x(), *r.get_dir().z(), *r.get_dir().y());

        let a = dz * dz - dx * dx - dy * dy;
        let b = 2.0 * (cz * dz - cx * dx - cy * dy);
        let c = cz * cz + self.z_shift - cx * cx - cy * cy;

        let delta = b * b - 4.0 * a * c;
        let mut t: Vec<f64> = Vec::new();

        if delta < 0.0 {
            return None;
        } else if delta == 0.0 {
            t.push((-b + delta.sqrt()) / (2.0 * a));
        } else {
            t.push((-b + delta.sqrt()) / (2.0 * a));
            t.push((-b - delta.sqrt()) / (2.0 * a));
        }

        t.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Some(t)
    }

    fn projection(&self, hit: &Hit) -> Projection {
        let mut projection: Projection = Projection::default();
        let scale = 4.;

        let constant_axis: Vec3;
        if *hit.norm() == Vec3::new(0., 1., 0.) {
            constant_axis = Vec3::new(1., 0., 0.);
        } else {
            constant_axis = Vec3::new(0., 1., 0.);
        }
        projection.i = hit.norm().cross(&constant_axis).normalize();
        projection.j = hit.norm().cross(&projection.i).normalize();
        projection.k = hit.norm().clone();
        
        let normalized_pos = hit.pos().normalize();
        let (x, y, z) = (*normalized_pos.x(), *normalized_pos.y(), *normalized_pos.z());

        if x.abs() > y.abs() && x.abs() > z.abs() {
            projection.u = z;
            projection.v = y;
        } else if y.abs() > x.abs() && y.abs() > z.abs() {
            projection.u = x;
            projection.v = z;
        } else {
            projection.u = x;
            projection.v = y;
        }

        if projection.u < 0. {
            projection.u += 1.;
        }
        if projection.v < 0. {
            projection.v += 1.;
        }

        projection
    }

    fn norm(&self, hit_position: &Vec3, ray_dir: &Vec3) -> Vec3 {
        let (x, y, z) = (*hit_position.x(), *hit_position.y(), *hit_position.z());

        Vec3::new(-2.0 * x, -2.0 * y, 2.0 * z).normalize()
    }

    fn as_hyperboloid(&self) -> Option<&Hyperboloid> {
        Some(self)
    }

    fn pos(&self) -> &Vec3 { &self.pos }

    fn outer_intersect(&self, ray: &Ray, displaced_factor: f64) -> Option<Vec<f64>> {
        self.intersect(ray)
    }

    fn intersect_displacement(&self, ray: &Ray, element: &crate::model::Element, scene: &crate::model::scene::Scene) -> Option<Vec<f64>> {
        self.intersect(ray)
    }

    fn get_ui(&self, element: &crate::model::Element, ui: &mut crate::ui::ui::UI, scene: &std::sync::Arc<std::sync::RwLock<crate::model::scene::Scene>>) -> crate::ui::uielement::UIElement {
        todo!()
    }

}

impl Hyperboloid {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn z_shift(&self) -> f64 { self.z_shift }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos; }
    pub fn set_dir(&mut self, z_shift: f64) { self.z_shift = z_shift }

    // Constructor
    pub fn new(pos: Vec3, z_shift: f64) -> Hyperboloid {
        self::Hyperboloid { pos, z_shift }
    }

    // Methods
    pub fn clone(&self) -> Hyperboloid {
        let pos = Vec3::new(*self.pos.x(), *self.pos.y(), *self.pos.z());
        self::Hyperboloid {
            pos: pos,
            z_shift: self.z_shift,
        }
    }

}