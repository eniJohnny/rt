use core::f64;
use std::f64::EPSILON;

use rand::Rng;
use crate::{
    model::{
        element::Element, materials::color::Color, maths::{
            hit::Hit,
            ray::Ray,
            vec3::Vec3,
            vec_utils::{random_unit_vector, reflect_dir}
        }, scene::Scene
    },
    render::raycasting::get_lighting_from_ray, BOUNCE_OFFSET
};
pub fn fresnel_reflect_ratio(n1: f64, n2: f64, norm: &Vec3, ray: &Vec3, reflectivity: f64) -> f64 {
    // Schlick aproximation
    let mut r0 = (n1 - n2) / (n1 + n2);
    r0 = r0 * r0;
    let mut cos_x = (norm.dot(&ray)).abs();
    if n1 > n2 {
        let n = n1 / n2;
        let sin_t2 = n * n * (1.0 - cos_x * cos_x);
        // Total internal reflection
        if sin_t2 > 1.0 {
            return reflectivity;
        }
        cos_x = (1.0 - sin_t2).sqrt();
    }
    let x = 1.0 - cos_x;
    let ret = r0 + (1.0 - r0) * x.powf(5.);
    // adjust reflect multiplier for object reflectivity
    reflectivity * ret
}

pub fn get_refraction_indices(hit: &mut Hit, ray: &Ray) -> (f64, f64) {
	let mut t_s = hit.t_list().clone();
	let mut t_final: Vec<(&Element, Vec<f64>)> = vec![];
	for (elem, mut t) in t_s.clone() {
		// if ray.debug {
		// 	println!("Checking composed id for elem {}", elem.id());
		// }
		if let Some(composed_id) = elem.composed_id() {
			// if ray.debug {
			// 	println!("Checking for other intersects for {}", composed_id);
			// }
			let mut first = false;
			for (elem2, t2) in &mut t_s {
				if let Some(composed_id2) = elem2.composed_id() {
					// if ray.debug {
					// 	println!("Composed found {} for elem {}", composed_id2, elem2.id());
					// }
					if composed_id == composed_id2{
						if elem2.id() == elem.id() {
							// if ray.debug {
							// 	println!("First");
							// }
							first = true;
						} else {
							// if ray.debug {
							// 	println!("Not same element");
							// }
							if !first {
								// if ray.debug {
								// 	println!("Not first");
								// }
								break;
							}
							// if ray.debug {
							// 	println!("Appending hits");
							// }
							t.append(t2);
						}
					}
				}
			}
			if first {
				t_final.push((elem, t));
			}
		} else {
			t_final.push((elem, t));
		}
	}
	// if ray.debug {
	// 	println!("Current parent");
	// }
	let current_parent_index = if let Some(parent) = get_parent_debug(t_final.clone(), hit.dist() - BOUNCE_OFFSET, ray.debug) {
		parent.material().refraction()
	} else {
		1.0
	};
	// if ray.debug {
	// 	println!("Next parent");
	// }
	let next_parent_index = if let Some(parent) = get_parent_debug(t_final, hit.dist() + BOUNCE_OFFSET, ray.debug) {
		parent.material().refraction()
	} else {
		1.0
	};
	(current_parent_index, next_parent_index)

}

pub fn global_lighting_from_hit(scene: &Scene, hit: &mut Hit, ray: &Ray) -> Color {
    if ray.debug {
        println!(
            "Metal : {}, Roughness: {}, Color: {}, Norm: {}, Emissive: {}, Opacity: {}, Refraction index: {}, Transparancy {}",
            hit.metalness(),
            hit.roughness(),
            hit.color(),
            hit.norm(),
            hit.emissive(),
            hit.opacity(),
			hit.element().material().refraction(),
			hit.transparency()
        );
    }
    if hit.emissive() > f64::EPSILON {
        return hit.emissive() * hit.color();
    }
    let mut light_color = Color::new(0., 0., 0.);
	if ray.get_depth() >= scene.settings().depth as u8 {
		return light_color;
	}

	let mut current_refraction_index = 1.;
	let mut next_refraction_index = 1.;
	if hit.transparency() > EPSILON {
		(current_refraction_index, next_refraction_index) = get_refraction_indices(hit, ray);
		if ray.debug {
			println!("Current {}, next {}", current_refraction_index, next_refraction_index);
		}
	}

	let fresnel_factor = fresnel_reflect_ratio(current_refraction_index, next_refraction_index, &hit.norm(), ray.get_dir(), 1.0 - hit.roughness());
	
	let reflected = fresnel_factor * (1.0 - hit.metalness());
	let absorbed = 1.0 - hit.metalness() - reflected;
    let rand = rand::thread_rng().gen_range(0.0..1.0);
    if rand > absorbed && scene.settings().reflections {
        if rand > absorbed + hit.metalness() {
			// Normal reflection
            light_color += get_reflected_light_color(scene, hit, ray);
        } else {
			// Metal reflection
            light_color += get_reflected_light_color(scene, hit, ray) * hit.color();
        }
    } else {
		if rand < absorbed * hit.transparency() {
			// Refracted Light
			light_color += get_refracted_light_color(scene, hit, ray, current_refraction_index, next_refraction_index, &hit.norm());
		} else if scene.settings().indirect {
			// Indirect Light
			light_color += get_indirect_light_color(scene, hit, ray);
        }
    }
    light_color
}



