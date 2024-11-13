use crate::{
    model::{
        materials::color::Color,
        maths::{hit::Hit, ray::Ray, vec3::Vec3},
        objects::light::{Light, ParallelLight},
        scene::Scene, shapes::Shape,
    }, render::{raycasting::get_closest_hit, skysphere::get_skysphere_color}, FILTER, USING_BVH
};

pub fn simple_lighting_from_hit(hit: &Hit, ambient: &Color, light: &ParallelLight) -> Color {

    return hit.color() * ambient + light.get_diffuse(hit) * hit.color();
}