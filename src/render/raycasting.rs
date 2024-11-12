use std::{collections::HashMap, iter};

use image::{ImageBuffer, Rgba};
use rand::Rng;

use crate::{
    model::{
        materials::{color::Color, texture::{Texture, TextureType}},
        maths::{hit::Hit, quaternion::Quaternion, ray::Ray, vec3::Vec3},
        scene::Scene,
        Element,
    }, ANTIALIASING, MAX_DEPTH, SCREEN_HEIGHT, SCREEN_WIDTH, USING_BVH
};

use super::{skysphere::get_skysphere_color,};

pub fn get_ray_debug(scene: &Scene, x: usize, y: usize, debug: bool) -> Ray {
    let width = (scene.camera().fov() / 2.).tan() * 2.;
    let height = width * SCREEN_HEIGHT as f64 / SCREEN_WIDTH as f64;
    // Centre de l'ecran
    let center: Vec3 = scene.camera().pos() + scene.camera().dir();

    // Coin superieur gauche, et les distances pour atteindre a partir de lui les coin superieur droit et inferieur gauche
    let top_left = center + scene.camera().u() * -width / 2. + scene.camera().v() * height / 2.;
    let left_to_right = scene.camera().u() * width;
    let top_to_bot = scene.camera().v() * height;

    let dir = &top_left
        - scene.camera().pos()
        - &top_to_bot
            * ((y as f64 / SCREEN_HEIGHT as f64)
                + rand::thread_rng().gen_range((0.)..ANTIALIASING))
        + &left_to_right
            * ((x as f64 / SCREEN_WIDTH as f64) + rand::thread_rng().gen_range((0.)..ANTIALIASING));
    let mut ray = Ray::new(scene.camera().pos().clone(), dir.normalize(), 0);
    ray.debug = debug;
    ray
}

pub fn get_ray(scene: &Scene, x: usize, y: usize) -> Ray {
    get_ray_debug(scene, x, y, false)
}

pub fn get_sorted_hit_from_t<'a>(scene: &'a Scene, ray: &Ray, t: &Option<Vec<f64>>, element: &'a Element) -> Option<Vec<Hit<'a>>> {
	let mut hits: Vec<Hit> = Vec::new();
	if let Some(t) = t {
		for dist in t {
			if *dist > 0.0 {
				let new_hit = Hit::new(
					element,
					*dist,
					ray.get_pos() + ray.get_dir() * (*dist - f64::EPSILON),
					ray.get_dir(),
					scene.textures(),
				);
				hits.push(new_hit);
			}
		}
	}
	if hits.len() == 0 {
		return None;
	}
	hits.sort_by(|a, b| a.dist().partial_cmp(b.dist()).unwrap());
	Some(hits)
}

pub fn get_closest_hit<'a>(scene: &'a Scene, ray: &Ray) -> Option<Hit<'a>> {
    let mut closest: Option<Hit> = None;
    let elements: &Vec<Element> = scene.elements();
    let composed_elements = scene.composed_elements();

    // TESTING PURPOSES
    // let elements;
    // if USING_BVH {
    //     elements = scene.non_bvh_elements();
    // } else {
    //     elements = scene.test_all_elements();
    // }
    // END TESTING PURPOSES

    for element in elements {
        let mut t = None;
		if scene.settings().displacement {
            if let Texture::Texture(file, TextureType::Float) = element.material().displacement() {
                t = element.shape().intersect_displacement(ray, element, scene);
            }
            else {
                t = element.shape().intersect(ray);
            }
        } else {
        	t = element.shape().intersect(ray);
        }
        if let Some(t) = t {
            for dist in t {
                if dist > 0.0 {
                    if let Some(hit) = &closest {
                        if &dist < hit.dist() {
                            let new_hit = Hit::new(
                                element,
                                dist,
                                ray.get_pos() + ray.get_dir() * (dist - f64::EPSILON),
                                ray.get_dir(),
                                scene.textures(),
                            );
                            if new_hit.opacity() > 0.5 {
                                closest = Some(new_hit);
                            }
                        }
                    } else {
                        let new_hit = Hit::new(
                            element,
                            dist,
                            ray.get_pos() + ray.get_dir() * (dist - f64::EPSILON),
                            ray.get_dir(),
                            scene.textures(),
                        );
                        if new_hit.opacity() > 0.5 {
                            closest = Some(new_hit);
                        }
                    }
                }
            }
        }
    }

    for composed in composed_elements {
        let mut t = None;
        let elems = composed.composed_shape().elements();
        for elem in elems {
            let shape = elem.shape();
            t = shape.intersect(ray);
            if let Some(t) = t {
                for dist in t {
                    if dist > 0.0 {
                        if let Some(hit) = &closest {
                            if &dist < hit.dist() {
                                let new_hit = Hit::new(
                                    elem,
                                    dist,
                                    ray.get_pos() + ray.get_dir() * (dist - f64::EPSILON),
                                    ray.get_dir(),
                                    scene.textures(),
                                );
                                if new_hit.opacity() > 0.5 {
                                    closest = Some(new_hit);
                                }
                            }
                        } else {
                            let new_hit = Hit::new(
                                elem,
                                dist,
                                ray.get_pos() + ray.get_dir() * (dist - f64::EPSILON),
                                ray.get_dir(),
                                scene.textures(),
                            );
                            if new_hit.opacity() > 0.5 {
                                closest = Some(new_hit);
                            }
                        }
                    }
                }
            }
        }
    }

    match closest {
        None => None,
        Some(mut hit) => {
            hit.map_textures(scene.textures());
            Some(hit)
        }
    }
}
