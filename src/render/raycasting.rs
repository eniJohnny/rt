use rand::Rng;

use crate::{
    model::{
        materials::color::Color,
        maths::{hit::Hit, quaternion::Quaternion, ray::Ray, vec3::Vec3},
        scene::Scene,
        shapes::plane::Plane,
        Element,
    },
    ANTIALIASING, MAX_DEPTH, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use super::{
    lighting::lighting_sampling::get_reflected_light_bucket,
    restir::{Path, PathBucket, Sample},
};

pub fn get_ray_debug(scene: &Scene, x: usize, y: usize, debug: bool) -> Ray {
    let width = (scene.camera().fov() / 2.).tan() * 2.;
    let height = width * SCREEN_HEIGHT as f64 / SCREEN_WIDTH as f64;
    // Centre de l'ecran
    let center: Vec3 = scene.camera().pos() + scene.camera().dir();

    // Coin superieur gauche, et les distances pour atteindre a partir de lui les coin superieur droit et inferieur gauche
    let top_left = center + scene.camera().u() * -width / 2. + scene.camera().v() * height / 2.;
    let left_to_right = scene.camera().u() * width;
    let top_to_bot = scene.camera().v() * height;

    let dir = &top_left
        - scene.camera().pos()
        - &top_to_bot
            * ((y as f64 / SCREEN_HEIGHT as f64)
                + rand::thread_rng().gen_range((0.)..ANTIALIASING))
        + &left_to_right
            * ((x as f64 / SCREEN_WIDTH as f64) + rand::thread_rng().gen_range((0.)..ANTIALIASING));
    let mut ray = Ray::new(scene.camera().pos().clone(), dir.normalize(), 0);
    ray.debug = debug;
    ray
}

pub fn get_ray(scene: &Scene, x: usize, y: usize) -> Ray {
    get_ray_debug(scene, x, y, false)
}

pub fn sampling_ray(scene: &Scene, ray: &Ray) -> PathBucket {
    match get_closest_hit(scene, ray) {
        Some(hit) => {
            let mut bucket = get_reflected_light_bucket(hit.clone(), scene, ray);
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
            bucket.sample = Some(Sample {
                color: Color::new(0., 0., 0.),
                weight,
            });
            bucket
        }
        None => PathBucket {
            sample: None,
            weight: 0.,
            nbSamples: 0,
        }, //TODO Handle background on None
    }
}
fn intersect2(plane: &Plane, r: Ray) -> Option<Vec<f64>> {
    let dist = plane.pos() - r.get_pos();
    let mut dir = plane.dir().clone();
    let mut dot_product = r.get_dir().dot(plane.dir());
    if dot_product > 0. {
        dir = -dir;
        dot_product = -dot_product;
    }
    let t = dist.dot(&dir) / dot_product;
    return Some(Vec::from([t]));
    None
}

pub fn get_closest_hit<'a>(scene: &'a Scene, ray: &Ray) -> Option<Hit<'a>> {
    let mut closest: Option<Hit> = None;
    for element in scene.elements().iter() {
        let mut t = None;
        t = element.shape().intersect(ray);
        if let Some(t) = t {
            for dist in t {
                if dist > 0.0 {
                    if let Some(hit) = &closest {
                        if &dist < hit.dist() {
                            let new_hit = Hit::new(
                                element,
                                dist,
                                ray.get_pos() + ray.get_dir() * (dist - f64::EPSILON),
                                ray.get_dir(),
                                scene.textures(),
                            );
                            if new_hit.opacity() > 0.5 {
                                closest = Some(new_hit);
                            }
                        }
                    } else {
                        let new_hit = Hit::new(
                            element,
                            dist,
                            ray.get_pos() + ray.get_dir() * (dist - f64::EPSILON),
                            ray.get_dir(),
                            scene.textures(),
                        );
                        if new_hit.opacity() > 0.5 {
                            closest = Some(new_hit);
                        }
                    }
                }
            }
        }
    }
    match closest {
        None => None,
        Some(mut hit) => {
            hit.map_textures(scene.textures());
            Some(hit)
        }
    }
}
