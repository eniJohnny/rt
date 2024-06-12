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
    let mut rng = rand::thread_rng();
    loop {
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
) -> Sample<'a> {
    match get_closest_hit(scene, ray) {
        Some(hit) => {
            sampling_lighting(hit.clone(), scene, ray)
        }
        //TODO: Background color
        None => Sample {
            path: Path {
                hit: hit.clone(),
                indirect: None,
                reflect: None,
            },
            color: Color::new(0., 0., 0.),
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
            let mut reflect_ray = Ray::new(hit.pos().clone(), dir, ray.get_depth() + 1);
            reflect_ray.set_sampling(ray.is_sampling());
            bucket.add(get_bounce_color(scene, &reflect_ray, hit.clone()));
        }
    } else {
        let sample_nb = 5;
        let material = hit.element().material();
        let mat_color = material.color(&hit);
        let mat_absorbed = 1.0 - material.reflection_coef() - material.refraction_coef();

        for _ in 0..sample_nb {
            let random_bounce =
                random_bounce(&hit, &ray, hit.norm(), hit.element().material().roughness());
            let mut sample = get_bounce_color(scene, &random_bounce, hit.clone());
            sample.color = &sample.color * material.reflection_coef() * &mat_color
            + sample.color * mat_absorbed;
            sample.weight = sample.color.as_weight();
            bucket.add(sample);
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
        color: Color::new(0., 0., 0.),
        weight: 0.,
    }
}

pub fn get_indirect_light_bucket<'a>(hit: Hit<'a>, scene: &'a Scene, ray: &Ray) -> PathBucket<'a> {
    let mut bucket: PathBucket = PathBucket {
        sample: None,
        weight: 0.,
        nbSamples: 0,
    };
    let sample_nb = 5;
    for _ in 0..sample_nb {
        let mut random_bounce = random_bounce(&hit, &ray, hit.norm(), 1.);
        random_bounce.set_sampling(true);
        let mut sample = get_bounce_color(scene, &random_bounce, hit.clone());
        sample.color = &sample.color * hit.element().material().color(&hit);
        sample.weight = sample.color.as_weight();
        bucket.add(sample);
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
        color: Color::new(0., 0., 0.),
        weight: 0.,
    };

    let material = sample.path.hit.element().material();
    let absorbed = 1.0 - material.reflection_coef() - material.refraction_coef();
    let mut light_color: Color = Color::new(0., 0., 0.);
    let color = material.color(&sample.path.hit);

    if ray.get_depth() > 0 {
        for light in scene.lights() {
            light_color += light.as_ref().get_diffuse(&sample.path.hit) * &color;
        }
    }
    // Indirect light
    if scene.indirect_lightning() && ray.get_depth() < MAX_DEPTH {
        let indirect_sample = get_indirect_light_sample(sample.path.hit.clone(), scene, ray);
        light_color += &indirect_sample.color * &color * indirect_sample.weight;
        sample.path.indirect = Some(Box::new(indirect_sample));
    }

    light_color = light_color * absorbed;
    // Reflection
    if ray.get_depth() < MAX_DEPTH {
        if scene.imperfect_reflections() {
            let reflect_sample = get_reflected_light_sample(sample.path.hit.clone(), scene, ray);
            
            if let Some(reflect_sample) = reflect_sample {
                light_color += &reflect_sample.color * reflect_sample.weight;
                sample.path.reflect = Some(Box::new(reflect_sample));
            }
        }
    }
    (light_color).clamp(0., 1.);
    sample.color = light_color;
    sample.weight = sample.color.as_weight();
    sample
}
