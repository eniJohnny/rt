use crate::model::{
    materials::Color,
    maths::{hit::Hit, ray::Ray},
    scene::Scene,
};

pub fn apply_lighting(hit: &Hit, scene: &Scene, ray: &Ray) -> Color {
    let material = hit.element().material();
    let color = match material.needs_projection() {
        false => material.color(0, 0),
        true => {
            let point = hit.element().shape().projection(&hit);
            material.color(point.0, point.1)
        }
    };
    let mut light_color: Color = Color::new(0., 0., 0.);
    light_color =
        light_color + scene.ambient_light().intensity() * scene.ambient_light().color() * &color;
    for light in scene.lights() {
        if !light.is_shadowed(scene, &hit) {
            light_color = light_color + light.as_ref().get_diffuse(&hit) * &color;
            light_color = light_color + light.as_ref().get_specular(&hit, ray);
        }
    }
    (light_color).clamp(0., 1.)
}
