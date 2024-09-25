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
            let a = root.a().as_ref()?;
            let b = root.b().as_ref()?;

            let a_hit = a.aabb().intersect(ray);
            let b_hit = b.aabb().intersect(ray);

            if a_hit.is_some() && b_hit.is_some() {
                if a_hit.unwrap() < b_hit.unwrap() {
                    root = a;
                } else {
                    root = b;
                }
            } else if a_hit.is_some() {
                root = a;
            } else if b_hit.is_some() {
                root = b;
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

    let a_t = if let Some(ref a) = node.a() {
        a.aabb().intersect(ray)
    } else {
        None
    };

    let b_t = if let Some(ref b) = node.b() {
        b.aabb().intersect(ray)
    } else {
        None
    };

    // If ray intersects both a and b children
    if a_t.is_some() && b_t.is_some() {
        // If a child is closer
        if a_t.unwrap() < b_t.unwrap() {
            let hit = traverse_bvh(ray, node.a().as_deref(), scene);
            // If leaf node hit on a child
            if hit.is_some() {
                return hit;
            } else {
                let hit = traverse_bvh(ray, node.b().as_deref(), scene);
                // If leaf node hit on b child
                if hit.is_some() {
                    return hit;
                }
            }
        // If b child is closer
        } else {
            let hit = traverse_bvh(ray, node.b().as_deref(), scene);
            // If leaf node hit on b child
            if hit.is_some() {
                return hit;
            } else {
                let hit = traverse_bvh(ray, node.a().as_deref(), scene);
                // If leaf node hit on a child
                if hit.is_some() {
                    return hit;
                }
            }
        }
    // If ray intersects only the a children
    } else if a_t.is_some() {
        let hit = traverse_bvh(ray, node.a().as_deref(), scene);
        if hit.is_some() {
            return hit;
        }
    // If ray intersects only the b children
    } else if b_t.is_some() {
        let hit = traverse_bvh(ray, node.b().as_deref(), scene);
        if hit.is_some() {
            return hit;
        }
    }

    return None;



    // let hit_a = if let Some(ref a) = node.a() {
    //     traverse_bvh(ray, Some(a), &scene)
    // } else {
    //     None
    // };

    // let hit_b = if let Some(ref b) = node.b() {
    //     traverse_bvh(ray, Some(b), &scene)
    // } else {
    //     None
    // };

    // return match (hit_a, hit_b) {
    //     (Some(a), Some(b)) => {
    //         if a.dist() < b.dist() {
    //             Some(a)
    //         } else {
    //             Some(b)
    //         }
    //     }
    //     (Some(a), None) => Some(a),
    //     (None, Some(b)) => Some(b),
    //     (None, None) => None,
    // };
}

pub fn new_traverse_bvh<'a>(ray: &Ray, node: Option<&Node>, scene: &'a Scene) -> Option<Hit<'a>> {
    let mut node = match node {
        Some(node) => node,
        None => return None,
    };

    let closest_non_bvh_hit = get_closest_non_bvh_hit(scene, ray, &scene.non_bvh_elements(), &scene.non_bvh_composed_elements());
    let tn = if let Some(ref hit) = closest_non_bvh_hit {
        Some(vec![*hit.dist()])
    } else {
        None
    };

    while node.is_leaf() == false {
        // println!("is leaf: {}\nlen: {}\nelements: {:?}\na: {}\nb: {}\n", node.is_leaf(), node.len(), node.elements(), node.a().as_ref().unwrap(), node.b().as_ref().unwrap());

        let (mut ta, mut tb): (Option<Vec<f64>>, Option<Vec<f64>>) = (None, None);

        // Check if ray intersects a child
        if let Some(ref a) = node.a() {
            ta = a.aabb().intersect(ray);
        }
        
        // Check if ray intersects b child
        if let Some(ref b) = node.b() {
            tb = b.aabb().intersect(ray);
        }

        // Check if ray intersects non_bvh element first
        if let Some(ref n) = tn {
            // For a children
            if let Some(ref a) = ta {
                if n[0] < a[0] || n[0] - a[0] < f64::EPSILON {
                    ta = None;
                }
            }
            
            // For b children
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
            println!("tn.is_some() && ta.is_none() && tb.is_none()");
            let mut hit = closest_non_bvh_hit.unwrap();
            hit.map_textures(scene.textures());
            return Some(hit);
        }

        // If there is a hit on a child
        if let Some(a) = ta {
            // If there is a hit on b child too
            if let Some(b) = tb {
                // If a child is closer
                if a[0] < b[0] {
                    node = node.a().as_ref()?;
                    continue;
                // If b child is closer
                } else {
                    node = node.b().as_ref()?;
                    continue;
                }
            // If there is no hit on b child
            } else {
                node = node.a().as_ref()?;
                continue;
            }
        }

        // If there is a hit on b child only
        if tb.is_some() {
            node = node.b().as_ref()?;
            continue;
        }

        // If there is no hit on both children
        // println!("wtf");
    }

    // Node is now a leaf node and closest hit is part of bvh
    let mut closest_hit = HitInfo::new();

    // Check for closest bvh hit
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


    let bvh_dist = closest_hit.tmin;
    let non_bvh_dist = match &closest_non_bvh_hit {
        Some(hit) => *hit.dist(),
        None => std::f64::MAX,
    };

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

fn get_closest_non_bvh_hit<'a>(scene: &'a Scene, ray: &Ray, elements: &Vec<&'a Element>, composed_elements: &Vec<&'a Element>) -> Option<Hit<'a>> {
    let mut closest: Option<Hit> = None;

    for element in elements {
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
    for element in composed_elements {
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