use super::node::Node;
use crate::{
    model::{
        maths::{hit::Hit, ray::Ray},
        scene::Scene,
        shapes::shape::Shape
    },
    render::raycasting::get_closest_hit_from_elements_with_index
};

pub fn recursive_traversal<'a>(ray: &Ray, node: &Node, scene: &'a Scene, mut closest: Option<Hit<'a>>, t_aabb: Vec<f64>, depth: usize) -> Option<Hit<'a>> {
    if t_aabb.len() == 2 {
        if node.elements().len() > 0 {
            // We check every element that is a direct child of the AABB for intersections 
            if let Some(mut hit) = get_closest_hit_from_elements_with_index(scene, ray, None, scene.elements(), node.elements()) {
                if let Some(previous) = &mut closest {
                    // In order to determine in we are inside or outside an object (notably for the refraction indices), we need every intersection along the ray to be passed
                    // So we merge both intersection lists from the old and the new hit
                    if hit.dist() > &0. && (previous.dist() < &0. || hit.dist() < previous.dist()) {
                        hit.t_list_mut().append(previous.t_list_mut());
                        closest = Some(hit)
                    } else {
                        previous.t_list_mut().append(hit.t_list_mut());
                    }
                } else {
                    closest = Some(hit);
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
                        if !scene.settings().bvh_full_traversal && hit_info.dist() < &dist_b[0] && hit_info.dist() > &0. {
                            return Some(hit_info);
                        }
                        closest = Some(hit_info);
                    }
                    if let Some(hit_info) = recursive_traversal(ray, b, scene, closest.clone(), dist_b, depth + 1) {
                        return Some(hit_info);
                    }
                } else {
                    if let Some(hit_info) = recursive_traversal(ray, b, scene, closest.clone(), dist_b, depth + 1) {
                        if !scene.settings().bvh_full_traversal && hit_info.dist() < &dist_a[0] && hit_info.dist() > &0. {
                            return Some(hit_info);
                        }
                        closest = Some(hit_info);
                    }
                    if let Some(hit_info) = recursive_traversal(ray, a, scene, closest.clone(), dist_a, depth + 1) {
                        return Some(hit_info);
                    }
                }
            }
        }
    }
    closest
}