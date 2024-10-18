use core::time;
use std::time::Instant;

use chrono::Duration;
use rand::Rng;

use crate::{
    bvh::{self, traversal::{get_closest_aabb_hit, new_traverse_bvh, traverse_bvh}}, model::{
        materials::color::Color,
        maths::{hit::Hit, ray::Ray, vec3::Vec3, vec_utils::{random_unit_vector, reflect_dir}},
        objects::light,
        scene::Scene, shapes::Shape,
    }, render::{raycasting::get_closest_hit, skysphere::get_skysphere_color}, FILTER, MAX_DEPTH, USING_BVH
};

pub fn get_lighting_from_ray(scene: &Scene, ray: &Ray) -> Color {
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

    return match hit {
        Some(hit) => {
            if hit.element().shape().as_wireframe().is_some() {
                return Color::new(1., 1., 1.);
            }
            let tmp = get_lighting_from_hit(scene, &hit, ray);
            tmp
        },
        None => {
            if FILTER == "cartoon" {
                return Color::new(1., 1., 1.);
            }
            get_skysphere_color(scene, ray)
        },
    };
}

pub fn fresnel_reflect_ratio(n1: f64, n2: f64, norm: &Vec3, ray: &Vec3, f0: f64, f90: f64) -> f64 {
    // Schlick aproximation
    let mut r0 = (n1 - n2) / (n1 + n2);
    r0 = r0 * r0;
    let mut cosX = -(norm.dot(&ray));
    if n1 > n2 {
        let n = n1 / n2;
        let sinT2 = n * n * (1.0 - cosX * cosX);
        // Total internal reflection
        if sinT2 > 1.0 {
            return f90;
        }
        cosX = (1.0 - sinT2).sqrt();
    }
    let x = 1.0 - cosX;
    let ret = r0 + (1.0 - r0) * x.powf(5.);

    // adjust reflect multiplier for object reflectivity
    f0 * (1.0 - ret) + f90 * ret
}

pub fn get_lighting_from_hit(scene: &Scene, hit: &Hit, ray: &Ray) -> Color {
    let absorbed = 1.0 - hit.metalness() - hit.refraction();
    if ray.debug {
        println!(
            "Metal : {}, Roughness: {}, Color: {}, Norm: {}, Emissive: {}, Opacity: {}, Refraction: {}",
            hit.metalness(),
            hit.roughness(),
            hit.color(),
            hit.norm(),
            hit.emissive(),
            hit.opacity(),
            hit.refraction()
        );
    }
    if hit.emissive() > f64::EPSILON {
        return hit.emissive() * hit.color();
    }
    let mut light_color = Color::new(0., 0., 0.);
    let fresnel_factor =
        fresnel_reflect_ratio(1., 1., hit.norm(), ray.get_dir(), 0., 1.0 - hit.roughness());
    let reflected = fresnel_factor * absorbed;

    let rand = rand::thread_rng().gen_range(0.0..1.0);
    if rand > reflected + hit.metalness() {
        // Indirect Light
        if scene.settings().indirect && ray.get_depth() < scene.settings().depth as u8 {
            let mut indirect_dir = hit.norm() + random_unit_vector();
            if indirect_dir.length() < 0.01 {
                indirect_dir = hit.norm().clone();
            }
            indirect_dir = indirect_dir.normalize();
            let indirect_ray = Ray::new(hit.pos().clone(), indirect_dir, ray.get_depth() + 1);
            light_color = get_lighting_from_ray(scene, &indirect_ray) * hit.color();
        }
    } else if scene.settings().reflections && ray.get_depth() < scene.settings().depth as u8 {
        let reflect_color;
        let dir = (reflect_dir(ray.get_dir(), hit.norm()) + random_unit_vector() * hit.roughness())
            .normalize();
        if dir.dot(hit.norm()) > f64::EPSILON {
            let reflect_ray = Ray::new(hit.pos().clone(), dir, ray.get_depth() + 1);
            reflect_color = get_lighting_from_ray(scene, &reflect_ray);
        } else {
            reflect_color = Color::new(0., 0., 0.);
        }
        if rand > hit.metalness() {
            light_color += reflect_color
        } else {
            light_color += reflect_color * hit.color();
        }
    }
    light_color
}