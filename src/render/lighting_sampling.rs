use core::str;

use crate::{
    model::{
        materials::color::Color,
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
    lighting_real::get_lighting_from_ray,
    raycasting::{get_closest_hit, sampling_ray},
    restir::{Path, PathBucket, Sample},
};

pub fn reflect_dir(dir: &Vec3, normal: &Vec3) -> Vec3 {
    (dir - 2. * dir.dot(normal) * normal).normalize()
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let mut rng = rand::thread_rng();
        let vec = Vec3::new(
            rng.gen_range((-1.)..(1.)),
            rng.gen_range((-1.)..(1.)),
            rng.gen_range((-1.)..(1.)),
        );
        if vec.length() <= 1. {
            return vec.normalize();
        }
    }
}

// pub fn get_bounce_color<'a>(scene: &'a Scene, ray: &Ray, hit: Hit<'a>) -> Sample<'a> {
//     match get_closest_hit(scene, ray) {
//         Some(hit) => sampling_lighting(hit, scene, ray),
//         //TODO: Background color
//         None => Sample {
//             color: Color::new(0., 0., 0.),
//             weight: 0.,
//         },
//     }
// }

pub fn get_indirect_light_sample<'a>(hit: Hit<'a>, scene: &'a Scene, ray: &Ray) -> Sample {
    let bucket = get_indirect_light_bucket(hit.clone(), scene, ray);
    if let Some(mut sample) = bucket.sample {
        if bucket.weight < 0.01 {
            sample.weight = 0.0;
            return sample;
        }
        sample.weight = bucket.weight / (sample.weight * bucket.nbSamples as f64);
        return sample;
    }
    Sample {
        color: Color::new(0., 0., 0.),
        weight: 0.,
    }
}

pub fn get_reflected_light_sample<'a>(
    hit: Hit<'a>,
    scene: &'a Scene,
    ray: &Ray,
    roughness: f64,
) -> Sample {
    let bucket = get_reflected_light_bucket(hit.clone(), scene, ray);
    if let Some(mut sample) = bucket.sample {
        if bucket.weight < 0.01 {
            sample.weight = 0.0;
            return sample;
        }
        sample.weight = bucket.weight / (sample.weight * bucket.nbSamples as f64);
        return sample;
    }
    Sample {
        color: Color::new(0., 0., 0.),
        weight: 0.,
    }
}

pub fn get_reflected_light_bucket<'a>(hit: Hit<'a>, scene: &'a Scene, ray: &Ray) -> PathBucket {
    let mut bucket: PathBucket = PathBucket {
        sample: None,
        weight: 0.,
        nbSamples: 0,
    };
    let sample_nb = 5;
    for _ in 0..sample_nb {
        let dir = (reflect_dir(ray.get_dir(), hit.norm()) + random_unit_vector() * hit.roughness())
            .normalize();
        if dir.dot(hit.norm()) < f64::EPSILON {
            bucket.add(Sample {
                weight: 0.,
                color: Color::new(0., 0., 0.),
            });
        } else {
            let mut reflect_ray = Ray::new(hit.pos().clone(), dir, ray.get_depth() + 1);
            reflect_ray.set_sampling(true);
            reflect_ray.debug = ray.debug;

            let color = get_lighting_from_ray(scene, &reflect_ray);
            let sample = Sample {
                weight: color.as_weight(),
                color,
            };
            bucket.add(sample);
        }
    }
    bucket
}

pub fn get_indirect_light_bucket<'a>(hit: Hit<'a>, scene: &'a Scene, ray: &Ray) -> PathBucket {
    let mut bucket: PathBucket = PathBucket {
        sample: None,
        weight: 0.,
        nbSamples: 0,
    };
    let sample_nb = 5;
    for _ in 0..sample_nb {
        let mut indirect_dir = hit.norm() + random_unit_vector();
        if indirect_dir.length() < 0.01 {
            indirect_dir = hit.norm().clone();
        }
        let mut indirect_ray = Ray::new(hit.pos().clone(), indirect_dir, ray.get_depth() + 1);
        indirect_ray.set_sampling(true);
        indirect_ray.debug = ray.debug;

        let color = get_lighting_from_ray(scene, &indirect_ray);
        let sample = Sample {
            weight: color.as_weight(),
            color,
        };
        bucket.add(sample);
    }
    bucket
}
