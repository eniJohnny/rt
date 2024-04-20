use crate::{
    model::{
        materials::Color,
        maths::{hit::Hit, quaternion::Quaternion, ray::Ray},
        scene::Scene,
        Element,
    },
    SCREEN_HEIGHT, SCREEN_WIDTH,
};

use super::lighting::apply_lighting;

fn get_angle_to(fov: f64, pos: f64, length: f64) -> f64 {
    (pos / length - 0.5) * fov
}

pub fn get_ray(scene: &Scene, x: usize, y: usize) -> Ray {
    let roll = get_angle_to(scene.camera().fov(), x as f64, SCREEN_WIDTH as f64);
    let pitch = get_angle_to(scene.camera().vfov(), y as f64, SCREEN_HEIGHT as f64);
    let quat = Quaternion::from_euler_angles(pitch, roll, 0.);
    Ray::new(
        scene.camera().pos().clone(),
        scene.camera().dir().clone().rotate(&quat).normalize(),
        0,
    )
}

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
