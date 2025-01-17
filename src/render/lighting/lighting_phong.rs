use crate::model::{
    materials::color::Color,
    maths::{hit::Hit, ray::Ray},
	scene::Scene,
};

pub fn phong_lighting_from_hit(scene: &Scene, hit: &Hit, ray: &Ray) -> Color {
	let mut color: Color = scene.ambient_light().color() * scene.ambient_light().intensity() * hit.color();
	for light in scene.lights() {
		if !light.light().is_shadowed(scene, &hit) {
			color += light.light().get_diffuse(&hit) * hit.color();
			color += light.light().get_specular(&hit, ray);
		}
	}
	(color).clamp(0., 1.)
}