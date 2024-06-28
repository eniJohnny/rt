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
    let mut node = scene.bvh().as_ref().unwrap();

    loop {
        let node_hit = ray_intersects_aabb(ray, node.aabb());
        let left_hit = if let Some(ref left) = node.left() {
            ray_intersects_aabb(ray, &left.aabb())
        } else {
            false
        };
        let right_hit = if let Some(ref right) = node.right() {
            ray_intersects_aabb(ray, &right.aabb())
        } else {
            false
        };

        if node_hit && !left_hit && !right_hit {
            break;
        } else if left_hit {
            node = &node.left().as_ref().unwrap();
        } else if right_hit {
            node = &node.right().as_ref().unwrap();
        } else {
            return None;
        }
    }
    
    return Some(&node);
}

pub fn traverse_bvh<'a>(ray: &Ray, node: Option<&Node>, scene: &'a Scene) -> Option<Hit<'a>> {
    let node = if let Some(node) = node {
        node
    } else {
        return None;
    };

    if !ray_intersects_aabb(ray, &node.aabb()) && !node.aabb().contains_point(ray.get_pos()) {
        return None;
    }

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
            let hit = Hit::new(element, closest_hit.tmin, ray.get_pos() + ray.get_dir() * closest_hit.tmin, &ray.get_dir(), scene.textures());
            return Some(hit);
        }
    }

    let hit_left = if let Some(ref left) = node.left() {
        traverse_bvh(ray, Some(left), &scene)
    } else {
        None
    };

    let hit_right = if let Some(ref right) = node.right() {
        traverse_bvh(ray, Some(right), &scene)
    } else {
        None
    };

    return match (hit_left, hit_right) {
        (Some(left), Some(right)) => {
            if left.dist() < right.dist() {
                Some(left)
            } else {
                Some(right)
            }
        }
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),
        (None, None) => None,
    };
}

struct HitInfo {
    element_index: usize,
    tmin: f64,
    tmax: f64,
}