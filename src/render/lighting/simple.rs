use crate::{
    bvh::traversal::new_traverse_bvh, model::{
        materials::color::Color,
        maths::{hit::Hit, ray::Ray, vec3::Vec3},
        objects::light::{Light, ParallelLight},
        scene::Scene, shapes::Shape,
    }, render::{raycasting::get_closest_hit, skysphere::get_skysphere_color}, FILTER, USING_BVH
};

pub fn simple_lighting_from_ray(
    scene: &Scene,
    ray: &Ray,
    ambient: &Color,
    light: &ParallelLight,
) -> Color {
    let hit;

    match USING_BVH {
        true => {
            let node = scene.bvh().as_ref().unwrap();
            hit = new_traverse_bvh(ray, Some(node), scene);
        },
        false => {
            hit = get_closest_hit(scene, ray);
        },
    };

    match hit {
        Some(hit) => {
            if hit.element().shape().as_wireframe().is_some() {
                return Color::new(1., 1., 1.);
            }
            simple_lighting_from_hit(&hit, ambient, light)
        }
        //TODO : Handle BG on None
        None => {
            if FILTER == "cartoon" {
                return Color::new(1., 1., 1.);
            }
            get_skysphere_color(scene, ray)
        }
    }
}

pub fn simple_lighting_from_hit(hit: &Hit, ambient: &Color, light: &ParallelLight) -> Color {

    return hit.color() * ambient + light.get_diffuse(hit) * hit.color();
}