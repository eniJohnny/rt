use rand::Rng;

use crate::{
    model::{
        materials::Color,
        maths::{hit::Hit, quaternion::Quaternion, ray::Ray, vec3::Vec3},
        scene::Scene,
        Element,
    }, ANTIALIASING, MAX_DEPTH, SCREEN_HEIGHT, SCREEN_WIDTH
};

use super::{
    lighting_sampling::{get_indirect_light_bucket, random_bounce, sampling_lighting},
    restir::{Path, PathBucket, Sample},
};

fn get_angle_to(fov: f64, pos: f64, length: f64) -> f64 {
    (pos / length - 0.5) * fov
}

pub fn get_ray(scene: &Scene, x: usize, y: usize) -> Ray {
    let mut rng = rand::thread_rng();
    let roll = -f64::atan2(-scene.camera().dir().x(), *scene.camera().dir().z());
    let pitch = scene.camera().dir().y().asin();
    let roll = roll + get_angle_to(scene.camera().fov(), x as f64, SCREEN_WIDTH as f64) + rng.gen_range((0.)..(ANTIALIASING));
    let pitch = pitch + get_angle_to(scene.camera().vfov(), y as f64, SCREEN_HEIGHT as f64) + rng.gen_range((0.)..(ANTIALIASING));
    let yaw = 0.;
    let quat = Quaternion::from_euler_angles(pitch, roll, yaw);
    Ray::new(
        scene.camera().pos().clone(),
        Vec3::new(0., 0., 1.).rotate(&quat).normalize(),
        0,
    )
}

pub fn sampling_ray<'a>(scene: &'a Scene, ray: &Ray) -> PathBucket<'a> {
    match get_closest_hit(scene, ray) {
        Some(hit) => {
            let mut bucket = get_indirect_light_bucket(hit.clone(), scene, ray);
            let mut path = Path {
                hit,
                reflect: None,
                indirect: None,
            };
            let mut weight = 0.;
            if let Some(sample) = bucket.sample.clone() {
                weight = sample.weight;
                path.indirect = Some(Box::new(sample));
            }
            bucket.sample = Some(Sample { path, color: Color::new(0., 0., 0.),weight });
            bucket
        }
        None => PathBucket {
            sample: None,
            weight: 0.,
            nbSamples: 0,
        }, //TODO Handle background on None
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
        Some((t, elem)) => Some(Hit::new(
            elem,
            t,
            ray.get_pos() + ray.get_dir() * (t - f64::EPSILON),
            ray.get_dir(),
        )),
    }
}
