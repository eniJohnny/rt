use std::f64::consts::PI;

use crate::{
    model::maths::{quaternion::Quaternion, vec3::Vec3}, ui::{ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}, LOOK_STEP, SCREEN_HEIGHT, SCREEN_WIDTH, STEP
};

#[derive(Debug)]
pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    fov_deg: f64,
    fov: f64,
    vfov: f64,
    u: Vec3,
    v: Vec3,
    q_up: Quaternion,
    q_down: Quaternion,
    q_left: Quaternion,
    q_right: Quaternion,
}

impl Camera {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn fov(&self) -> f64 {
        self.fov
    }
    pub fn vfov(&self) -> f64 {
        self.vfov
    }
    pub fn u(&self) -> &Vec3 {
        &self.u
    }
    pub fn v(&self) -> &Vec3 {
        &self.v
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.u = Vec3::new(*dir.z(), 0., -*dir.x()).normalize();
        self.v = dir.cross(&self.u).normalize();
        self.dir = dir.normalize();
    }
    pub fn set_fov(&mut self, fov: f64) {
        self.fov = fov;
        self.vfov = fov * SCREEN_HEIGHT as f64 / SCREEN_WIDTH as f64;
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, fov: f64) -> Camera {
		let u;
		if *dir.x() == 0. && *dir.z() == 0. {
			u = Vec3::new(1., 0., 0.);
		}
		else {
			u = Vec3::new(*dir.z(), 0., -*dir.x()).normalize();
		}
        let fov_deg = fov;
        let fov = fov_deg * PI / 180.;
		let v = dir.cross(&u).normalize();
        let vfov = fov * SCREEN_HEIGHT as f64 / SCREEN_WIDTH as f64;
        let q_up = Quaternion::new_from_axis_angle(&u, -LOOK_STEP);
        let q_down = Quaternion::new_from_axis_angle(&u, LOOK_STEP);
        let q_left = Quaternion::new_from_axis_angle(&Vec3::new(0., 1., 0.), -LOOK_STEP);
        let q_right = Quaternion::new_from_axis_angle(&Vec3::new(0., 1., 0.), LOOK_STEP);
        self::Camera {
            pos,
            dir,
            fov_deg,
            fov,
            u,
            v,
            vfov,
            q_left,
            q_right,
            q_up,
            q_down,
        }
    }

