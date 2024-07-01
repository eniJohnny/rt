use crate::model::{maths::{hit::Hit, ray::Ray}, scene::{self, Scene}, shapes::{aabb::Aabb, Shape}};

use super::node::Node;

pub fn ray_intersects_aabb(ray: &Ray, aabb: &Aabb) -> bool {
    let mut tmin = (aabb.x_min() - ray.get_pos().x()) / ray.get_dir().x();
    let mut tmax = (aabb.x_max() - ray.get_pos().x()) / ray.get_dir().x();

    if tmin > tmax {
        std::mem::swap(&mut tmin, &mut tmax);
    }

    let mut tymin = (aabb.y_min() - ray.get_pos().y()) / ray.get_dir().y();
    let mut tymax = (aabb.y_max() - ray.get_pos().y()) / ray.get_dir().y();

    if tymin > tymax {
        std::mem::swap(&mut tymin, &mut tymax);
    }

    if (tmin > tymax) || (tymin > tmax) {
        return false;
    }

    if tymin > tmin {
        tmin = tymin;
    }

    if tymax < tmax {
        tmax = tymax;
    }

    let mut tzmin = (aabb.z_min() - ray.get_pos().z()) / ray.get_dir().z();
    let mut tzmax = (aabb.z_max() - ray.get_pos().z()) / ray.get_dir().z();

    if tzmin > tzmax {
        std::mem::swap(&mut tzmin, &mut tzmax);
    }

    if (tmin > tzmax) || (tzmin > tmax) {
        return false;
    }

    if tzmin > tmin {
        tmin = tzmin;
    }

    if tzmax < tmax {
        tmax = tzmax;
    }

    tmin < tmax
}

pub fn get_closest_aabb_hit<'a>(scene: &'a  Scene, ray: &'a Ray) -> Option<&'a Node> {
    // Get the first aabb hit

    let mut root = scene.bvh().as_ref()?;
    
    loop {
        if root.aabb().intersect(ray).is_some() || root.aabb().contains_point(ray.get_pos()) {
            let left = root.left().as_ref()?;
            let right = root.right().as_ref()?;

            let left_hit = left.aabb().intersect(ray);
            let right_hit = right.aabb().intersect(ray);

            if left_hit.is_some() && right_hit.is_some() {
                if left_hit.unwrap() < right_hit.unwrap() {
                    root = left;
                } else {
                    root = right;
                }
            } else if left_hit.is_some() {
                root = left;
            } else if right_hit.is_some() {
                root = right;
            } else {
                break;
            }
        } else {
            return None;
        }
    }
    return Some(root);
}

pub fn traverse_bvh<'a>(ray: &Ray, node: Option<&Node>, scene: &'a Scene) -> Option<Hit<'a>> {
    let node = match node {
        Some(node) => node,
        None => return None,
    };

    // if !ray_intersects_aabb(ray, &node.aabb()) && !node.aabb().contains_point(ray.get_pos()) {
    //     return None;
    // }
    
    if node.is_leaf() {
        let mut closest_hit: HitInfo = HitInfo {
            element_index: 0,
            tmin: std::f64::MAX,
            tmax: std::f64::MAX,
        };

        for &element_index in node.elements() {
            let element = scene.get_element(element_index);
            let hit = element.shape().intersect(ray);
            if let Some(hit) = hit {
                let tmin = hit[0];
                let tmax = hit[1];

                if (tmin == closest_hit.tmin && tmax < closest_hit.tmax) || tmin < closest_hit.tmin {
                    closest_hit.element_index = element_index;
                    closest_hit.tmin = tmin;
                    closest_hit.tmax = tmax;
                }
            }
        }
        
        if closest_hit.tmin < std::f64::MAX {
            let element = scene.get_element(closest_hit.element_index);
            let mut hit = Hit::new(element, closest_hit.tmin, ray.get_pos() + ray.get_dir() * closest_hit.tmin, &ray.get_dir(), scene.textures());
            hit.map_textures(scene.textures());
            return Some(hit);
        }
    }

    let left_t = if let Some(ref left) = node.left() {
        left.aabb().intersect(ray)
    } else {
        None
    };

    let right_t = if let Some(ref right) = node.right() {
        right.aabb().intersect(ray)
    } else {
        None
    };

    // If ray intersects both left and right children
    if left_t.is_some() && right_t.is_some() {
        // If left child is closer
        if left_t.unwrap() < right_t.unwrap() {
            let hit = traverse_bvh(ray, node.left().as_deref(), scene);
            // If leaf node hit on left child
            if hit.is_some() {
                return hit;
            } else {
                let hit = traverse_bvh(ray, node.right().as_deref(), scene);
                // If leaf node hit on right child
                if hit.is_some() {
                    return hit;
                }
            }
        // If right child is closer
        } else {
            let hit = traverse_bvh(ray, node.right().as_deref(), scene);
            // If leaf node hit on right child
            if hit.is_some() {
                return hit;
            } else {
                let hit = traverse_bvh(ray, node.left().as_deref(), scene);
                // If leaf node hit on left child
                if hit.is_some() {
                    return hit;
                }
            }
        }
    // If ray intersects only the left children
    } else if left_t.is_some() {
        let hit = traverse_bvh(ray, node.left().as_deref(), scene);
        if hit.is_some() {
            return hit;
        }
    // If ray intersects only the right children
    } else if right_t.is_some() {
        let hit = traverse_bvh(ray, node.right().as_deref(), scene);
        if hit.is_some() {
            return hit;
        }
    }

    return None;



    // let hit_left = if let Some(ref left) = node.left() {
    //     traverse_bvh(ray, Some(left), &scene)
    // } else {
    //     None
    // };

    // let hit_right = if let Some(ref right) = node.right() {
    //     traverse_bvh(ray, Some(right), &scene)
    // } else {
    //     None
    // };

    // return match (hit_left, hit_right) {
    //     (Some(left), Some(right)) => {
    //         if left.dist() < right.dist() {
    //             Some(left)
    //         } else {
    //             Some(right)
    //         }
    //     }
    //     (Some(left), None) => Some(left),
    //     (None, Some(right)) => Some(right),
    //     (None, None) => None,
    // };
}

struct HitInfo {
    element_index: usize,
    tmin: f64,
    tmax: f64,
}