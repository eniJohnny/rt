use std::fmt::Debug;

use chrono::format::Numeric;
use image::error;
use rand::{distributions, Rng};
use rusttype::Vector;

use crate::{
    model::{
        materials::color::Color,
        maths::{hit::{self, Hit}, ray::Ray, vec3::Vec3},
        objects::light,
        scene::Scene, shapes::Shape, Element,
    },
    MAX_DEPTH,
};

use super::{
    lighting_sampling::{
        get_indirect_light_sample, get_reflected_light_bucket, get_reflected_light_sample,
        random_unit_vector, reflect_dir,
    },
    raycasting::{get_closest_hit, get_ray},
    restir::{PathBucket, Sample}, skysphere::get_skysphere_color,
};

pub fn get_lighting_from_ray(scene: &Scene, ray: &Ray) -> Color {
    match get_closest_hit(scene, ray) {
        Some(hit) => get_lighting_from_hit(scene, &hit.0, ray, &hit.1),
        //TODO : Handle BG on None
        // None => Color::new(0., 0., 0.),
        None => {
            get_skysphere_color(scene, ray)
        }
    }
}

pub fn fresnel_reflect_ratio(n1: f64, n2: f64, norm: &Vec3, ray: &Vec3, f0: f64, f90: f64) -> f64 {
    // Schlick aproximation
    let mut r0 = (n1 - n2) / (n1 + n2);
    r0 = r0 * r0;
    let mut cosX = -(norm.dot(&ray));
    if n1 > n2 {
        let n = n1 / n2;
        let sinT2 = n * n * (1.0 - cosX * cosX);
        // Total internal reflection
        if sinT2 > 1.0 {
            return f90;
        }
        cosX = (1.0 - sinT2).sqrt();
    }
    let x = 1.0 - cosX;
    let ret = r0 + (1.0 - r0) * x.powf(5.);

    // adjust reflect multiplier for object reflectivity
    f0 * (1.0 - ret) + f90 * ret
}

pub fn get_lighting_from_hit(scene: &Scene, hit: &Hit, ray: &Ray, t: &Vec<(Vec<f64>, &Element)>) -> Color {
	let is_inside;
	
	if hit.all_dist().len() % 2 == 0 {
		let mut nb_dist_positif = 0;
		for dist in hit.all_dist() {
			if dist > &0. {
				nb_dist_positif += 1;
			}
		}
		is_inside = nb_dist_positif % 2 != 0;
	} else {
		is_inside = true;
	} 
	
	let current_refraction_index;
	let next_refraction_index;
	let normal: Vec3;
	if is_inside {
		current_refraction_index = hit.refraction();
		next_refraction_index = 1.0;
		normal = -hit.norm().clone();
	} else {
		current_refraction_index = 1.0;
		next_refraction_index = hit.refraction();
		normal = hit.norm().clone();
	}


    let absorbed = 1.0 - hit.metalness();

    if ray.debug {
        println!(
            "Metal : {}, Roughness: {}, Color: {}, Norm: {}, Emissive: {}, Opacity: {}, Refraction: {}",
            hit.metalness(),
            hit.roughness(),
            hit.color(),
            hit.norm(),
            hit.emissive(),
            hit.opacity(),
            hit.transparency()
        );
    }

    if hit.emissive() > f64::EPSILON {
        return hit.emissive() * hit.color();
    }
    let mut light_color = Color::new(0., 0., 0.);
    let fresnel_factor = fresnel_reflect_ratio(current_refraction_index, next_refraction_index, &normal, ray.get_dir(), 0., 1.0 - hit.roughness());
    let reflected = fresnel_factor * absorbed;
    let rand = rand::thread_rng().gen_range(0.0..1.0);
    if rand < reflected && scene.imperfect_reflections() && ray.get_depth() < MAX_DEPTH {
		// Reflected Light
        if rand > hit.metalness() {
            light_color += get_reflected_light_color(scene, hit, ray);
        } else {
            light_color += get_reflected_light_color(scene, hit, ray) * hit.color();
        }
	} else {
		let refracted = hit.transparency();
		let rand = rand::thread_rng().gen_range(0.0..1.0);
		if rand < refracted && ray.get_depth() < MAX_DEPTH {
			// Refracted Light
			light_color += get_refracted_light_color(scene, hit, ray, current_refraction_index, next_refraction_index, &normal);
		} else if scene.indirect_lightning() && ray.get_depth() < MAX_DEPTH {
			// Indirect Light
			light_color += get_indirect_light_color(scene, hit, ray);
        }
    }
	light_color
}

fn refract_dir(incoming: &Vec3, normal: &Vec3, n1: f64, n2: f64) -> Option<Vec3>
{
    let n = n1 / n2;
    let cos_i = -incoming.dot(&normal);
    let sin_t2 = n * n * (1.0 - cos_i * cos_i);

    // Check for total internal reflection
    if sin_t2 > 1.0 {
        return None;
    }
    let cos_t = (1.0 - sin_t2).sqrt();
    let refracted = n * incoming + (n * cos_i - cos_t) * normal;
    Some(refracted)
}

fn get_reflected_light_color(scene: &Scene, hit: &Hit, ray: &Ray) -> Color
{
	let mut light_color = Color::new(0., 0., 0.);
	let reflect_color;
	// if ray.get_depth() == 0 {
	//     let sample = get_reflected_light_sample(hit.clone(), scene, &ray, hit.roughness());
	//     reflect_color = &sample.color * sample.weight;
	// } else {
	let dir = (reflect_dir(ray.get_dir(), hit.norm()) + random_unit_vector() * hit.roughness())
		.normalize();
	if dir.dot(hit.norm()) > f64::EPSILON {
		let reflect_ray = Ray::new(hit.pos().clone(), dir, ray.get_depth() + 1);
		reflect_color = get_lighting_from_ray(scene, &reflect_ray);
	} else {
		reflect_color = Color::new(0., 0., 0.);
	}
	// }
	reflect_color
}

fn get_refracted_light_color(scene: &Scene, hit: &Hit, ray: &Ray, n1: f64, n2: f64, normal: &Vec3) -> Color
{
	let mut refract_color = Color::new(0., 0., 0.);
	let refraction_result = refract_dir(&ray.get_dir(), normal, n1, n2);
	if let Some(refracted_ray) = refraction_result {
		let refract_ray = Ray::new(hit.pos().clone() - normal * 0.2, refracted_ray.clone(), ray.get_depth());
		refract_color = get_lighting_from_ray(scene, &refract_ray);
	}
	refract_color
}

fn get_indirect_light_color(scene: &Scene, hit: &Hit, ray: &Ray) -> Color
{
	let mut light_color = Color::new(0., 0., 0.);
	if ray.get_depth() == 0 {
		let sample = get_indirect_light_sample(hit.clone(), scene, &ray);
		light_color += sample.color * sample.weight * hit.color();
	} else {
		let mut indirect_dir = hit.norm() + random_unit_vector();
		if indirect_dir.length() < 0.01 {
			indirect_dir = hit.norm().clone();
		}
		indirect_dir = indirect_dir.normalize();
		let indirect_ray = Ray::new(hit.pos().clone(), indirect_dir, ray.get_depth() + 1);
		light_color = get_lighting_from_ray(scene, &indirect_ray) * hit.color();
	}
	light_color
}