    pub fn default() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 0.0, 0.0),
            fov_deg: 0.,
            fov: 0.,
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            vfov: 0.,
            q_up: Quaternion::new_from_axis_angle(&Vec3::new(1., 0., 0.), -LOOK_STEP),
            q_down: Quaternion::new_from_axis_angle(&Vec3::new(1., 0., 0.), LOOK_STEP),
            q_left: Quaternion::new_from_axis_angle(&Vec3::new(0., 1., 0.), -LOOK_STEP),
            q_right: Quaternion::new_from_axis_angle(&Vec3::new(0., 1., 0.), LOOK_STEP),
        }
    }

    // Movement methods
    pub fn move_forward(&mut self) {
        self.pos += self.dir() * STEP;
    }
    pub fn move_backward(&mut self) {
        self.pos -= self.dir() * STEP;
    }
    pub fn move_left(&mut self) {
        self.pos -= self.u() * STEP;
    }
    pub fn move_right(&mut self) {
        self.pos += self.u() * STEP;
    }
    pub fn move_up(&mut self) {
        self.pos -= Vec3::new(0., 1., 0.) * STEP;
    }
    pub fn move_down(&mut self) {
        self.pos += Vec3::new(0., 1., 0.) * STEP;
    }
    pub fn look_up(&mut self) {
        self.set_dir(self.q_up.rotate(&self.dir()));
	}
    pub fn look_down(&mut self) {
        self.set_dir(self.q_down.rotate(&self.dir()));
    }
    pub fn look_left(&mut self) {
        self.set_dir(self.q_left.rotate(&self.dir()));
		self.q_up = Quaternion::new_from_axis_angle(&self.u(), -LOOK_STEP);
		self.q_down = Quaternion::new_from_axis_angle(&self.u(), LOOK_STEP);
    }
    pub fn look_right(&mut self) {
        self.set_dir(self.q_right.rotate(&self.dir()));
		self.q_up = Quaternion::new_from_axis_angle(&self.u(), -LOOK_STEP);
		self.q_down = Quaternion::new_from_axis_angle(&self.u(), LOOK_STEP);
    }
    pub fn debug_print(&self) {
        println!();
        println!("pos: {:.2} {:.2} {:.2}", self.pos.x(), self.pos.y(), self.pos.z());
        println!("dir: {:.2} {:.2} {:.2}", self.dir.x(), self.dir.y(), self.dir.z());
        println!("u: {:.2} {:.2} {:.2}", self.u.x(), self.u.y(), self.u.z());
        println!("v: {:.2} {:.2} {:.2}", self.v.x(), self.v.y(), self.v.z());
    }

    pub fn get_ui(&self, ui: &mut UI) -> UIElement {
        let mut category = UIElement::new("Camera", "camera", ElemType::Category(Category::collapsed()), ui.uisettings());
        // let pos = get_vector_ui(self.pos, "Position", "pos", ui.uisettings(), 
        // Box::new(move |_, value, context, _| {
        //     if let Some(scene) = context.get_active_scene() {
        //         let mut scene = scene.write().unwrap();
        //         let camera = scene.camera_mut();
        //         if let Value::Float(value) = value {
        //             camera.pos.set_x(value);
        //         }
        //     }
        // }),
        // Box::new(move |_, value, context, _| {
        //     if let Some(scene) = context.get_active_scene() {
        //         let mut scene = scene.write().unwrap();
        //         let camera = scene.camera_mut();
        //         if let Value::Float(value) = value {
        //             camera.pos.set_y(value);
        //         }
        //     }
        // }),
        // Box::new(move |_, value, context, _| {
        //     if let Some(scene) = context.get_active_scene() {
        //         let mut scene = scene.write().unwrap();
        //         let camera = scene.camera_mut();
        //         if let Value::Float(value) = value {
        //             camera.pos.set_z(value);
        //         }
        //     }
        // }), false, None, None);

        // let dir = get_vector_ui(self.dir, "Direction", "dir", ui.uisettings(), 
        // Box::new(move |_, value, context, _| {
        //     if let Some(scene) = context.get_active_scene() {
        //         let mut scene = scene.write().unwrap();
        //         let camera = scene.camera_mut();
        //         if let Value::Float(value) = value {
        //             camera.dir.set_x(value);
        //             camera.dir = camera.dir.normalize();
        //         }
        //     }
        // }),
        // Box::new(move |_, value, context, _| {
        //     if let Some(scene) = context.get_active_scene() {
        //         let mut scene = scene.write().unwrap();
        //         let camera = scene.camera_mut();
        //         if let Value::Float(value) = value {
        //             camera.dir.set_y(value);
        //             camera.dir = camera.dir.normalize();
        //         }
        //     }
        // }),
        // Box::new(move |_, value, context, _| {
        //     if let Some(scene) = context.get_active_scene() {
        //         let mut scene = scene.write().unwrap();
        //         let camera = scene.camera_mut();
        //         if let Value::Float(value) = value {
        //             camera.dir.set_z(value);
        //             camera.dir = camera.dir.normalize();
        //         }
        //     }
        // }), false, None, None);

        let fov = UIElement::new("FOV", "fov", ElemType::Property(Property::new(Value::Float(self.fov_deg),
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene = scene.write().unwrap();
                let camera = scene.camera_mut();
                if let Value::Float(value) = value {
                    camera.fov_deg = value;
                    camera.fov = value * PI / 180.;
                }
            }
        }),
        Box::new(move |value, _, _| {
            if let Value::Float(value) = value {
                if *value < 0. {
                    return Err("The value should not be inferior to 0".to_string());
                }
                if *value > 360. {
                    return Err("The value should not be superior to 360".to_string());
                }
            }
            Ok(())
        }), ui.uisettings())), ui.uisettings());

        // category.add_element(pos);
        // category.add_element(dir);
        category.add_element(fov);
        category
    }
}
