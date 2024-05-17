use core::str;

use crate::{model::{
    materials::Color, maths::{hit::{self, Hit}, quaternion, ray::Ray, vec3::Vec3}, objects::light, scene::Scene
}, MAX_DEPTH};
use rand::Rng;

use super::raycasting::{cast_ray, get_closest_hit};

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
struct Bucket {
	ray: Ray,
	weight: f64,
}


pub fn apply_lighting_bounce(hit: &Hit, scene: &Scene, ray: &Ray) -> Color {
    let material = hit.element().material();
    let color = match material.needs_projection() {
        false => material.color(0, 0),
        true => {
            let point = hit.element().shape().projection(&hit);
            material.color(point.0, point.1)
        }
    };
    let mut light_color: Color = Color::new(0., 0., 0.);
    light_color =
		light_color + scene.ambient_light().intensity() * scene.ambient_light().color() * &color;
	for light in scene.lights() {
		light_color = light_color + light.as_ref().get_diffuse(&hit) * &color;
	}
	(light_color).clamp(0., 1.)
}

pub fn get_bounce_color(scene: &Scene, ray: &Ray) -> Color {
    match get_closest_hit(scene, ray) {
        Some(hit) => {
            apply_lighting_bounce(&hit, scene, ray)
        }
        None => Color::new(0., 0., 0.),
    }
}

pub fn random_bounce(ray: &Ray, normal: &Vec3, roughness: f64) -> Ray {
	let random_dir = random_bounce_dir(ray.get_dir(), normal, roughness);
	let random_bounce = Ray::new(ray.get_pos() + normal * 0.001, random_dir, ray.get_depth() + 1);
	random_bounce
}


pub fn get_reflected_light(hit: &Hit, scene: &Scene, ray: &Ray) -> Color {
	let mut bucket: Bucket = Bucket {
		ray: Ray::new(hit.pos() + hit.norm() * 0.001,
					reflect_dir(ray.get_dir(), hit.norm()),
					ray.get_depth() + 1),
		weight: 0.,
	};
	if scene.imperfect_reflections() || hit.element().material().roughness() <= f64::EPSILON {
		let sample_nb = 1;
		for i in 0..sample_nb {
			let random_bounce = random_bounce(&ray, hit.norm(), hit.element().material().roughness());
			let bounce_color = get_bounce_color(scene, &random_bounce);
			let weight = bounce_color.r() + bounce_color.g() + bounce_color.b();
			bucket.weight += weight;
			if i == 0 {
				bucket.ray = random_bounce;
			} else {
				let rand: f64 = rand::thread_rng().gen_range((0.)..(bucket.weight + weight));
				if rand > bucket.weight {
					bucket.ray = random_bounce;
				}	
			}
		}
		return cast_ray(scene, &bucket.ray);
	} 
	Color::new(0., 0., 0.)
}




pub fn get_indirect_light(hit: &Hit, scene: &Scene, ray: &Ray) -> Color {
	let mut bucket: Bucket = Bucket {
		ray: Ray::new(ray.get_pos() + hit.norm() * 0.001,
					ray.get_dir().clone(),
					ray.get_depth() + 1),
		weight: 0.,
	};
	let sample_nb = 100;
	for i in 0..sample_nb {
		let random_bounce = random_bounce(&ray, hit.norm(), 1.);
		let bounce_color = get_bounce_color(scene, &random_bounce);
		let weight = bounce_color.r() + bounce_color.g() + bounce_color.b();
		bucket.weight += weight;
		if i == 0 {
			bucket.ray = random_bounce;
		} else {
			let rand: f64 = rand::thread_rng().gen_range((0.)..(bucket.weight + weight));
			if rand > bucket.weight {
				bucket.ray = random_bounce;
			}	
		}
	}

	cast_ray(scene, &bucket.ray)
}

pub fn apply_lighting(hit: &Hit, scene: &Scene, ray: &Ray) -> Color {

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
			// light_color = light_color + light.as_ref().get_specular(&hit, ray);
	    }
	}

	// Indirect light
	if scene.indirect_lightning() && ray.get_depth() < MAX_DEPTH {
		light_color = light_color + get_indirect_light(&hit, scene, ray);
	}
	
	(light_color).clamp(0., 1.);

	// Reflection
	let absorbed = 1.0 - material.reflection_coef() - material.refraction_coef();
	if ray.get_depth() < MAX_DEPTH {
		let reflected_light = get_reflected_light(&hit, scene, ray);
		light_color = light_color + &reflected_light * 0.2;
		if material.reflection_coef() > 0. {
			light_color = light_color * absorbed + reflected_light * material.reflection_coef();
		}
	} else {
		light_color = light_color * absorbed;
	}
	light_color
}