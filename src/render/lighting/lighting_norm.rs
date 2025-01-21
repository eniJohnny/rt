use crate::model::{materials::color::Color, maths::{hit::Hit, vec3::Vec3}};

pub fn norm_lighting_from_hit(hit: &Hit) -> Color {
	let color = Color::from_vec3(&((hit.norm() + Vec3::new(1., 1., 1.)) * 0.5));
	color.clamp(0., 1.)
}