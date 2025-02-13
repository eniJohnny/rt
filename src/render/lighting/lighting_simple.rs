use crate::{model::{
    materials::color::Color,
    maths::{hit::Hit, ray::Ray}, objects::lights::{light::Light, parallel_light::ParallelLight}, scene::Scene
}, render::skybox::get_skybox_color};

pub fn simple_lighting_from_hit(scene: &Scene, hit: &Option<Hit>, ray: &Ray, ambient: &Color, light: &ParallelLight) -> Color {
    if let Some(hit) = hit {
        return hit.color() * ambient + light.get_diffuse(hit) * hit.color();
    } else {
        get_skybox_color(scene, ray)
    }
}