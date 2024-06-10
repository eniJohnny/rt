use core::str;

use crate::{
    model::{
        materials::Color,
        maths::{
            hit::{self, Hit, HitType},
            quaternion,
            ray::Ray,
            vec3::Vec3,
        },
        objects::light,
        scene::Scene,
    },
    MAX_DEPTH,
};
use rand::Rng;

use super::{
    raycasting::{get_closest_hit, sampling_ray},
    restir::{Path, PathBucket, Sample},
};

pub fn reflect_dir(dir: &Vec3, normal: &Vec3) -> Vec3 {
    (dir - 2. * dir.dot(normal) * normal).normalize()
}

pub fn random_bounce_dir(dir: &Vec3, normal: &Vec3, roughness: f64) -> Vec3 {
    let reflect: Vec3 = reflect_dir(dir, normal);
    if roughness == 0. {
        return reflect;
    }
    loop {
        let mut rng = rand::thread_rng();
        let roll: f64 = rng.gen_range((0.)..(2. * std::f64::consts::PI));
        let yaw: f64 =
            rng.gen_range((-std::f64::consts::PI * roughness)..(std::f64::consts::PI * roughness));

        let mut axis = normal.cross(&Vec3::new(1., 0., 0.));
        if axis == Vec3::new(0., 0., 0.) {
            axis = normal.cross(&Vec3::new(0., 1., 0.));
        }
        let mut random = reflect.rotate(&quaternion::Quaternion::new_from_axis_angle(&axis, yaw));
        random = random.rotate(&quaternion::Quaternion::new_from_axis_angle(&reflect, roll));
        if random.dot(&normal) >= 0. {
            return random.normalize();
        }
    }
}

pub fn get_bounce_color<'a>(
    scene: &'a Scene,
    ray: &Ray,
    hit: Hit<'a>,
    attenuation: bool,
) -> Sample<'a> {
    match get_closest_hit(scene, ray) {
        Some(hit) => {
            let mut sample = sampling_lighting(hit.clone(), scene, ray);
            if attenuation {
                sample.weight /= hit.dist() * hit.dist();
            }
            sample
        }
        //TODO: Background color
        None => Sample {
            path: Path {
                hit: hit.clone(),
                indirect: None,
                reflect: None,
            },
            weight: 0.,
        },
    }
}

pub fn random_bounce(hit: &Hit, ray: &Ray, normal: &Vec3, roughness: f64) -> Ray {
    let random_dir = random_bounce_dir(ray.get_dir(), normal, roughness);
    let random_bounce = Ray::new(hit.pos() + normal * 0.001, random_dir, ray.get_depth() + 1);
    random_bounce
}

pub fn get_reflected_light_bucket<'a>(hit: Hit<'a>, scene: &'a Scene, ray: &Ray) -> PathBucket<'a> {
    let mut bucket: PathBucket = PathBucket {
        sample: None,
        weight: 0.,
        nbSamples: 0,
    };
    if hit.element().material().roughness() < f64::EPSILON {
        if hit.dist() < &100. {
            let dir = reflect_dir(ray.get_dir(), hit.norm());
            let mut ray = Ray::new(hit.pos().clone(), dir, ray.get_depth() + 1);
            // dbg!(ray.clone());
            bucket.add(get_bounce_color(scene, &ray, hit.clone(), false))
        }
    } else {
        let sample_nb = 10;
        for _ in 0..sample_nb {
            let random_bounce =
                random_bounce(&hit, &ray, hit.norm(), hit.element().material().roughness());
            bucket.add(get_bounce_color(scene, &random_bounce, hit.clone(), false));
        }
    }
    bucket
}

pub fn get_reflected_light_sample<'a>(
    hit: Hit<'a>,
    scene: &'a Scene,
    ray: &Ray,
) -> Option<Sample<'a>> {
    let bucket = get_reflected_light_bucket(hit.clone(), scene, ray);
    if let Some(mut sample) = bucket.sample {
        sample.weight = bucket.weight / (sample.weight * bucket.nbSamples as f64);
        return Some(sample);
    }
    None
}

pub fn get_indirect_light_sample<'a>(hit: Hit<'a>, scene: &'a Scene, ray: &Ray) -> Sample<'a> {
    let bucket = get_indirect_light_bucket(hit.clone(), scene, ray);
    if let Some(mut sample) = bucket.sample {
        sample.weight = bucket.weight / (sample.weight * bucket.nbSamples as f64);
        return sample;
    }
    Sample {
        path: Path {
            hit,
            reflect: None,
            indirect: None,
        },
        weight: 0.,
    }
}

pub fn get_indirect_light_bucket<'a>(hit: Hit<'a>, scene: &'a Scene, ray: &Ray) -> PathBucket<'a> {
    let mut bucket: PathBucket = PathBucket {
        sample: None,
        weight: 0.,
        nbSamples: 0,
    };
    let sample_nb = 20;
    for _ in 0..sample_nb {
        let mut random_bounce = random_bounce(&hit, &ray, hit.norm(), 1.);
        random_bounce.set_sampling(true);
        bucket.add(get_bounce_color(scene, &random_bounce, hit.clone(), true));
    }
    bucket
}

pub fn sampling_lighting<'a>(hit: Hit<'a>, scene: &'a Scene, ray: &Ray) -> Sample<'a> {
    let mut sample = Sample {
        path: Path {
            hit,
            indirect: None,
            reflect: None,
        },
        weight: 0.,
    };

    let material = sample.path.hit.element().material();
    let mut light_color: Color = Color::new(0., 0., 0.);
    let color = match material.needs_projection() {
        false => material.color(0, 0),
        true => {
            let point = sample
                .path
                .hit
                .element()
                .shape()
                .projection(&sample.path.hit);
            material.color(point.0, point.1)
        }
    };

    let mut indirect_light_weight = 0.;

    if ray.get_depth() > 0 {
        for light in scene.lights() {
            light_color = light_color + light.as_ref().get_diffuse(&sample.path.hit) * &color;
        }
        indirect_light_weight += light_color.r() + light_color.g() + light_color.b();
    }

    // Indirect light
    if scene.indirect_lightning() && ray.get_depth() < MAX_DEPTH {
        let indirect_sample = get_indirect_light_sample(sample.path.hit.clone(), scene, ray);
        indirect_light_weight = indirect_sample.weight;
        sample.path.indirect = Some(Box::new(indirect_sample));
    }

    (light_color).clamp(0., 1.);

    // Reflection
    let absorbed = 1.0 - material.reflection_coef() - material.refraction_coef();
    if ray.get_depth() < MAX_DEPTH {
        if scene.imperfect_reflections() {
            let reflect_sample = get_reflected_light_sample(sample.path.hit.clone(), scene, ray);
            let color_weight = color.r() + color.g() + color.b();

            sample.weight = indirect_light_weight * absorbed;
            if let Some(reflect_sample) = reflect_sample {
                sample.weight +=
                    reflect_sample.weight * (material.reflection_coef() * color_weight + absorbed);
                sample.path.reflect = Some(Box::new(reflect_sample));
            }
        }
    } else {
        sample.weight = indirect_light_weight * absorbed;
    }
    sample
}
