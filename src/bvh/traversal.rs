use crate::{model::{materials::texture::{Texture, TextureType}, maths::{hit::{self, Hit}, ray::Ray}, scene::{self, Scene}, shapes::{aabb::Aabb, Shape}, Element}, render::raycasting::get_closest_hit, USING_BVH};

use super::node::Node;

fn recursive_traversal(ray: &Ray, mut node: &Node, scene: &Scene, non_bvh_dist: f64) -> HitInfo {
    if let Some(t) = node.aabb().intersect(ray) {
        if t[0] > non_bvh_dist {
            return HitInfo::new();
        }
        if node.is_leaf() {
            // Node is now a leaf node and closest hit is part of bvh
            let mut closest_hit = HitInfo::new();

            // Check for closest bvh hit
            for &element_index in node.elements() {
                let element = scene.get_element(element_index);
                let hit;
                if scene.settings().displacement {
                    if let Texture::Texture(file, TextureType::Float) = element.material().displacement() {
                        hit = element.shape().intersect_displacement(ray, element, scene);
                    }
                    else {
                        hit = element.shape().intersect(ray);
                    }
                } else {
                    hit = element.shape().intersect(ray);
                }
                
                if let Some(hit) = hit {
                    let tmin = hit[0];
                    let tmax = if hit.len() > 1 {hit[1]} else {tmin};

                    if (tmin == closest_hit.tmin && tmax < closest_hit.tmax) || tmin < closest_hit.tmin {
                        closest_hit.element_index = element_index;
                        closest_hit.tmin = tmin;
                        closest_hit.tmax = tmax;
                    }
                }
            }

            if closest_hit.tmin < std::f64::MAX {
                return closest_hit;
            }
        } else {
            let mut hit_info;
            if let Some(a) = node.a() {
                if let Some(b) = node.b() {
                    let dist_a = a.aabb().distance(ray.get_pos());
                    let dist_b = b.aabb().distance(ray.get_pos());
                    if dist_a < dist_b {
                        hit_info = recursive_traversal(ray, a, scene, non_bvh_dist);
                        if hit_info.tmin < std::f64::MAX {
                            return hit_info;
                        }
                        hit_info = recursive_traversal(ray, b, scene, non_bvh_dist);
                        if hit_info.tmin < std::f64::MAX {
                            return hit_info;
                        }
                    } else {
                        hit_info = recursive_traversal(ray, b, scene, non_bvh_dist);
                        if hit_info.tmin < std::f64::MAX {
                            return hit_info;
                        }
                        hit_info = recursive_traversal(ray, a, scene, non_bvh_dist);
                        if hit_info.tmin < std::f64::MAX {
                            return hit_info;
                        }
                    }
                } else {
                    hit_info = recursive_traversal(ray, a, scene, non_bvh_dist);
                    if hit_info.tmin < std::f64::MAX {
                        return hit_info;
                    }
                }
            } else if let Some(ref b) = node.b() {
                hit_info = recursive_traversal(ray, b, scene, non_bvh_dist);
                if hit_info.tmin < std::f64::MAX {
                    return hit_info;
                }
            }
        }
    }
    HitInfo::new()
}

pub fn traverse_bvh<'a>(ray: &Ray, node: Option<&Node>, scene: &'a Scene) -> Option<Hit<'a>> {
    let node = match node {
        Some(node) => node,
        None => return None,
    };
    

    let closest_non_bvh_hit = get_closest_non_bvh_hit(scene, ray, &scene.non_bvh_elements(), &scene.non_bvh_composed_elements());
    let non_bvh_dist = if let Some(ref hit) = closest_non_bvh_hit {
        *hit.dist()
    } else {
        std::f64::MAX
    };

    let closest_hit = recursive_traversal(ray, node, scene, std::f64::MAX);
    let bvh_dist = closest_hit.tmin;

    // If there is a hit
    if bvh_dist < non_bvh_dist {
        let element = scene.get_element(closest_hit.element_index);
        let mut hit = Hit::new(element, closest_hit.tmin, ray.get_pos() + ray.get_dir() * closest_hit.tmin, &ray.get_dir(), scene.textures());
        hit.map_textures(scene.textures());
        // println!("bvh hit: {:?}", hit.element().shape());
        return Some(hit);
    } else if non_bvh_dist < bvh_dist {
        let mut hit = closest_non_bvh_hit.unwrap();
        hit.map_textures(scene.textures());
        // println!("non bvh hit: {:?}", hit.element().shape());
        return Some(hit);
    }
    None
}

fn get_closest_non_bvh_hit<'a>(scene: &'a Scene, ray: &Ray, elements_index: &Vec<usize>, composed_elements_index: &Vec<usize>) -> Option<Hit<'a>> {
    let mut closest: Option<Hit> = None;

    for element_index in elements_index {
        let element = scene.get_element(*element_index);
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
    for element_index in composed_elements_index {
        let element = scene.get_element(*element_index);
        let mut t = None;

        t = element.shape().intersect(ray);
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
    match closest {
        None => None,
        Some(hit) => {
            // hit.map_textures(scene.textures());
            Some(hit)
        }
    }
}

#[derive(Debug)]
struct HitInfo {
    element_index: usize,
    tmin: f64,
    tmax: f64,
}

impl HitInfo {
    fn new() -> Self {
        Self {
            element_index: 0,
            tmin: std::f64::MAX,
            tmax: std::f64::MAX,
        }
    }
}