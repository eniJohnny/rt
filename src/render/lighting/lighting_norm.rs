use crate::{model::{materials::color::Color, maths::{hit::Hit, ray::Ray, vec3::Vec3}, scene::Scene}, render::skybox::get_skybox_color};

pub fn norm_lighting_from_hit(scene: &Scene, hit: &Option<Hit>, ray: &Ray) -> Color {
	if let Some(hit) = hit {
		let color = Color::from_vec3(&((hit.norm() + Vec3::new(1., 1., 1.)) * 0.5));
		color.clamp(0., 1.)
    } else {
        get_skybox_color(scene, ray)
    }
}