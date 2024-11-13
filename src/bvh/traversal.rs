use super::node::Node;
use crate::{
    model::{
        maths::{hit::Hit, ray::Ray},
        scene::Scene,
        shapes::Shape
    },
    render::raycasting::get_closest_hit_from_elements_with_index
};

pub fn recursive_traversal<'a>(ray: &Ray, node: &Node, scene: &'a Scene, closest: Option<Hit<'a>>) -> Option<Hit<'a>> {
    if let Some(t_aabb) = node.aabb().intersect(ray) {
        if closest.is_some() {
            let mut go_on = false;
            for t in t_aabb {
                if t > 0. && &t < closest.clone().unwrap().dist() {
                    go_on = true;
                }
            }
            if !go_on {
                return closest;
            }
        }
        if node.is_leaf() {
            if let Some(hit) = get_closest_hit_from_elements_with_index(scene, ray, None, scene.elements(), node.elements()) {
                if closest.is_none() || hit.dist() < closest.clone().unwrap().dist() {
                    return Some(hit);
                }
            }
        } else {
            match (node.a(), node.b()) {
                (None, None) => (),
                (Some(a), None) => {
                    if let Some(hit_info) = recursive_traversal(ray, a, scene, closest.clone()) {
                        return Some(hit_info);
                    }
                },
                (None, Some(b)) => {
                    if let Some(hit_info) = recursive_traversal(ray, b, scene, closest.clone()) {
                        return Some(hit_info);
                    }
                },
                (Some(a), Some(b)) => {
                    let dist_a = a.aabb().distance(ray.get_pos());
                    let dist_b = b.aabb().distance(ray.get_pos());
                    if dist_a < dist_b {
                        if let Some(hit_info) = recursive_traversal(ray, a, scene, closest.clone()) {
                            return Some(hit_info);
                        }
                        if let Some(hit_info) = recursive_traversal(ray, b, scene, closest.clone()) {
                            return Some(hit_info);
                        }
                    } else {
                        if let Some(hit_info) = recursive_traversal(ray, b, scene, closest.clone()) {
                            return Some(hit_info);
                        }
                        if let Some(hit_info) = recursive_traversal(ray, a, scene, closest.clone()) {
                            return Some(hit_info);
                        }
                    }
                }
            }
        }
    }
    closest
}