use crate::model::{
    materials::color::Color,
    maths::{hit::Hit, ray::Ray},
    objects::light::{Light, ParallelLight}, scene::Scene,
};

pub fn simple_lighting_from_hit(scene: &Scene, hit: &Hit, ambient: &Color, default_light: &ParallelLight, ray: &Ray) -> Color {
	let mut color: Color = ambient * hit.color();
	for light in scene.lights() {
		if !light.light().is_shadowed(scene, &hit) {
			color += light.light().get_diffuse(&hit) * hit.color();
			color += light.light().get_specular(&hit, ray);
		}
	}
	if scene.lights().len() == 0 {
		color += default_light.get_diffuse(hit) * hit.color();
	}
	(color).clamp(0., 1.)
}