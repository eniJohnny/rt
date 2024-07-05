use crate::{
    model::{
        materials::color::Color,
        maths::{hit::Hit, ray::Ray, vec3::Vec3},
        objects::light::{Light, ParallelLight},
        scene::Scene,
    },
    render::raycasting::get_closest_hit,
};

pub fn simple_lighting_from_ray(
    scene: &Scene,
    ray: &Ray,
    ambient: &Color,
    light: &ParallelLight,
) -> Color {
    match get_closest_hit(scene, ray) {
        Some(hit) => simple_lighting_from_hit(&hit, ambient, light),
        //TODO : Handle BG on None
        None => Color::new(0., 0., 0.),
    }
}

pub fn simple_lighting_from_hit(hit: &Hit, ambient: &Color, light: &ParallelLight) -> Color {
    return hit.color() * ambient + light.get_diffuse(hit) * hit.color();
}
