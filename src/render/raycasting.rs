use std::{
    borrow::Borrow,
    cmp::min,
    collections::VecDeque,
    sync::{
        mpsc::{self, Receiver},
        Arc, Mutex, RwLock,
    },
    thread,
    time::{Duration, Instant},
};

use image::RgbaImage;

use crate::{
    model::{
        materials::Color,
        maths::{hit::Hit, quaternion::Quaternion, ray::Ray, vec3::Vec3},
        objects::{camera::Camera, light::AmbientLight},
        scene::Scene,
        Element,
    },
    BASE_SIMPLIFICATION, MAX_THREADS, SCREEN_HEIGHT, SCREEN_WIDTH,
}

use super::lighting::apply_lighting;

pub fn cast_ray(scene: &Scene, ray: &Ray) -> Color {
    match get_closest_hit(scene, ray) {
        Some(hit) => apply_lighting(hit, scene, ray),
        None => Color::new(0., 0., 0.),
    }
}

pub fn get_closest_hit<'a>(scene: &'a Scene, ray: &Ray) -> Option<Hit<'a>> {
    let mut closest: Option<(f64, &Element)> = None;
    for element in scene.elements().iter() {
        if let Some(t) = element.shape().intersect(ray) {
            if let Some((tmin, _)) = &closest {
                if &t[0] < tmin {
                    closest = Some((t[0], element));
                }
            } else {
                closest = Some((t[0], element))
            }
        }
    }
    match closest {
        None => None,
        Some((t, elem)) => Some(Hit::new(elem, t, ray.get_pos() + ray.get_dir() * t)),
    }
}
