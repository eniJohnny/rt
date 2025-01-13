use crate::model::{
    materials::color::Color,
    maths::hit::Hit,
    objects::light::{Light, ParallelLight},
};

pub fn simple_lighting_from_hit(hit: &Hit, ambient: &Color, light: &ParallelLight) -> Color {
    return hit.color() * ambient + light.get_diffuse(hit) * hit.color();
}