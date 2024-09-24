use std::fmt::Debug;

use rand::Rng;

use crate::{
    model::{
        materials::color::Color,
        maths::{hit::Hit, ray::Ray, vec3::Vec3},
        objects::light,
        scene::Scene,
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
        Some(hit) => get_lighting_from_hit(scene, &hit, ray),
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

pub fn get_lighting_from_hit(scene: &Scene, hit: &Hit, ray: &Ray) -> Color {
    let absorbed = 1.0 - hit.metalness() - hit.refraction();

    if ray.debug {
        println!(
            "Metal : {}, Roughness: {}, Color: {}, Norm: {}, Emissive: {}, Opacity: {}, Refraction: {}",
            hit.metalness(),
            hit.roughness(),
            hit.color(),
            hit.norm(),
            hit.emissive(),
            hit.opacity(),
            hit.refraction()
        );
    }

    if hit.emissive() > f64::EPSILON {
        return hit.emissive() * hit.color();
    }
    let mut light_color = Color::new(0., 0., 0.);
    let fresnel_factor =
        fresnel_reflect_ratio(1., 1.52, hit.norm(), ray.get_dir(), 0., 1.0 - hit.roughness());
    let reflected = fresnel_factor * absorbed;
    let refracted = hit.refraction();
    let rand = rand::thread_rng().gen_range(0.0..1.0);
    if rand < reflected && scene.imperfect_reflections() && ray.get_depth() < MAX_DEPTH {
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
        if rand > hit.metalness() {
            light_color += reflect_color
        } else {
            light_color += reflect_color * hit.color();
        }
	} else if rand < refracted + reflected && scene.imperfect_reflections() && ray.get_depth() < MAX_DEPTH {
		// Refracted Light
		// let refraction_ratio = 1.52;
		// let refract_color;
		// if let Some(refracted_dir) = refract(ray.get_dir(), hit.norm(), refraction_ratio) {
		// 	let refract_ray = Ray::new(hit.pos().clone(), refracted_dir, ray.get_depth() + 1);
		// 	refract_color = get_lighting_from_ray(scene, &refract_ray);
		// 	// refract_color = Color::new(0., 1., 0.);
		// } else {
		// 	refract_color = Color::new(0., 0., 0.);
		// }
		// light_color += refract_color * (1.0 - hit.metalness());

		// Try to refract the ray
		let refract_color;
		let refraction_result = refract(ray.get_dir().clone(), hit.norm().clone(), 1., 1.52);
		if let Some(refracted_ray) = refraction_result {
			let refract_ray = Ray::new(hit.pos().clone() - hit.norm() * 0.01, refracted_ray, ray.get_depth() + 1);
			refract_color = get_lighting_from_ray(scene, &refract_ray);
		} else {
			// Handle total internal reflection, fallback to reflection
			refract_color = Color::new(0., 0., 0.);
		}
		light_color += refract_color * (1.0 - hit.metalness());

	} else {
        // Indirect Light
        if scene.indirect_lightning() && ray.get_depth() < MAX_DEPTH {
            // if ray.get_depth() == 0 {
            //     let sample = get_indirect_light_sample(hit.clone(), scene, &ray);
            //     light_color += sample.color * sample.weight * hit.color();
            // } else {
            let mut indirect_dir = hit.norm() + random_unit_vector();
            if indirect_dir.length() < 0.01 {
                indirect_dir = hit.norm().clone();
            }
            indirect_dir = indirect_dir.normalize();
            let indirect_ray = Ray::new(hit.pos().clone(), indirect_dir, ray.get_depth() + 1);
            light_color = get_lighting_from_ray(scene, &indirect_ray) * hit.color();
            // }
        }
    }
	light_color
}

// fn refract(v: &Vec3, n: &Vec3, eta: f64) -> Option<Vec3> {
// 	let uv = v.clone().normalize();
// 	let dt = uv.dot(n);
// 	let discriminant = 1.0 - eta * eta * (1.0 - dt * dt);
// 	// if discriminant > 0.0 {
// 		Some(eta * (uv - n * dt) - n * discriminant.sqrt())
// 	// } else {
// 	// 	None // Total internal reflection
// 	// }
// }

fn refract(incoming: Vec3, normal: Vec3, n1: f64, n2: f64) -> Option<Vec3> {
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
