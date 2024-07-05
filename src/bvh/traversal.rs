use crate::{model::{maths::{hit::{self, Hit}, ray::Ray}, scene::{self, Scene}, shapes::{aabb::Aabb, Shape}, Element}, render::raycasting::get_closest_hit};

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
                let tmax = if hit.len() > 1 {hit[1]} else {tmin};

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

pub fn new_traverse_bvh<'a>(ray: &Ray, node: Option<&Node>, scene: &'a Scene) -> Option<Hit<'a>> {
    let mut node = match node {
        Some(node) => node,
        None => return None,
    };

    let closest_non_bvh_hit = get_closest_non_bvh_hit(scene, ray, &scene.non_bvh_elements());
    let tn = if let Some(ref hit) = closest_non_bvh_hit {
        Some(vec![*hit.dist()])
    } else {
        None
    };

    while node.is_leaf() == false {
        // println!("is leaf: {}\nlen: {}\nelements: {:?}\nleft: {}\nright: {}\n", node.is_leaf(), node.len(), node.elements(), node.left().as_ref().unwrap(), node.right().as_ref().unwrap());

        let (mut ta, mut tb): (Option<Vec<f64>>, Option<Vec<f64>>) = (None, None);

        // Check if ray intersects left child
        if let Some(ref left) = node.left() {
            ta = left.aabb().intersect(ray);
        }
        
        // Check if ray intersects right child
        if let Some(ref right) = node.right() {
            tb = right.aabb().intersect(ray);
        }

        // Check if ray intersects non_bvh element first
        if let Some(ref n) = tn {
            // For left children
            if let Some(ref a) = ta {
                if n[0] < a[0] || n[0] - a[0] < f64::EPSILON {
                    ta = None;
                }
            }
            
            // For right children
            if let Some(ref b) = tb {
                if n[0] < b[0] || n[0] - b[0] < f64::EPSILON {
                    tb = None;
                }
            }
        }

        if tn.is_some() || ta.is_some() || tb.is_some() || closest_non_bvh_hit.is_some() {
            println!("tn: {:?}\nta: {:?}\ntb: {:?}\nclosest hit: {:?}\n", tn, ta, tb, closest_non_bvh_hit);
        }

        if tn.is_some() && ta.is_none() && tb.is_none() {
            let mut hit = closest_non_bvh_hit.unwrap();
            hit.map_textures(scene.textures());
            return Some(hit);
        }

        // If there is a hit on left child
        if let Some(a) = ta {
            // If there is a hit on right child too
            if let Some(b) = tb {
                // If left child is closer
                if a[0] < b[0] {
                    node = node.left().as_ref()?;
                    continue;
                // If right child is closer
                } else {
                    node = node.right().as_ref()?;
                    continue;
                }
            // If there is no hit on right child
            } else {
                node = node.left().as_ref()?;
                continue;
            }
        }

        // If there is a hit on right child only
        if tb.is_some() {
            node = node.right().as_ref()?;
            continue;
        }

        // If there is no hit on both children
        // println!("wtf");
    }

    // Node is now a leaf node and closest hit is part of bvh
    let mut closest_hit = HitInfo::new();

    // Check for closest hit
    for &element_index in node.elements() {
        let element = scene.get_element(element_index);
        let hit = element.shape().intersect(ray);
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

    // If there is a hit
    if closest_hit.tmin < std::f64::MAX {
        let element = scene.get_element(closest_hit.element_index);
        let mut hit = Hit::new(element, closest_hit.tmin, ray.get_pos() + ray.get_dir() * closest_hit.tmin, &ray.get_dir(), scene.textures());
        hit.map_textures(scene.textures());
        return Some(hit);
    }
    None
}

fn get_closest_non_bvh_hit<'a>(scene: &'a Scene, ray: &Ray, elements: &Vec<&'a Element>) -> Option<Hit<'a>> {
    let mut closest: Option<Hit> = None;

    for element in elements {
        let t = element.shape().intersect(ray);
        let mut tmin = std::f64::MAX;

        if let Some(t) = t {
            for tx in t {
                if tx < tmin && tx > 0.0 {
                    tmin = tx;
                }
            }

            if tmin < std::f64::MAX {
                let hit = Hit::new(element, tmin, ray.get_pos() + ray.get_dir() * tmin, &ray.get_dir(), scene.textures());

                if hit.opacity() > 0.5 {
                    closest = Some(hit);
                }
            }
        }
    }

    closest
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