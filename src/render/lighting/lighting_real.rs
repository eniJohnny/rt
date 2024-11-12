use rand::Rng;
use crate::{
    model::{
        maths::{
            hit::Hit,
            ray::Ray,
            vec3::Vec3,
            vec_utils::{random_unit_vector, reflect_dir}
        },
        materials::color::Color,
        scene::Scene,
    },
    render::raycasting::get_lighting_from_ray
};

pub fn fresnel_reflect_ratio(n1: f64, n2: f64, norm: &Vec3, ray: &Vec3, f0: f64, f90: f64) -> f64 {
    // Schlick aproximation
    let mut r0 = (n1 - n2) / (n1 + n2);
    r0 = r0 * r0;
    let mut cos_x = -(norm.dot(&ray));
    if n1 > n2 {
        let n = n1 / n2;
        let sin_t2 = n * n * (1.0 - cos_x * cos_x);
        // Total internal reflection
        if sin_t2 > 1.0 {
            return f90;
        }
        cos_x = (1.0 - sin_t2).sqrt();
    }
    let x = 1.0 - cos_x;
    let ret = r0 + (1.0 - r0) * x.powf(5.);

    // adjust reflect multiplier for object reflectivity
    f0 * (1.0 - ret) + f90 * ret
}

pub fn global_lighting_from_hit(scene: &Scene, hit: &Hit, ray: &Ray) -> Color {
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
	let parent_element_index = if let Some(_) = hit.parent_element() {
		dbg!("Parent");
		1.52
	} else {
		1.
	};
	if is_inside {
		current_refraction_index = hit.refraction();
		next_refraction_index = parent_element_index;
		normal = -hit.norm().clone();
	} else {
		current_refraction_index = parent_element_index;
		next_refraction_index = hit.refraction();
		normal = hit.norm().clone();
	}

	let fresnel_factor =
        fresnel_reflect_ratio(current_refraction_index, next_refraction_index, &normal, ray.get_dir(), 0., 1.0 - hit.roughness());
	let reflected = fresnel_factor * absorbed;
    let rand = rand::thread_rng().gen_range(0.0..1.0);
    if rand < reflected && scene.settings().reflections && ray.get_depth() < scene.settings().depth as u8 {
		// Reflected Light
        if rand > hit.metalness() {
            light_color += get_reflected_light_color(scene, hit, ray);
        } else {
            light_color += get_reflected_light_color(scene, hit, ray) * hit.color();
        }
    } else {
		let rand = rand::thread_rng().gen_range(0.0..1.0);
		if rand < hit.transparency() && ray.get_depth() < scene.settings().depth as u8 {
			// Refracted Light
			// light_color += Color::new(1., 0., 0.)
			light_color += get_refracted_light_color(scene, hit, ray, current_refraction_index, next_refraction_index, &normal);
		} else if scene.settings().indirect && ray.get_depth() < scene.settings().depth as u8 {
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
