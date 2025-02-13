use crate::{model::{materials::color::Color, maths::{hit::Hit, ray::Ray}, scene::Scene}, render::skybox::get_skybox_color};

pub fn projection_lighting_from_hit(scene: &Scene, hit: &mut Option<Hit>, ray: &Ray) -> Color {
    if let Some(hit) = hit {
        let projection = hit.projection();
        Color::new(projection.u, projection.v, 0.)
    } else {
        get_skybox_color(scene, ray)
    }
}