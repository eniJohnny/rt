use std::f64::EPSILON;

use rand::Rng;
use crate::{
    model::{
        materials::color::Color, maths::{
            hit::Hit,
            ray::Ray,
            vec3::Vec3,
            vec_utils::{random_unit_vector, reflect_dir}
        }, scene::Scene, Element
    },
    render::raycasting::get_lighting_from_ray
};
pub fn fresnel_reflect_ratio(n1: f64, n2: f64, norm: &Vec3, ray: &Vec3, reflectivity: f64, debug: bool) -> f64 {
    // Schlick aproximation
    let mut r0 = (n1 - n2) / (n1 + n2);
    r0 = r0 * r0;
    let mut cos_x = (norm.dot(&ray)).abs();
	if debug {
		println!("cos_x {}, norm {} {} {}, ray {} {} {}", cos_x, norm.x(), norm.y(), norm.z(), ray.x(), ray.y(), ray.z());
	}
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
	if debug {
		println!("r0 {}, x {}", r0, x);
	}
    let ret = r0 + (1.0 - r0) * x.powf(5.);
    // adjust reflect multiplier for object reflectivity
    reflectivity * ret
}

pub fn global_lighting_from_hit(scene: &Scene, hit: &mut Hit, ray: &Ray) -> Color {
	if ray.debug {
		println!("Lighting is being done");
	}
    if ray.debug {
        println!(
            "Metal : {}, Roughness: {}, Color: {}, Norm: {}, Emissive: {}, Opacity: {}",
            hit.metalness(),
            hit.roughness(),
            hit.color(),
            hit.norm(),
            hit.emissive(),
            hit.opacity(),
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
			is_inside = false;
		} 
		let parent_element_index = if let Some(parent) = get_parent(hit.t_list().clone(), *hit.dist()) {
			parent.material().refraction()
		} else {
			1.
		};
		if is_inside {
			current_refraction_index = hit.element().material().refraction();
			next_refraction_index = parent_element_index;
			hit.set_norm(-hit.norm().clone());
			
		} else {
			current_refraction_index = parent_element_index;
			next_refraction_index = hit.element().material().refraction();
		}
	}
	

	let fresnel_factor;
	if ray.debug {
		fresnel_factor =
			fresnel_reflect_ratio(current_refraction_index, next_refraction_index, &hit.norm(), ray.get_dir(), 1.0 - hit.roughness(), true);	
	} else {
		fresnel_factor =
			fresnel_reflect_ratio(current_refraction_index, next_refraction_index, &hit.norm(), ray.get_dir(), 1.0 - hit.roughness(), false);
	}
	if ray.debug {
		println!("fresnel {}", fresnel_factor);
	}
	
	let reflected = fresnel_factor * (1.0 - hit.metalness());
	let absorbed = 1.0 - hit.metalness() - reflected;
	if ray.debug {
		println!("Reflected {}, absorbed {}, fresnel_factor: {}", reflected, absorbed, fresnel_factor);
	}
    let rand = rand::thread_rng().gen_range(0.0..1.0);
    if rand > absorbed && scene.settings().reflections {
		// Reflected Light
        if rand > absorbed + hit.metalness() {
            light_color += get_reflected_light_color(scene, hit, ray);
        } else {
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
		let indirect_ray = Ray::new(hit.pos().clone(), indirect_dir, ray.get_depth() + 1);
		light_color = get_lighting_from_ray(scene, &indirect_ray) * hit.color();
	}
	light_color
}

fn get_reflected_light_color(scene: &Scene, hit: &Hit, ray: &Ray) -> Color
{
	let reflect_color;
	let dir = (reflect_dir(ray.get_dir(), hit.norm()) + random_unit_vector() * hit.roughness())
		.normalize();
	if dir.dot(hit.norm()) > f64::EPSILON {
		let reflect_ray = Ray::new(hit.pos().clone(), dir, ray.get_depth() + 1);
		reflect_color = get_lighting_from_ray(scene, &reflect_ray);
	} else {
		reflect_color = Color::new(0., 0., 0.);
	}
	reflect_color
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

pub fn get_parent<'a>(mut t_s: Vec<(&Element, Vec<f64>)>, closest_dist: f64) -> Option<&Element> {
	for (_, t) in t_s.iter_mut() {
		for dist in t.iter_mut() {
			*dist -= closest_dist;
		}
	}
    let mut closest: Option<(&Element, f64)> = None;
	for (elem, t) in t_s {
		if t.len() > 0
		{
			if t.len() % 2 == 0
			{
				let mut nb_t_positives = 0;
				for dist in &t
				{
					if dist > &0.
					{
						nb_t_positives += 1;
					}
				}
				if nb_t_positives % 2 == 0
				{
					for dist in t
					{
						if &dist > &0.
						{
							if let Some((_, closest_dist)) = closest
							{
								if &dist < &closest_dist
								{
									closest = Some((elem, dist));
								}
							}
							else
							{
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