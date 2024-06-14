use rand::Rng;

use crate::{
    model::{
        materials::color::Color, maths::{hit::Hit, quaternion::Quaternion, ray::Ray, vec3::Vec3}, scene::Scene, Element
    }, ANTIALIASING, MAX_DEPTH, SCREEN_HEIGHT, SCREEN_WIDTH
};

use super::{
    lighting_sampling::{get_indirect_light_bucket, random_bounce, sampling_lighting},
    restir::{Path, PathBucket, Sample},
};

pub fn get_ray_debug(scene: &Scene, x: usize, y: usize, debug: bool) -> Ray {
    let width = (scene.camera().fov() / 2.).tan() * 2.;
    let height = width * SCREEN_HEIGHT as f64 / SCREEN_WIDTH as f64;
    // Centre de l'ecran
    let center: Vec3 = scene.camera().pos() + scene.camera().dir();

    // Coin superieur gauche, et les distances pour atteindre a partir de lui les coin superieur droit et inferieur gauche
    let top_left = center +  scene.camera().u() * - width / 2. + scene.camera().v() * height / 2.;
    let left_to_right = scene.camera().u() * width;
    let top_to_bot = scene.camera().v() * height;

	let dir = &top_left - scene.camera().pos()
		- &top_to_bot * ((y as f64 / SCREEN_HEIGHT as f64) + rand::thread_rng().gen_range((0.)..ANTIALIASING))
		+ &left_to_right * ((x as f64 / SCREEN_WIDTH as f64) + rand::thread_rng().gen_range((0.)..ANTIALIASING));
	let mut ray = Ray::new(scene.camera().pos().clone(), dir.normalize(), 0);
    ray.debug = debug;
    ray
}

pub fn get_ray(scene: &Scene, x: usize, y: usize) -> Ray {
    get_ray_debug(scene, x, y, false)
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
