use rand::Rng;
use crate::{
    bvh::traversal::recursive_traversal, model::{
        element::Element, materials::{
            color::Color,
            texture::{Texture, TextureType}
        }, maths::{hit::Hit, ray::Ray, vec3::Vec3}, scene::Scene, shapes::shape::Shape
    }, ANTIALIASING, SCREEN_HEIGHT, SCREEN_WIDTH, USING_BVH
};
use super::{
    lighting::{
        lighting_phong::phong_lighting_from_hit, lighting_real::global_lighting_from_hit, lighting_simple::simple_lighting_from_hit, lightning_norm::norm_lighting_from_hit
    },
    settings::ViewMode,
    skybox::get_skybox_color
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
					t.clone(),
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
    let mut t_list = match &closest {
        Some(hit) => hit.t_list().clone(),
        _ => vec![]
    };
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
        if let Some(t) = &t {
            // if ray.debug {
            //     println!("Hit element {} at dist {}", element.id(), t[0]);
            // }
            t_list.push((element, t.clone()));
            for dist in t {
                if closest.is_none() || dist > &0. && (closest.clone().unwrap().dist() < &0. || dist < closest.clone().unwrap().dist()) {
                    let new_hit = Hit::new(
                        element,
                        *dist,
                        ray.get_pos() + ray.get_dir() * (dist - f64::EPSILON),
                        ray.get_dir(),
                        scene.textures(),
                        t.clone(),
                    );
                    if new_hit.opacity() > 0.5 {
                        closest = Some(new_hit);
                    }
                }
            }
        }
    }
    if let Some(hit) = &mut closest {
        hit.set_t_list(t_list);
    }

    closest
}

pub fn get_closest_hit<'a>(scene: &'a Scene, ray: &Ray) -> Option<Hit<'a>> {
    let mut closest: Option<Hit> = None;
    
    if USING_BVH {
        // We first check every element that is not supported by the BVH (i.e. infinite shapes that cannot be contained in an AABB)
        closest = get_closest_hit_from_elements_with_index(scene, ray, closest, scene.elements(), scene.non_bvh_elements());
        // We then do a BVH traversal to check for intersections
        if let Some(root_node) = scene.bvh() {
            if let Some(hit) = recursive_traversal(ray, root_node, scene, closest.clone(), root_node.aabb().intersect(ray).unwrap_or(vec![]), 1) {
                if let Some(previous) = &closest {
                    if hit.dist() > &0. && (hit.dist() < previous.dist() || previous.dist() < &0.) {
                        closest = Some(hit);
                    }
                } else {
                    closest = Some(hit);
                }
            }
            
        }
    } else {
        // When we are not using the bvh, we just check for every element intersection
        closest = get_closest_hit_from_elements(scene, ray, closest, scene.elements());
    }
    
    match closest {
        None => None,
        Some(mut hit) => {
            if hit.dist() > &0. {
                // For optimization purposes, every texture that doesn't need to be mapped to check for the intersection is mapped once we have the final one.
                // Unfortunately, some properties like the norm, the opacity and such need to be processed for every possible intersection beforehand
                hit.map_textures(scene.textures());
                Some(hit)
            } else {
                None
            }
        }
    }
}

pub fn get_lighting_from_ray(scene: &Scene, ray: &Ray) -> Color {
    let hit = get_closest_hit(scene, ray);

    return match hit {
        Some(mut hit) => {
            if hit.element().shape().as_wireframe().is_some() {
                return Color::new(1., 1., 1.);
            }
			match &scene.settings().view_mode {
				ViewMode::Simple(ambient, light) => {
					simple_lighting_from_hit(&hit, ambient, light)
				}
				ViewMode::Norm => {
					norm_lighting_from_hit(&hit)
				},
				ViewMode::Phong => {
					phong_lighting_from_hit(scene, &hit, ray)
				},
				_ => global_lighting_from_hit(scene, &mut hit, ray)
			}
        },
        None => {
            get_skybox_color(scene, ray)
        },
    };
}
