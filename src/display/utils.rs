use winit::event::VirtualKeyCode;
use crate::model::{maths::vec3::Vec3, objects::camera::Camera, scene::Scene, shapes::{sphere, Shape}};

pub fn move_camera(camera: &mut Camera, c: Option<VirtualKeyCode>) {

    match c {
        Some(VirtualKeyCode::W) => camera.move_forward(),
        Some(VirtualKeyCode::S) => camera.move_backward(),
        Some(VirtualKeyCode::A) => camera.move_left(),
        Some(VirtualKeyCode::D) => camera.move_right(),
        Some(VirtualKeyCode::Space) => camera.move_up(),
        Some(VirtualKeyCode::LShift) => camera.move_down(),
        _ => (),
    }
}

pub fn update_shape(shape: &dyn Shape, key: String, value: String) -> Option<Box<dyn Sync + Shape>> {    
    if shape.as_sphere().is_some() {
        return update_sphere(shape, key, value);
    } else if shape.as_plane().is_some() {
        return update_plane();
    } else if shape.as_cylinder().is_some() {
        return update_cylinder();
    } else if shape.as_cone().is_some() {
        return update_cone();
    } else {
        return None;
    }
}

fn update_sphere(shape: &dyn Shape, key: String, value: String) -> Option<Box<dyn Sync + Shape>> {
        let sphere = shape.as_sphere().unwrap();

        let mut pos = sphere.pos().clone();
        let mut radius = sphere.radius();
        let mut dir = sphere.dir().clone();
        
        match key.as_str() {
            "posx" => {
                pos = Vec3::new(value.parse::<f64>().unwrap(), *pos.y(), *pos.z());
            }
            "posy" => {
                pos = Vec3::new(*pos.x(), value.parse::<f64>().unwrap(), *pos.z());
            }
            "posz" => {
                pos = Vec3::new(*pos.x(), *pos.y(), value.parse::<f64>().unwrap());
            }
            "dirx" => {
                dir = Vec3::new(value.parse::<f64>().unwrap(), *dir.y(), *dir.z());
            }
            "diry" => {
                dir = Vec3::new(*dir.x(), value.parse::<f64>().unwrap(), *dir.z());
            }
            "dirz" => {
                dir = Vec3::new(*dir.x(), *dir.y(), value.parse::<f64>().unwrap());
            }
            "radius" => {
                radius = value.parse::<f64>().unwrap();
            }
            _ => (),
        }
        let sphere = sphere::Sphere::new(pos, dir, radius);
        Some(Box::new(sphere))
}

fn update_plane() -> Option<Box<dyn Sync + Shape>> {
    None
}

fn update_cylinder() -> Option<Box<dyn Sync + Shape>> {
    None
}

fn update_cone() -> Option<Box<dyn Sync + Shape>> {
    None
}