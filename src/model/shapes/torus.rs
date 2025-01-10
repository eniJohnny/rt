use std::f64::consts::PI;
use std::sync::{Arc, RwLock};
use super::shape::Shape;
use crate::model::element::Element;
use crate::model::materials::material::Projection;
use crate::model::maths::{hit, ray};
use crate::model::maths::vec2::Vec2;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};
use crate::model::scene::Scene;
use crate::ui::prefabs::vector_ui::get_vector_ui;
use crate::ui::ui::UI;
use crate::ui::uielement::{Category, UIElement};
use crate::ui::utils::misc::{ElemType, Property, Value};
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
            C - center of the torus (pos)
            A - axis of revolution of the torus (dir)
            R - major radius of the torus (radius)
            r - minor radius of the torus (radius2)
                
            P - ray
            P0 - origin of the ray
            P1 - direction of the ray
                P = P0 + P1t 
            y - distance along the axis of revolution
                y = A•(P0 – C + P1t ) = A•(P0 – C) + A•P1t
            D - radial position of the ray relative to the torus
                D = P0 + P1t – (C + Ay) = P0 – C + P1t – A• (P0 – C + P1t )A

            Q = P₀ – C
            u = A•Q
            v = A•P₁

            a = 1 - v²
            b = 2(Q•P₁ – uv)
            c = Q•Q – u²
            d = (Q•Q + R² – r²)

            A = 1
            B = 4Q•P₁
            C = 2d + B² * 0.25 – 4R²a
            D = Bd– 4R²b
            E = d² – 4R²c
        */

        let C = self.pos;
        let Ax = self.dir;
        let R = self.radius;
        let r = self.radius2;
        let P0 = ray.get_pos();
        let P1 = ray.get_dir();      

        let Q = P0 - C;
        let u = Ax.dot(&Q);
        let v = Ax.dot(&P1);

        let a = 1.0 - v.powi(2);
        let b = 2.0 * (Q.dot(&P1) - u * v);
        let c = Q.dot(&Q) - u.powi(2);
        let d = Q.dot(&Q) + R.powi(2) - r.powi(2);

        let A = 1.0;
        let B = 4.0 * Q.dot(&P1);
        let C = 2.0 * d + B.powi(2) * 0.25 - 4.0 * R.powi(2) * a;
        let D = B * d - 4.0 * R.powi(2) * b;
        let E = d.powi(2) - 4.0 * R.powi(2) * c;

        let roots = find_roots_quartic(A, B, C, D, E);

        return match roots {
            Roots::No(_)=> None,
            _ => {
                let len = roots.as_ref().len();
                let mut t = Vec::with_capacity(len);
                for i in 0..len {
                    if roots.as_ref()[i] > 0.0 {
                        t.push(roots.as_ref()[i]);
                    }
                }
                if t.len() > 0 {
                    return Some(t);
                }
                return None;
            }
        };
    }

    fn projection(&self, hit: &Hit) -> Projection {
        let vector = hit.pos() - &self.pos;
        let projected_v = vector - self.dir * self.dir.dot(&vector);
        let phi = projected_v.y().atan2(*projected_v.x());
        let r_center = projected_v.length() - self.radius;
        let theta = (*hit.pos().z() - *self.pos.z()).atan2(r_center);

        let phi_norm = (phi + PI) / (2.0 * PI);
        let theta_norm = (theta + PI) / (2.0 * PI);
        let u = phi_norm;
        let v = theta_norm;

        Projection {
            u,
            v,
            i: Vec3::new(1.0, 0.0, 0.0),
            j: Vec3::new(0.0, 1.0, 0.0),
            k: Vec3::new(0.0, 0.0, 1.0),
        }
    }

    fn norm(&self, hit_position: &Vec3) -> Vec3 {
        let P = hit_position;
        let C = self.pos;
        let A = self.dir;
        let R = self.radius;

        let y = (P - C).dot(&A);
        let D = (P - C) - A * y;
        let X = D * (1.0 / D.dot(&D).sqrt()) * R;

        let N = P - C - X;
        N.normalize()
    }

    fn as_torus(&self) -> Option<&Torus> {
        Some(self)
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn outer_intersect(&self, ray: &Ray, _displaced_factor: f64) -> Option<Vec<f64>> {
        self.intersect(ray)
    }

    fn intersect_displacement(&self, ray: &Ray, _element: &Element, _scene: &Scene) -> Option<Vec<f64>> {
        self.intersect(ray)
    }

    fn get_ui(&self, element: &Element, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Torus", "torus", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(torus) = element.shape().as_torus() {
            let id = element.id().clone();
            category.add_element(get_vector_ui(torus.pos.clone(), "Position", "pos", &ui.uisettings_mut(), 
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(torus) = elem.shape_mut().as_torus_mut() {
                        if let Value::Float(value) = value {
                            torus.pos.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(torus) = elem.shape_mut().as_torus_mut() {
                        if let Value::Float(value) = value {
                            torus.pos.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(torus) = elem.shape_mut().as_torus_mut() {
                        if let Value::Float(value) = value {
                            torus.pos.set_z(value);
                        }
                    }
                }),
                false, None, None));
            category.add_element(get_vector_ui(torus.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
                Box::new(move |_, value, scene, _ui| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(torus) = elem.shape_mut().as_torus_mut() {
                        if let Value::Float(value) = value {
                            torus.dir.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(torus) = elem.shape_mut().as_torus_mut() {
                        if let Value::Float(value) = value {
                            torus.dir.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, ui| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(torus) = elem.shape_mut().as_torus_mut() {
                        if let Value::Float(value) = value {
                            torus.dir.set_z(value);
                        }
                        torus.set_dir(torus.dir.normalize());
                        ui.set_dirty();
                    }
                }),
                false, Some(-1.), Some(1.)));

            category.add_element(UIElement::new(
                "Radius",
                "radius", 
                ElemType::Property(Property::new(
                    Value::Float(torus.radius), 
                    Box::new(move |_, value, scene, _: &mut UI| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(torus) = elem.shape_mut().as_torus_mut() {
                            if let Value::Float(value) = value {
                                torus.set_radius(value);
                            }
                        }
                        scene.set_dirty(true);
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));

            category.add_element(UIElement::new(
                "Half-witdth",
                "half_width", 
                ElemType::Property(Property::new(
                    Value::Float(torus.radius), 
                    Box::new(move |_, value, scene, _: &mut UI| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(torus) = elem.shape_mut().as_torus_mut() {
                            if let Value::Float(value) = value {
                                torus.set_radius(value);
                            }
                        }
                        scene.set_dirty(true);
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));
        }
        category
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
