use crate::{model::{materials::Color, maths::{hit::Hit, ray::Ray}, scene::Scene}, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::model::Element;

pub mod lighting;

// pub fn render_scene(scene: &Scene) {
//     let camera = scene.camera();
//     let rays = camera.get_rays();
//     let mut image: Vec<Vec<Color>> = vec![];

//     for x in [0, SCREEN_WIDTH] {
//         let mut line: Vec<Color> = vec![];
//         for y in [0, SCREEN_HEIGHT] {
//             line.push(cast_ray(scene, &rays[x][y]))
//         }
//         image.push(line)
//     }
// }

pub fn cast_ray(scene: &Scene, ray: &Ray) -> Color {
    match get_closest_hit(scene, ray) {
        Some(hit) => unimplemented!(),
        None => Color::new(0, 0, 0)
    }
}

pub fn get_closest_hit<'a>(scene: &'a Scene, ray: &Ray) -> Option<Hit<'a>> {
    let mut closest: Option<(f64, &Element)> = None;
    for element in scene.elements().iter() {
        if let Some(t) = element.shape().intersect(ray) {
            if let Some((tmin, elem)) = &closest {
                if &t[0] < tmin {
                    closest = Some((t[0], element));
                }
            }
        }
    }
    if closest.is_none() {
        return None;
    }
    let hit = Hit::new(closest.unwrap().1, closest.unwrap().0, ray.get_pos() + ray.get_dir() * closest.unwrap().0);
    return Some(hit);
}