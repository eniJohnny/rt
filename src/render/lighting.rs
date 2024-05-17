use core::str;

use crate::{model::{
    materials::Color, maths::{hit::{self, Hit}, quaternion, ray::Ray, vec3::Vec3}, objects::light, scene::Scene
}, MAX_DEPTH};
use rand::Rng;

use super::{raycasting::{cast_ray, get_closest_hit}, restir::{PathBucket}};

pub fn reflect_dir(dir: &Vec3, normal: &Vec3) -> Vec3 {
	(dir - 2. * dir.dot(normal) * normal)
	    .normalize()
}

pub fn random_bounce_dir(dir: &Vec3, normal: &Vec3, roughness: f64) -> Vec3 {
	let reflect: Vec3 = reflect_dir(dir, normal);
	if roughness == 0. {
		return reflect;
	}
	loop {
		let mut rng = rand::thread_rng();
		let roll: f64 = rng.gen_range((0.)..(2. * std::f64::consts::PI));
		let yaw: f64 = rng.gen_range((-std::f64::consts::PI * roughness)..(std::f64::consts::PI * roughness));

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

pub fn diffuse_lighting(hit: &Hit, scene: &Scene, ray: &Ray) -> Color {
	let material = hit.element().material();
    let color = match material.needs_projection() {
        false => material.color(0, 0),
        true => {
            let point = hit.element().shape().projection(&hit);
            material.color(point.0, point.1)
        }
    };

    let mut light_color: Color = Color::new(0., 0., 0.);
	// Basic Diffuse
	for light in scene.lights() {
	    if !light.is_shadowed(scene, &hit) {
			light_color = light_color + light.as_ref().get_diffuse(&hit) * &color;
	    }
	}
	(light_color).clamp(0., 1.)
}

pub fn get_bounce_color<'a>(scene: &Scene, ray: &Ray, attenuation: bool, path: Vec<Hit<'a>>) -> PathBucket<'a> {
    match get_closest_hit(scene, ray) {
        Some(hit) => {
            let mut path_bucket = sampling_lighting(&hit, scene, ray, path);
			if attenuation {
				path_bucket.weight /= hit.dist() * hit.dist();
				path_bucket.pathWeight /= hit.dist() * hit.dist();
			}
			path_bucket
        }
		//TODO: Background color
        None => PathBucket {
			path: path,
			weight: 0.,
			pathWeight: 0.,
			nbElements: 0
		},
    }
}

pub fn random_bounce(hit: &Hit, ray: &Ray, normal: &Vec3, roughness: f64) -> Ray {
	let random_dir = random_bounce_dir(ray.get_dir(), normal, roughness);
	let random_bounce = Ray::new(hit.pos() + normal * 0.001, random_dir, ray.get_depth() + 1);
	random_bounce
}


pub fn get_reflected_light<'a>(hit: &Hit, scene: &Scene, ray: &Ray, path: Vec<Hit<'a>>) -> PathBucket<'a> {
	let mut bucket: PathBucket = PathBucket {
		path,
		weight: 0.,
		pathWeight: 0.,
		nbElements: 0
	};
	let sample_nb = 10;
	for i in 0..sample_nb {
		let random_bounce = random_bounce(hit, &ray, hit.norm(), hit.element().material().roughness());
		bucket.combine(get_bounce_color(scene, &random_bounce, false, path));
	}
	bucket
}

pub fn get_indirect_light<'a>(hit: &Hit, scene: &Scene, ray: &Ray, path: Vec<Hit<'a>>) -> PathBucket<'a> {
	let mut bucket: PathBucket = PathBucket {
		path,
		weight: 0.,
		pathWeight: 0.,
		nbElements: 0
	};
	let sample_nb = 100;
	for i in 0..sample_nb {
		let random_bounce = random_bounce(hit, &ray, hit.norm(), 1.);
		bucket.combine(get_bounce_color(scene, &random_bounce, true, path));
	}
	bucket
}

pub fn sampling_lighting<'a>(hit: &Hit, scene: &Scene, ray: &Ray, path: Vec<Hit<'a>>) -> PathBucket<'a> {
	let mut bucket: PathBucket = PathBucket {
		path,
		weight: 0.,
		pathWeight: 0.,
		nbElements: 0
	};

    let material = hit.element().material();
	let mut light_color: Color = Color::new(0., 0., 0.);
    let color = match material.needs_projection() {
        false => material.color(0, 0),
        true => {
            let point = hit.element().shape().projection(&hit);
            material.color(point.0, point.1)
        }
    };

	if ray.get_depth() > 0 {
		for light in scene.lights() {
			light_color = light_color + light.as_ref().get_diffuse(&hit) * &color;
		}
		let weight: f64 = light_color.r() + light_color.g() + light_color.b();
		bucket.add(bucket.path, weight);
	}

	// Indirect light
	if scene.indirect_lightning() && ray.get_depth() < MAX_DEPTH {
		bucket.combine(get_indirect_light(&hit, scene, ray, path));
	}
	
	(light_color).clamp(0., 1.);

	// Reflection
	let absorbed = 1.0 - material.reflection_coef() - material.refraction_coef();
	if ray.get_depth() < MAX_DEPTH {
		if scene.imperfect_reflections() || hit.element().material().roughness() >= f64::EPSILON {
			let mut reflect_bucket = get_reflected_light(&hit, scene, ray, path);
			let color_weight = color.r() + color.g() + color.b();
			bucket.weight *= absorbed;
			bucket.pathWeight *= absorbed;
			reflect_bucket.weight *= material.reflection_coef() * color_weight + absorbed;
			reflect_bucket.weight *= material.reflection_coef() * color_weight + absorbed;
			bucket.combine(reflect_bucket);
		}
	} else {
		bucket.weight *= absorbed;
		bucket.pathWeight *= absorbed;
	}
	bucket
}