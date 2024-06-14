use crate::{
    model::{
        materials::color::Color, maths::{hit::Hit, ray::Ray}, objects::light, scene::Scene
    },
    MAX_DEPTH,
};

use super::{
    lighting_sampling::{
        get_indirect_light_bucket, get_indirect_light_sample, get_reflected_light_sample, random_bounce, reflect_dir,
    },
    raycasting::{get_closest_hit, get_ray},
    restir::{Path, PathBucket, Sample},
};

pub fn get_lighting_from_ray(scene: &Scene, ray: &Ray) -> Color {
    match get_closest_hit(scene, ray) {
        Some(hit) => get_lighting_from_hit(scene, &hit, ray),
        //TODO : Handle BG on None
        None => Color::new(0., 0., 0.)
    }
}

pub fn get_lighting_from_hit(scene: &Scene, hit: &Hit, ray: &Ray) -> Color {
    let absorbed = (1.0 - hit.metalness() - hit.refraction()) * (1.0 - hit.emissive());

    let mut light_color: Color = hit.emissive() * hit.color();

    //Indirect Light
    if scene.indirect_lightning() && ray.get_depth() < MAX_DEPTH {
        let indirect_ray = random_bounce(hit, ray, hit.norm(), hit.roughness());
        light_color += get_lighting_from_ray(scene, &indirect_ray) * hit.color() * absorbed;
    }

    //Reflect Light
    let reflect_ray;
    if scene.imperfect_reflections() && ray.get_depth() < MAX_DEPTH {
        if hit.roughness() < f64::EPSILON {
            let dir = reflect_dir(ray.get_dir(), hit.norm());
            reflect_ray = Ray::new(hit.pos().clone(), dir, ray.get_depth() + 1);
        } else {
            reflect_ray =
                random_bounce(&hit, &ray, hit.norm(), hit.roughness());
        }
        let reflect_color = get_lighting_from_ray(scene, &reflect_ray);
        light_color += &reflect_color * hit.metalness() * hit.color()
            + reflect_color * absorbed;
    }

    light_color = light_color.clamp(0., 1.);
    light_color
}

pub fn get_real_lighting_old(scene: &Scene, sample: &Sample, ray: &Ray) -> Color {
    let hit = &sample.path.hit;

    let color = hit.color();
    let absorbed = (1.0 - hit.metalness() - hit.refraction()) * (1.0 - hit.emissive());

    let mut light_color: Color = hit.emissive() * color;
    for light in scene.lights() {
        if !light.is_shadowed(scene, &hit) {
            let diffuse = light.as_ref().get_diffuse(&hit);
            light_color = light_color + diffuse;
        }
    }

    let mut indirect_sample: Option<Sample> = None;

    if let Some(sample) = sample.path.indirect.clone() {
        indirect_sample = Some(*sample);
    } else if scene.indirect_lightning() && ray.get_depth() < MAX_DEPTH {
        let sample = get_indirect_light_sample(hit.clone(), scene, ray);
        if sample.weight > 0. {
            indirect_sample = Some(sample);
        }
    }
    if let Some(sample) = indirect_sample {
        let mut indirect_ray = Ray::new(
            hit.pos().clone(),
            (sample.path.hit.pos() - hit.pos()).normalize(),
            ray.get_depth() + 1,
        );
        indirect_ray.debug = ray.debug; 
        indirect_ray.set_sampling(false);
        light_color = light_color
            + get_real_lighting_old(scene, &sample, &indirect_ray);
    }

    let mut reflect_sample: Option<Sample> = None;

    light_color = light_color * absorbed * color;

    if let Some(sample) = sample.path.reflect.clone() {
        reflect_sample = Some(*sample);
    } else if scene.imperfect_reflections() && ray.get_depth() < MAX_DEPTH {
        let sample = get_reflected_light_sample(hit.clone(), scene, ray);
        if let Some(sample) = sample {
            reflect_sample = Some(sample);
        }
    }
    if let Some(sample) = reflect_sample {
        let mut reflect_ray = Ray::new(
            hit.pos().clone(),
            (sample.path.hit.pos() - hit.pos()).normalize(),
            ray.get_depth() + 1,
        );
        reflect_ray.debug = ray.debug; 
        reflect_ray.set_sampling(false);
        let reflect_light = get_real_lighting_old(scene, &sample, &reflect_ray);

        light_color = light_color
            + &reflect_light * hit.metalness() * color
            + reflect_light * absorbed;
    }

    light_color = light_color.clamp(0., 1.);
    light_color
}
