use crate::model::{
    materials::{
        color::Color,
        diffuse::Diffuse,
        material::Material,
        texture::{Texture, TextureType},
    },
    maths::vec3::Vec3,
    shapes::{cone, cylinder, plane, sphere, Shape},
};
use std::cmp::min;
pub fn update_color(key: String, value: String, material: &mut dyn Material) {
    if let Texture::Value(color, _) = material.color() {
        let mut new_color: (u8, u8, u8) = (
            (color.x() * 255.) as u8,
            (color.y() * 255.) as u8,
            (color.z() * 255.) as u8,
        );
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
        let new_color = Vec3::new(
            new_color.0 as f64 / 255.,
            new_color.1 as f64 / 255.,
            new_color.2 as f64 / 255.,
        );
        material.set_color(Texture::Value(new_color, TextureType::Color));
    }
}

pub fn update_shape(
    shape: &dyn Shape,
    key: String,
    value: String,
) -> Option<Box<dyn Sync + Shape>> {
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
