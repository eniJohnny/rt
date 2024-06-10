use crate::{
    model::{
        materials::Color,
        maths::{hit::Hit, ray::Ray},
        objects::light,
        scene::Scene,
    },
    MAX_DEPTH,
};

use super::{
    lighting_sampling::{
        get_indirect_light_bucket, get_indirect_light_sample, get_reflected_light_sample,
    },
    raycasting::{get_closest_hit, get_ray},
    restir::{Path, PathBucket, Sample},
};

// pub fn get_final_color(scene: &Scene, path: Path) -> Color {}

pub fn get_real_lighting(scene: &Scene, sample: &Sample, ray: &Ray) -> Color {
    let hit = &sample.path.hit;

    let material = hit.element().material();
    let absorbed = 1.0 - material.reflection_coef() - material.refraction_coef();
    let color = match material.needs_projection() {
        false => material.color(0, 0),
        true => {
            let point = hit.element().shape().projection(&hit);
            material.color(point.0, point.1)
        }
    };

    let mut light_color: Color = Color::new(0., 0., 0.);
    for light in scene.lights() {
        if !light.is_shadowed(scene, &hit) {
            let diffuse = light.as_ref().get_diffuse(&hit);
            light_color = light_color + diffuse * &color;
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
        let mut ray = Ray::new(
            hit.pos().clone(),
            (sample.path.hit.pos() - hit.pos()).normalize(),
            ray.get_depth() + 1,
        );
        ray.set_sampling(false);
        light_color = light_color
            + get_real_lighting(scene, &sample, &ray) * (sample.weight / (hit.dist() * hit.dist()));
        // / (hit.dist() * hit.dist())
    }

    let mut reflect_sample: Option<Sample> = None;

    light_color = light_color * absorbed;

    // if let Some(sample) = sample.path.reflect.clone() {
    //     reflect_sample = Some(*sample);
    // } else if scene.imperfect_reflections() && ray.get_depth() < MAX_DEPTH {
    //     let sample = get_reflected_light_sample(hit.clone(), scene, ray);
    //     if let Some(sample) = sample {
    //         reflect_sample = Some(sample);
    //     }
    // }
    // if let Some(sample) = reflect_sample {
    //     let mut ray = Ray::new(
    //         hit.pos().clone(),
    //         (sample.path.hit.pos() - hit.pos()).normalize(),
    //         ray.get_depth() + 1,
    //     );
    //     ray.set_sampling(false);
    //     let reflect_light = get_real_lighting(scene, &sample, &ray) * sample.weight;

    //     light_color = light_color
    //         + &reflect_light * material.reflection_coef() * &color
    //         + absorbed * reflect_light;
    // }

    light_color = light_color.clamp(0., 1.);
    light_color.apply_gamma();
    light_color
}
