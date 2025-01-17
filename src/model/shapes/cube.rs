use std::sync::{Arc, RwLock};

use super::{aabb::Aabb,shape::Shape};
use nalgebra::Matrix3;
use crate::{
    error, model::{
        element::Element, materials::material::Projection, maths::{hit::Hit, ray::Ray, vec3::Vec3}, scene::Scene
    }, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}
};

#[derive(Debug)]
pub struct Cube {
    pos: Vec3,
    dir: Vec3,
    width: f64,
    alpha: f64,
    beta: f64,
    gamma: f64,
    rotation: Matrix3<f64>,
    axis_aligned_cube: Aabb
}

impl Shape for Cube {
    fn distance(&self, _vec: &Vec3) -> f64 {
        unimplemented!()
    }

    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        let axis_aligned_cube = self.axis_aligned_cube.clone();
        let rotation = self.rotation;
        let pos = matrix3_vec3_mult(rotation.transpose(), *r.get_pos());
        let dir = matrix3_vec3_mult(rotation.transpose(), *r.get_dir());

        let mut ray = r.clone();
        ray.set_pos(pos);
        ray.set_dir(dir);
        let hit = axis_aligned_cube.intersect(&ray);
        
        hit
    }

    fn projection(&self, hit: &Hit) -> Projection {
        let axis_aligned_cube = self.axis_aligned_cube.clone();
        let rotation = self.rotation;
        let pos = matrix3_vec3_mult(rotation.transpose(), *hit.pos());

        let mut u: f64;
        let mut v: f64;
        if (*pos.z() - axis_aligned_cube.z_min() < 1e-6 && *pos.z() - axis_aligned_cube.z_min() > -1e-6)
        || (*pos.z() - axis_aligned_cube.z_max() < 1e-6 && *pos.z() - axis_aligned_cube.z_max() > -1e-6) {
            // Back or Front
            u = (pos.x() - axis_aligned_cube.x_min()) / (axis_aligned_cube.x_max() - axis_aligned_cube.x_min());
            v = (pos.y() - axis_aligned_cube.y_min()) / (axis_aligned_cube.y_max() - axis_aligned_cube.y_min());
        } else if (*pos.y() - axis_aligned_cube.y_min() < 1e-6 && *pos.y() - axis_aligned_cube.y_min() > -1e-6)
        || (*pos.y() - axis_aligned_cube.y_max() < 1e-6 && *pos.y() - axis_aligned_cube.y_max() > -1e-6) {
            // Top or Bottom
            u = (pos.x() - axis_aligned_cube.x_min()) / (axis_aligned_cube.x_max() - axis_aligned_cube.x_min());
            v = (pos.z() - axis_aligned_cube.z_min()) / (axis_aligned_cube.z_max() - axis_aligned_cube.z_min());
        } else if (*pos.x() - axis_aligned_cube.x_min() < 1e-6 && *pos.x() - axis_aligned_cube.x_min() > -1e-6)
        || (*pos.x() - axis_aligned_cube.x_max() < 1e-6 && *pos.x() - axis_aligned_cube.x_max() > -1e-6) {
            // Left or Right
            u = (pos.z() - axis_aligned_cube.z_min()) / (axis_aligned_cube.z_max() - axis_aligned_cube.z_min());
            v = (pos.y() - axis_aligned_cube.y_min()) / (axis_aligned_cube.y_max() - axis_aligned_cube.y_min());
        } else {
            error("Cube projection error");
            println!("self.dir: {:?}, hit.norm: {:?}", self.dir, hit.norm());
            u = 0.;
            v = 0.;
        }
        if u < 0. {
            u += 1.;
        }
        if v < 0. {
            v += 1.;
        }
        // println!("u: {}, v: {}", u, v);
        let constant_axis = get_constant_axis(&self.dir, &hit.norm().normalize());
        let i = hit.norm().normalize().cross(&constant_axis).normalize();
        let j = hit.norm().normalize().cross(&i).normalize();
        let k = hit.norm().normalize();
        // println!("Projection: u: {}, v: {}, i: {:?}, j: {:?}, k: {:?}", u, v, i, j, k);
        Projection { u, v, i, j, k }
    }

    fn norm(&self, hit_position: &Vec3) -> Vec3 {
        let axis_aligned_cube = self.axis_aligned_cube.clone();
        let rotation = self.rotation;
        let pos = matrix3_vec3_mult(rotation.transpose(), *hit_position);
        let norm = axis_aligned_cube.norm(&pos);
        
        matrix3_vec3_mult(rotation, norm)
    }

    fn as_cube(&self) -> Option<&Cube> {
        Some(self)
    }

    fn as_cube_mut(&mut self) -> Option<&mut Cube> {
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
        let mut category = UIElement::new("Cube", "cube", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(cube) = element.shape().as_cube() {
            let id = element.id().clone();
            category.add_element(get_vector_ui(cube.pos.clone(), "Position", "pos", &ui.uisettings_mut(), 
                Box::new(move |_, value, context, _| {
                    if let Some(scene) = context.get_active_scene() {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(cube) = elem.shape_mut().as_cube_mut() {
                            if let Value::Float(value) = value {
                                cube.pos.set_x(value);
                            }
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    if let Some(scene) = context.get_active_scene() {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(cube) = elem.shape_mut().as_cube_mut() {
                            if let Value::Float(value) = value {
                                cube.pos.set_y(value);
                            }
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    if let Some(scene) = context.get_active_scene() {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(cube) = elem.shape_mut().as_cube_mut() {
                            if let Value::Float(value) = value {
                                cube.pos.set_z(value);
                            }
                        }
                    }
                }),
                false, None, None));
            category.add_element(get_vector_ui(cube.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
                Box::new(move |_, value, context, _ui| {
                    if let Some(scene) = context.get_active_scene() {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(cube) = elem.shape_mut().as_cube_mut() {
                            if let Value::Float(value) = value {
                                cube.dir.set_x(value);
                            }
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    if let Some(scene) = context.get_active_scene() {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(cube) = elem.shape_mut().as_cube_mut() {
                            if let Value::Float(value) = value {
                                cube.dir.set_y(value);
                            }
                        }
                    }
                }),
                Box::new(move |_, value, context, ui| {
                    if let Some(scene) = context.get_active_scene() {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(cube) = elem.shape_mut().as_cube_mut() {
                            if let Value::Float(value) = value {
                                cube.dir.set_z(value);
                                cube.set_dir(cube.dir.normalize());
                                ui.set_dirty();
                            }
                        }
                    }
                }),
                false, Some(-1.), Some(1.)));

            category.add_element(UIElement::new(
                "Width",
                "width", 
                ElemType::Property(Property::new(
                    Value::Float(cube.width), 
                    Box::new(move |_, value, context, _: &mut UI| {
                        if let Some(scene) = context.get_active_scene() {
                            let mut scene = scene.write().unwrap();
                            let elem = scene.element_mut_by_id(id.clone()).unwrap();
                            if let Some(cube) = elem.shape_mut().as_cube_mut() {
                                if let Value::Float(value) = value {
                                    cube.set_width(value);
                                }
                            }
                            scene.set_dirty(true);
                        }
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));
        }
        category
    }
}

impl Cube {
    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn width(&self) -> f64 { self.width }
    pub fn alpha(&self) -> f64 { self.alpha }
    pub fn beta(&self) -> f64 { self.beta }
    pub fn gamma(&self) -> f64 { self.gamma }
    pub fn rotation(&self) -> Matrix3<f64> { self.rotation }
    pub fn axis_aligned_cube(&self) -> Aabb { self.axis_aligned_cube.clone() }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) { self.pos = pos; }
    pub fn set_dir(&mut self, dir: Vec3) { self.dir = dir; }
    pub fn set_width(&mut self, width: f64) {
        self.width = width;
        self.update();
    }
    pub fn set_alpha(&mut self, alpha: f64) { self.alpha = alpha; }
    pub fn set_beta(&mut self, beta: f64) { self.beta = beta; }
    pub fn set_gamma(&mut self, gamma: f64) { self.gamma = gamma; }
    pub fn set_rotation(&mut self, rotation: Matrix3<f64>) { self.rotation = rotation; }
    pub fn set_axis_aligned_cube(&mut self, axis_aligned_cube: Aabb) { self.axis_aligned_cube = axis_aligned_cube; }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, width: f64) -> Cube {
        let (mut alpha, mut beta, mut gamma) = (0., 0., 0.);

        if dir != Vec3::new(0.0, 1.0, 0.0) && dir != Vec3::new(0.0, -1.0, 0.0)
        && dir != Vec3::new(0.0, 0.0, 1.0) && dir != Vec3::new(0.0, 0.0, -1.0) 
        && dir != Vec3::new(1.0, 0.0, 0.0) && dir != Vec3::new(-1.0, 0.0, 0.0) {
            (alpha, beta, gamma) = (*get_angles(&dir).x(), *get_angles(&dir).y(), *get_angles(&dir).z());
        }

        let rotation = rotation_z(gamma) * rotation_y(beta) * rotation_x(alpha);
        let axis_aligned_cube = to_aabb(pos, width);
        self::Cube { pos, dir, width, alpha, beta, gamma, rotation, axis_aligned_cube }
    }

    // Methods
    pub fn clone(&self) -> Cube {
        let pos = Vec3::new(*self.pos.x(), *self.pos.y(), *self.pos.z());
        let dir = Vec3::new(*self.dir.x(), *self.dir.y(), *self.dir.z());
        self::Cube {
            pos: pos,
            dir: dir,
            width: self.width,
            alpha: self.alpha,
            beta: self.beta,
            gamma: self.gamma,
            rotation: self.rotation,
            axis_aligned_cube: self.axis_aligned_cube.clone()
        }
    }

    pub fn update(&mut self) {
        let (mut alpha, mut beta, mut gamma) = (0., 0., 0.);
        let dir = self.dir.clone();

        if dir != Vec3::new(0.0, 1.0, 0.0) && dir != Vec3::new(0.0, -1.0, 0.0)
        && dir != Vec3::new(0.0, 0.0, 1.0) && dir != Vec3::new(0.0, 0.0, -1.0) 
        && dir != Vec3::new(1.0, 0.0, 0.0) && dir != Vec3::new(-1.0, 0.0, 0.0) {
            (alpha, beta, gamma) = (*get_angles(&dir).x(), *get_angles(&dir).y(), *get_angles(&dir).z());
        }

        self.set_alpha(alpha);
        self.set_beta(beta);
        self.set_gamma(gamma);
        self.set_rotation(rotation_z(gamma) * rotation_y(beta) * rotation_x(alpha));
        self.set_axis_aligned_cube(to_aabb(self.pos, self.width));
    }
}

pub fn get_angles(dir: &Vec3) -> Vec3 {
    let axis;

    if dir == &Vec3::new(0.0, 1.0, 0.0) || dir == &Vec3::new(0.0, -1.0, 0.0) {
        axis = Vec3::new(1.0, 0.0, 0.0);
    } else {
        axis = Vec3::new(0.0, 1.0, 0.0);
    }

    let a_unit = dir.cross(&axis).normalize();
    let b_unit = a_unit.cross(&dir).normalize();
    let gamma = a_unit.y().atan2(*a_unit.x());
    let gamma_proj = (a_unit.x().powi(2) + a_unit.y().powi(2)).sqrt();
    let beta = a_unit.z().atan2(gamma_proj);
    let normal = a_unit.cross(&b_unit).normalize();
    let alpha = normal.y().atan2(*normal.z());

    Vec3::new(alpha, beta, gamma)
}

fn rotation_x (angle: f64) -> Matrix3<f64> {
    Matrix3::new(
        1.0, 0.0, 0.0,
        0.0, angle.cos(), -angle.sin(),
        0.0, angle.sin(), angle.cos()
    )
}

fn rotation_y (angle: f64) -> Matrix3<f64> {
    Matrix3::new(
        angle.cos(), 0.0, angle.sin(),
        0.0, 1.0, 0.0,
        -angle.sin(), 0.0, angle.cos()
    )
}

fn rotation_z (angle: f64) -> Matrix3<f64> {
    Matrix3::new(
        angle.cos(), -angle.sin(), 0.0,
        angle.sin(), angle.cos(), 0.0,
        0.0, 0.0, 1.0
    )
}

fn matrix3_vec3_mult(rotation: Matrix3<f64>, vec: Vec3) -> Vec3 {
    Vec3::new(
        rotation[(0, 0)] * vec.x() + rotation[(0, 1)] * vec.y() + rotation[(0, 2)] * vec.z(),
        rotation[(1, 0)] * vec.x() + rotation[(1, 1)] * vec.y() + rotation[(1, 2)] * vec.z(),
        rotation[(2, 0)] * vec.x() + rotation[(2, 1)] * vec.y() + rotation[(2, 2)] * vec.z()
    )
}

fn to_aabb(pos: Vec3, width: f64) -> Aabb {
    let (x_min, x_max, y_min, y_max, z_min, z_max) = (
        *pos.x() - width / 2.0, *pos.x() + width / 2.0,
        *pos.y() - width / 2.0, *pos.y() + width / 2.0,
        *pos.z() - width / 2.0, *pos.z() + width / 2.0
    );

    Aabb::new(x_min, x_max, y_min, y_max, z_min, z_max)
}

fn get_constant_axis(dir: &Vec3, norm: &Vec3) -> Vec3 {
    let (dirx, diry, dirz) = (*dir.x(), *dir.y(), *dir.z());
    let (normx, normy, normz) = (*norm.x(), *norm.y(), *norm.z());
    let any_x = (dirx.abs() == 1.0 && diry.abs() == 0.0 && dirz.abs() == 0.0) || (normx.abs() == 1.0 && normy.abs() == 0.0 && normz.abs() == 0.0);
    let any_y = (dirx.abs() == 0.0 && diry.abs() == 1.0 && dirz.abs() == 0.0) || (normx.abs() == 0.0 && normy.abs() == 1.0 && normz.abs() == 0.0);
    let any_z = (dirx.abs() == 0.0 && diry.abs() == 0.0 && dirz.abs() == 1.0) || (normx.abs() == 0.0 && normy.abs() == 0.0 && normz.abs() == 1.0);

    if any_x && !any_y {
        Vec3::new(0.0, 1.0, 0.0)
    } else if any_x && !any_z {
        Vec3::new(0.0, 0.0, 1.0)
    } else if any_y && !any_x {
        Vec3::new(1.0, 0.0, 0.0)
    } else if any_y && !any_z {
        Vec3::new(0.0, 0.0, 1.0)
    } else if any_z && !any_x {
        Vec3::new(1.0, 0.0, 0.0)
    } else if any_z && !any_y {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    }
}