use std::cmp::min;

use winit::event::VirtualKeyCode;
use crate::model::{materials::{Color, Material, metal::Metal}, maths::vec3::Vec3, objects::camera::Camera,  shapes::{plane, sphere, cylinder, cone, Shape}};

pub fn move_camera(camera: &mut Camera, c: Option<VirtualKeyCode>) {

    match c {
        Some(VirtualKeyCode::W) => camera.move_forward(),
        Some(VirtualKeyCode::S) => camera.move_backward(),
        Some(VirtualKeyCode::A) => camera.move_left(),
        Some(VirtualKeyCode::D) => camera.move_right(),
        Some(VirtualKeyCode::Up) => camera.look_up(),
        Some(VirtualKeyCode::Down) => camera.look_down(),
        Some(VirtualKeyCode::Left) => camera.look_left(),
        Some(VirtualKeyCode::Right) => camera.look_right(),
        Some(VirtualKeyCode::LShift) => camera.move_up(),
        Some(VirtualKeyCode::Space) => camera.move_down(),
        _ => (),
    }
    // camera.debug_print();
}

pub fn update_color(key: String, value: String, color: Color, metalness: f64, roughness: f64) -> Option<Box<dyn Sync + Material>> {
    let mut new_color: (u8, u8, u8) = ((color.r() * 255.) as u8, (color.g() * 255.) as u8, (color.b() * 255.) as u8);
    let new_value = min(value.parse::<u32>().unwrap(), 255) as u8;
    match key.as_str() {
        "colr" => {
            new_color.0 = new_value;
        }
        "colg" => {
            new_color.1 = new_value;
        }
        "colb" => {
            new_color.2 = new_value;
        }
        _ => (),
    }
    let new_color = Color::new(new_color.0 as f64 / 255., new_color.1 as f64 / 255., new_color.2 as f64 / 255.);
    let new_material = Metal::new(new_color, metalness, roughness);
    Some(Box::new(new_material))
}

pub fn update_metalness(value: String, color: Color, roughness: f64) -> Option<Box<dyn Sync + Material>> {
    let metalness = value.parse::<f64>().unwrap();
    let new_material = Metal::new(color, metalness, roughness);
    Some(Box::new(new_material))
}

pub fn update_roughness(value: String, color: Color, metalness: f64) -> Option<Box<dyn Sync + Material>> {
    let roughness = value.parse::<f64>().unwrap();
    let new_material = Metal::new(color, metalness, roughness);
    Some(Box::new(new_material))
}

pub fn update_shape(shape: &dyn Shape, key: String, value: String) -> Option<Box<dyn Sync + Shape>> {    
    if shape.as_sphere().is_some() {
        return update_sphere(shape, key, value);
    } else if shape.as_plane().is_some() {
        return update_plane(shape, key, value);
    } else if shape.as_cylinder().is_some() {
        return update_cylinder(shape, key, value);
    } else if shape.as_cone().is_some() {
        return update_cone(shape, key, value);
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

fn update_plane(shape: &dyn Shape, key: String, value: String) -> Option<Box<dyn Sync + Shape>> {
    let plane = shape.as_plane().unwrap();

    let mut pos = plane.pos().clone();
    let mut dir = plane.dir().clone();

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
        _ => (),
    }
    let plane = plane::Plane::new(pos, dir);
    Some(Box::new(plane))
}

fn update_cylinder(shape: &dyn Shape, key: String, value: String) -> Option<Box<dyn Sync + Shape>> {
    let cylinder = shape.as_cylinder().unwrap();

    let mut pos = cylinder.pos().clone();
    let mut radius = cylinder.radius();
    let mut dir = cylinder.dir().clone();
    let mut height = cylinder.height();

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
        "height" => {
            height = value.parse::<f64>().unwrap();
        }
        _ => (),
    }
    let cylinder = cylinder::Cylinder::new(pos, dir, radius, height);
    Some(Box::new(cylinder))
}

fn update_cone(shape: &dyn Shape, key: String, value: String) -> Option<Box<dyn Sync + Shape>> {
    let cone = shape.as_cone().unwrap();

    let mut pos = cone.pos().clone();
    let mut radius = cone.radius();
    let mut dir = cone.dir().clone();
    let mut height = cone.height();

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
        "height" => {
            height = value.parse::<f64>().unwrap();
        }
        _ => (),
    }
    let cone = cone::Cone::new(pos, dir, radius, height);
    Some(Box::new(cone))
}