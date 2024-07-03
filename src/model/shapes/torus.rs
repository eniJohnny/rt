use super::Shape;
use crate::model::materials::material::Projection;
use crate::model::maths::{hit, ray};
use crate::model::maths::vec2::Vec2;
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
        let ro = ray.get_pos();
        let rd = ray.get_dir();

        // let cos_theta = ray.get_dir().dot(&self.dir);
        // let ro = ray.get_pos() - self.pos;
        // let rd = ray.get_dir() - self.dir * cos_theta;


        let mut po = 1.0;
        let Ra2 = self.radius * self.radius;
        let ra2 = self.radius2 * self.radius2;
        let m = ro.dot(&ro);
        let n = rd.dot(&ro);
        let k = (m + Ra2 - ra2) / 2.0;
        let mut k3 = n;
        let mut k2 = n * n - Ra2 * rd.dot(&rd) + k;
        let mut k1 = n * k - Ra2 * ro.dot(&rd);
        let mut k0 = k * k - Ra2 * ro.dot(&ro);

        if (k3 * (k3 * k3 - k2) + k1).abs() - f64::EPSILON < 0.0 {
            (k1, k3) = (k3, k1);

            po = -1.0;
            k0 = 1.0 / k0;
            k1 *= k0;
            k2 *= k0;
            k3 *= k0;
        }

        let mut c2 = k2 * 2.0 - 3.0 * k3 * k3;
        let mut c1 = k3 * (k3 * k3 - k2) + k1;
        let mut c0 = k3 * (k3 * (c2 + 2.0 * k2) - 8.0 * k1) + 4.0 * k0;

        c2 /= 3.0;
        c1 *= 2.0;
        c0 /= 3.0;

        let Q = c2 * c2 + c0;
        let R = c2 * c2 * c2 - 3.0 * c2 * c0 + c1 * c1;
        let mut h = R * R - Q * Q * Q;
        
        if h >= 0.0 {
            h = h.sqrt();
            let v = (R + h).cbrt();
            let u = (R - h).cbrt();
            let s = Vec2::new((v + u) + 4.0 * c2, (v - u) * 3.0_f64.sqrt());
            let y = (0.5 * (s.length() + s.x())).sqrt();
            let x = 0.5 * s.y() / y;
            let r = 2.0 * c1 / (x * x + y * y);
            let t1 = if po < 0.0 {x - r - k3} else {2.0 / (x - r - k3)};
            let t2 = if po < 0.0 {-x - r - k3} else {2.0 / (-x - r - k3)};
            let mut t = f64::MAX;

            if t1 > 0.0 {
                t = t1;
            }
            if t2 > 0.0 && t2 < t {
                t = t2;
            }
            if t != f64::MAX {
                return Some(vec![t]);
            }
            return None;
        }

        let sQ = Q.sqrt();
        let w = sQ * ((-R / (sQ * Q).acos()) / 3.0).cos();
        let d2 = -(w + c2);

        if  d2 < 0.0 {
            return None;
        }

        let d1 = d2.sqrt();
        let h2 = (w - 2.0 * c2 - c1 / d1).sqrt();
        let h1 = (w - 2.0 * c2 + c1 / d1).sqrt();
        let mut t = f64::MAX;


        let tx = vec![
            if po < 0.0 {-d1 - h1 - k3} else {2.0 / (-d1 - h1 - k3)},
            if po < 0.0 {-d1 + h1 - k3} else {2.0 / (-d1 + h1 - k3)},
            if po < 0.0 {d1 - h2 - k3} else {2.0 / (d1 - h2 - k3)},
            if po < 0.0 {d1 + h2 - k3} else {2.0 / (d1 + h2 - k3)},
        ];
        
        for i in 0..4 {
            if tx[i] > 0.0 && tx[i] < t {
                t = tx[i];
            }
        }

        if t != f64::MAX {
            return Some(vec![t]);
        }
        None
    }

    fn projection(&self, hit: &Hit) -> Projection {
        Projection::default()
    }

    fn norm(&self, hit_position: &Vec3, ray_dir: &Vec3) -> Vec3 {
        let x2 = self.radius * self.radius;
        let y2 = self.radius2 * self.radius2;
        
        let a = hit_position.dot(&hit_position);
        let b = y2;
        let c = x2 * Vec3::new(1.0, 1.0, -1.0);
        let d = c - (a - b);

        (hit_position * d).normalize()
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
