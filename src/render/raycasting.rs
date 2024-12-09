use rand::Rng;
use crate::{
    bvh::traversal::recursive_traversal, model::{
        materials::{
            color::Color,
            texture::{Texture, TextureType}
        },
        maths::{hit::Hit, ray::Ray, vec3::Vec3},
        scene::Scene,
        Element,
    },
    ANTIALIASING, FILTER, SCREEN_HEIGHT, SCREEN_WIDTH, USING_BVH
};
use super::{
    lighting::{
        lighting_real::global_lighting_from_hit,
        simple::simple_lighting_from_hit
    },
    settings::ViewMode,
    skysphere::get_skysphere_color
};

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

pub fn get_closest_hit_from_elements<'a>(scene: &'a Scene, ray: &Ray, closest: Option<Hit<'a>>, elements: &'a Vec<Element>) -> Option<Hit<'a>> {
    let elements_index = (0..elements.len()).collect();
    return get_closest_hit_from_elements_with_index(scene, ray, closest, elements, &elements_index);
}

pub fn get_closest_hit_from_elements_with_index<'a>(scene: &'a Scene, ray: &Ray, mut closest: Option<Hit<'a>>, elements: &'a Vec<Element>, elements_index: &Vec<usize>) -> Option<Hit<'a>> {
    for index in elements_index {
        let element = &elements[*index];
        let t;
		if scene.settings().displacement {
            if let Texture::Texture(_file, TextureType::Float) = element.material().displacement() {
                t = element.shape().intersect_displacement(ray, &element, scene);
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
                    if closest.is_none() || &dist < closest.clone().unwrap().dist() {
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

    closest
}

pub fn get_closest_hit<'a>(scene: &'a Scene, ray: &Ray) -> Option<Hit<'a>> {
    let mut closest: Option<Hit> = None;
    
    if USING_BVH {
        closest = get_closest_hit_from_elements_with_index(scene, ray, closest, scene.elements(), scene.non_bvh_elements());
        for composed in scene.composed_elements() {
            closest = get_closest_hit_from_elements(scene, ray, closest, composed.composed_shape().elements());
        }
        if let Some(root_node) = scene.bvh() {
            closest = recursive_traversal(ray, root_node, scene, closest);
        }
    } else {
        closest = get_closest_hit_from_elements(scene, ray, closest, scene.elements());
        for composed in scene.composed_elements() {
            closest = get_closest_hit_from_elements(scene, ray, closest, composed.composed_shape().elements());
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

pub fn get_lighting_from_ray(scene: &Scene, ray: &Ray) -> Color {
    let hit = get_closest_hit(scene, ray);

    return match hit {
        Some(hit) => {
            if hit.element().shape().as_wireframe().is_some() {
                return Color::new(1., 1., 1.);
            }
            if let ViewMode::Simple(ambient, light) = &scene.settings().view_mode {
                simple_lighting_from_hit(&hit, ambient, light)
            } else {
                global_lighting_from_hit(scene, &hit, ray)
            }
        },
        None => {
            if FILTER == "cartoon" {
                return Color::new(1., 1., 1.);
            }
            get_skysphere_color(scene, ray)
        },
    };
}
