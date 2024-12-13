use super::node::Node;
use crate::{
    model::{
        maths::{hit::Hit, ray::Ray},
        scene::Scene,
        shapes::Shape
    },
    render::raycasting::get_closest_hit_from_elements_with_index
};

pub fn recursive_traversal<'a>(ray: &Ray, node: &Node, scene: &'a Scene, mut closest: Option<Hit<'a>>, t_aabb: Vec<f64>, depth: usize) -> Option<Hit<'a>> {
    let mut return_closest = false;
    if t_aabb.len() == 2 {
        let mut is_closest = false;
        let mut will_traverse = false;
        for t in &t_aabb {
            if t > &0. {
                will_traverse = true;
            }
            if let Some(closest) = &closest {
                if t < closest.dist() {
                    is_closest = true;
                }
            } else {
                is_closest = true;
            }
        }
        // If any of the bounding volume is not before the already found hit, we just skip it
        if !is_closest || !will_traverse {
            return None;
        }
        if node.elements().len() > 0 {
            // We check every element that is a direct child of the AABB for intersections 
            if let Some(hit) = get_closest_hit_from_elements_with_index(scene, ray, None, scene.elements(), node.elements()) {
                if closest.is_none() || (hit.dist() > &0. && hit.dist() < closest.clone().unwrap().dist()) {
                    if node.is_leaf() {
                        return Some(hit);
                    } else {
                        // If we find any better hit than the one provided in the function parameters, we eventually return it, if we don't find anything better in the child nodes.
                        return_closest = true;
                        closest = Some(hit)
                    }
                }
            }
        }
        match (node.a(), node.b()) {
            (None, None) => (),
            (Some(a), None) => {
                if let Some(hit_info) = recursive_traversal(ray, a, scene, closest.clone(), a.aabb().intersect(ray).unwrap_or(vec![]), depth + 1) {
                    return Some(hit_info);
                }
            },
            (None, Some(b)) => {
                if let Some(hit_info) = recursive_traversal(ray, b, scene, closest.clone(), b.aabb().intersect(ray).unwrap_or(vec![]), depth + 1) {
                    return Some(hit_info);
                }
            },
            (Some(a), Some(b)) => {
                let dist_a = a.aabb().intersect(ray).unwrap_or(vec![f64::MAX]);
                let dist_b = b.aabb().intersect(ray).unwrap_or(vec![f64::MAX]);

                // We first check for intersections in the closest AABB, because if we find a hit there, there's no reason to check for one in the other one.
                if dist_a[0] < dist_b[0] {
                    if let Some(hit_info) = recursive_traversal(ray, a, scene, closest.clone(), dist_a, depth + 1) {
                        if hit_info.dist() < &dist_b[0] {
                            return Some(hit_info);
                        }
                        closest = Some(hit_info);
                        return_closest = true;
                    }
                    if let Some(hit_info) = recursive_traversal(ray, b, scene, closest.clone(), dist_b, depth + 1) {
                        return Some(hit_info);
                    }
                } else {
                    if let Some(hit_info) = recursive_traversal(ray, b, scene, closest.clone(), dist_b, depth + 1) {
                        if hit_info.dist() < &dist_a[0] {
                            return Some(hit_info);
                        }
                        closest = Some(hit_info);
                        return_closest = true;
                    }
                    if let Some(hit_info) = recursive_traversal(ray, a, scene, closest.clone(), dist_a, depth + 1) {
                        return Some(hit_info);
                    }
                }
            }
        }
    }
    if return_closest {
        return closest;
    }
    None
}