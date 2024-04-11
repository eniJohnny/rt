use crate::{model::{materials::Color, maths::{hit::Hit, ray::Ray}, scene::Scene}, WINDOW_HEIGHT, WINDOW_WIDTH};

pub mod lighting;

pub fn render_scene(scene: &Scene) {
    let camera = scene.camera();
    let rays = camera.get_rays();
    let mut image: Vec<Vec<Color>> = vec![];

    for x in [0, WINDOW_WIDTH] {
        let mut line: Vec<Color> = vec![];
        for y in [0, WINDOW_HEIGHT] {
            line.push(cast_ray(scene, &rays[x][y]))
        }
        image.push(line)
    }
}

pub fn cast_ray(scene: &Scene, ray: &Ray) -> Color {
    match get_closest_hit(scene, ray) {
        Some(hit) => unimplemented!(),
        None => Color {r: 0, g: 0, b: 0}
    }
}

pub fn get_closest_hit<'a>(scene: &'a Scene, ray: &Ray) -> Option<Hit<'a>> {
    let mut closest: Option<Hit> = None;
    for element in scene.elements().iter() {
        if let Some(hit) = element.shape().intersect(ray) {
            if let Some(closest_hit) = &closest{
                if hit.dist() < closest_hit.dist() {
                    closest = Some(hit);
                }
            }
        }
    }
    closest
}