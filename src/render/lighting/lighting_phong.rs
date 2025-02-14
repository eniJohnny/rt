use core::f64;

use crate::{model::{
    element::Element, materials::{color::Color, diffuse::Diffuse}, maths::{hit::Hit, ray::Ray, vec_utils::reflect_dir}, scene::Scene, shapes::plane::Plane
}, render::{raycasting::get_lighting_from_ray, skybox::get_skybox_color}, BOUNCE_OFFSET};

pub fn phong_lighting_from_hit(scene: &Scene, hit: &Option<Hit>, ray: &Ray) -> Color {
	let mut cam_color = Color::new(0., 0., 0.);
	if ray.get_pos() == scene.camera().pos() {
		let incoming_dir = reflect_dir(ray.get_dir(), scene.camera().dir());
		let mut fake_ray = Ray::new(*scene.camera().pos() - incoming_dir, incoming_dir, 0);
		let elem = Element::new(Box::new(
			Plane::new(*scene.camera().pos(), *scene.camera().dir())), Diffuse::default());
		let hit: Hit<'_> = Hit::new(&elem, 0., *scene.camera().pos(), fake_ray.get_dir(), scene.textures(), vec![0.]);
		for light in scene.lights() {
			let throughput = light.light().throughput(scene, &hit);
			if throughput.length() > f64::EPSILON {
				if ray.debug {
					fake_ray.debug = true;
				}
				let specular = light.light().get_specular(&hit, &fake_ray);
				let diffuse = light.light().get_diffuse(&hit);
				cam_color += diffuse + specular;
				cam_color = cam_color * Color::from_vec3(&throughput);
			}
		}
	}
	if let Some(hit) = hit {
		let mut color: Color = scene.ambient_light().color() * scene.ambient_light().intensity() * hit.color();
	
		for light in scene.lights() {
			let throughput = light.light().throughput(scene, &hit);
			if throughput.length() > f64::EPSILON {
				color += light.light().get_diffuse(&hit) * hit.color();
				color += light.light().get_specular(&hit, ray);
				color = color * Color::from_vec3(&throughput);
			}
		}
		if hit.opacity() < 1. - f64::EPSILON {
			let light_through = get_lighting_from_ray(scene, &Ray::new(hit.pos().clone() + *ray.get_dir() * BOUNCE_OFFSET, ray.get_dir().clone(), ray.get_depth()));
			color = color * hit.opacity() + hit.color() * light_through * (1. - hit.opacity());
		}
		(color + cam_color).clamp(0., 1.)
	} else {
        get_skybox_color(scene, ray) + cam_color
	}
	
}