fn get_indirect_light_color(scene: &Scene, hit: &Hit, ray: &Ray) -> Color
{
	let mut light_color = Color::new(0., 0., 0.);
	if scene.settings().indirect && ray.get_depth() < scene.settings().depth as u8 {
		let mut indirect_dir = hit.norm() + random_unit_vector();
		if indirect_dir.length() < 0.01 {
			indirect_dir = hit.norm().clone();
		}
		indirect_dir = indirect_dir.normalize();
		let mut indirect_ray = Ray::new(hit.pos().clone() + hit.norm() * BOUNCE_OFFSET, indirect_dir, ray.get_depth() + 1);
		indirect_ray.debug = ray.debug;
		light_color = get_lighting_from_ray(scene, &indirect_ray) * hit.color();
	}
	light_color
}

fn get_reflected_light_color(scene: &Scene, hit: &Hit, ray: &Ray) -> Color
{
	let reflect_color;
	let dir = (reflect_dir(ray.get_dir(), hit.norm()) + random_unit_vector() * hit.roughness() * hit.roughness())
		.normalize();
	if dir.dot(hit.norm()) > f64::EPSILON {
		let reflect_ray = Ray::new(hit.pos().clone() + hit.norm() * BOUNCE_OFFSET, dir, ray.get_depth() + 1);
		reflect_color = get_lighting_from_ray(scene, &reflect_ray);
	} else {
		reflect_color = Color::new(0., 0., 0.);
	}
	reflect_color
}
fn refract_dir(incoming: &Vec3, normal: &Vec3, n1: f64, n2: f64, roughness: f64) -> Option<Vec3>
{
    let n = n1 / n2;
    let mut cos_i = incoming.dot(&normal);
	if cos_i > 0. {
		cos_i = -cos_i;
	}
    let sin_t2 = 1.0 - n * n * (1.0 - cos_i * cos_i);

    // Check for total internal reflection
    if sin_t2 < 0.0 {
        return None;
    }
    let refracted = (incoming + cos_i * normal) * sin_t2 - normal * sin_t2.sqrt();
	let with_roughness = refracted.clone() + random_unit_vector() * roughness * roughness;
	if with_roughness.length() < 0.01 {
		return Some(refracted.normalize());
	}
    Some(with_roughness.normalize())
}

fn get_refracted_light_color(scene: &Scene, hit: &Hit, ray: &Ray, n1: f64, n2: f64, normal: &Vec3) -> Color
{
	let mut refract_color = Color::new(0., 0., 0.);
	let refraction_result = refract_dir(&ray.get_dir(), normal, n1, n2, hit.roughness());
	if let Some(refracted_ray) = refraction_result {
		let mut refract_ray = Ray::new(hit.pos().clone() - normal * BOUNCE_OFFSET, refracted_ray.clone(), ray.get_depth() + 1);
		refract_ray.debug = ray.debug;
		refract_color = get_lighting_from_ray(scene, &refract_ray);
	}
	refract_color
}

pub fn get_parent<'a>(t_s: Vec<(&Element, Vec<f64>)>, closest_dist: f64) -> Option<&Element> {
	get_parent_debug(t_s, closest_dist, false)
}

pub fn get_parent_debug<'a>(mut t_s: Vec<(&Element, Vec<f64>)>, closest_dist: f64, debug: bool) -> Option<&Element> {
	for (_, t) in t_s.iter_mut() {
		for dist in t.iter_mut() {
			*dist -= closest_dist;
		}
	}
    let mut closest: Option<(&Element, f64)> = None;
	// if debug {
	// 	println!("Get parent");
	// }

	for (elem, t) in t_s {
		if debug {
			// print!("For element {}, nb_hit {} : ", elem.id(), t.len());
			// for dist in &t {
			// 	print!("{} ", dist);
			// }
			// println!();
		}
		if t.len() > 1 {
			if t.len() % 2 == 0 {
				let mut nb_t_positives = 0;
				for dist in &t {
					if dist > &0.{
						nb_t_positives += 1;
					}
				}
				if nb_t_positives % 2 != 0 {
					for dist in t {
						if &dist > &0. {
							if let Some((_, closest_dist)) = closest {
								if &dist < &closest_dist {
									// if debug {
									// 	println!("Closest is {} at dist {}", elem.id(), dist);
									// }
									closest = Some((elem, dist));
								}
							}
							else {
								// if debug {
								// 	println!("Closest is {} at dist {}", elem.id(), dist);
								// }
								closest = Some((elem, dist));
							}
						}
					}
				}
			}
		}
	}
	match closest
	{
		Some((elem, _)) => Some(elem),
		None => None,
	}
}