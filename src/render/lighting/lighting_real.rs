use rand::Rng;

use crate::{
    bvh::{self, traversal::{get_closest_aabb_hit, traverse_bvh}}, model::{
        materials::color::Color,
        maths::{hit::Hit, ray::Ray, vec3::Vec3},
        objects::light,
        scene::Scene,
    },
    render::{raycasting::get_closest_hit, skysphere::get_skysphere_color},
    MAX_DEPTH,
};

use super::{
    lighting_sampling::{
        get_indirect_light_bucket, get_indirect_light_sample, get_reflected_light_sample,
        random_unit_vector, reflect_dir,
    },
    raycasting::{get_closest_hit, get_closest_plane_hit, get_ray},
    restir::{PathBucket, Sample}, skysphere::get_skysphere_color,
};

pub fn get_lighting_from_ray(scene: &Scene, ray: &Ray) -> Color {
    let node = scene.bvh().as_ref().unwrap(); // ~19 nanoseconds
    // let debug = false;

    // let perf_timer = std::time::Instant::now();
    // let bvh_hit = traverse_bvh(ray, Some(node), scene);
    // let bvh_time = perf_timer.elapsed().as_nanos();
    

    // let perf_timer = std::time::Instant::now();
    // let old_hit = get_closest_hit(scene, ray);
    // let old_time = perf_timer.elapsed().as_nanos();

    // if debug {

    //     let mut is_bvh = false;
    //     let mut bvhstr = "None".to_string();
    //     let mut oldstr = "None".to_string();
    
    //     if bvh_hit.is_some() {
    //         bvhstr = format!("{:?}", bvh_hit.as_ref().unwrap());
    //     }

    //     if old_hit.is_some() {
    //         oldstr = format!("{:?}", old_hit.as_ref().unwrap());
    //         if old_hit.unwrap().element().shape().as_plane().is_none() {
    //             is_bvh = true;
    //         }
    //     }

    //     if is_bvh  {

    //         println!("\n---------------------");
    //         println!("BVH: {}", bvhstr);
    //         println!("OLD: {}", oldstr);
    //         println!("BVH == OLD: {}", bvhstr == oldstr);
    //         println!("BVH Time: {}ns", bvh_time);
    //         println!("OLD Time: {}ns", old_time);
    //     }
    // }
    
    // let match_hit = match bvh_hit {
    //     Some(hit) => {
    //         // println!("BVH");
    //         Some(hit)
    //     },
    //     None => {
    //         // println!("NOT BVH");
    //         get_closest_non_bvh_hit(scene, ray)
    //     },
    // };

    // let perf_timer = std::time::Instant::now();
    
    let plane_hit = get_closest_plane_hit(scene.textures(), ray, scene.planes());

    // let plane_time = perf_timer.elapsed().as_nanos();
    // let perf_timer = std::time::Instant::now();

    let bvh_hit = traverse_bvh(ray, Some(node), scene);

    // let bvh_time = perf_timer.elapsed().as_nanos();
    // println!("Plane: {}ns, BVH: {}ns, Plane takes more time than BVH: {}", plane_time, bvh_time, plane_time > bvh_time);


    let match_hit = match (plane_hit, bvh_hit) {
        (Some(plane), Some(bvh)) => {
            if plane.dist() < bvh.dist() {
                Some(plane)
            } else {
                Some(bvh)
            }
        },
        (Some(plane), None) => {
            Some(plane)
        },
        (None, Some(bvh)) => {
            Some(bvh)
        },
        (None, None) => {
            None
        },
    };
    
    // match get_closest_hit(scene, ray) {
    // match traverse_bvh(ray, Some(node), scene) {
    match match_hit {
        Some(hit) => {
            // println!("Time elapsed: {}ns", perf_timer.elapsed().as_nanos());
            get_lighting_from_hit(scene, &hit, ray)
        },
        //TODO : Handle BG on None
        // None => Color::new(0., 0., 0.),
        None => {
            get_skysphere_color(scene, ray)
        }
    }
}

pub fn fresnel_reflect_ratio(n1: f64, n2: f64, norm: &Vec3, ray: &Vec3, f0: f64, f90: f64) -> f64 {
    // Schlick aproximation
    let mut r0 = (n1 - n2) / (n1 + n2);
    r0 = r0 * r0;
    let mut cosX = -(norm.dot(&ray));
    if n1 > n2 {
        let n = n1 / n2;
        let sinT2 = n * n * (1.0 - cosX * cosX);
        // Total internal reflection
        if sinT2 > 1.0 {
            return f90;
        }
        cosX = (1.0 - sinT2).sqrt();
    }
    let x = 1.0 - cosX;
    let ret = r0 + (1.0 - r0) * x.powf(5.);

    // adjust reflect multiplier for object reflectivity
    f0 * (1.0 - ret) + f90 * ret
}

pub fn get_lighting_from_hit(scene: &Scene, hit: &Hit, ray: &Ray) -> Color {
    let absorbed = 1.0 - hit.metalness() - hit.refraction();
    if ray.debug {
        println!(
            "Metal : {}, Roughness: {}, Color: {}, Norm: {}, Emissive: {}, Opacity: {}, Refraction: {}",
            hit.metalness(),
            hit.roughness(),
            hit.color(),
            hit.norm(),
            hit.emissive(),
            hit.opacity(),
            hit.refraction()
        );
    }

    if hit.emissive() > f64::EPSILON {
        return hit.emissive() * hit.color();
    }
    let mut light_color = Color::new(0., 0., 0.);
    let fresnel_factor =
        fresnel_reflect_ratio(1., 1., hit.norm(), ray.get_dir(), 0., 1.0 - hit.roughness());
    let reflected = fresnel_factor * absorbed;

    let rand = rand::thread_rng().gen_range(0.0..1.0);
    if rand > reflected + hit.metalness() {
        // Indirect Light
        if scene.settings().indirect && ray.get_depth() < MAX_DEPTH as u8 {
            let mut indirect_dir = hit.norm() + random_unit_vector();
            if indirect_dir.length() < 0.01 {
                indirect_dir = hit.norm().clone();
            }
            indirect_dir = indirect_dir.normalize();
            let indirect_ray = Ray::new(hit.pos().clone(), indirect_dir, ray.get_depth() + 1);
            light_color = get_lighting_from_ray(scene, &indirect_ray) * hit.color();
        }
    } else if scene.settings().reflections && ray.get_depth() < scene.settings().depth as u8 {
        let reflect_color;
        let dir = (reflect_dir(ray.get_dir(), hit.norm()) + random_unit_vector() * hit.roughness())
            .normalize();
        if dir.dot(hit.norm()) > f64::EPSILON {
            let reflect_ray = Ray::new(hit.pos().clone(), dir, ray.get_depth() + 1);
            reflect_color = get_lighting_from_ray(scene, &reflect_ray);
        } else {
            reflect_color = Color::new(0., 0., 0.);
        }
        if rand > hit.metalness() {
            light_color += reflect_color
        } else {
            light_color += reflect_color * hit.color();
        }
    }
    light_color
}
