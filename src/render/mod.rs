use image::RgbaImage;

use crate::{model::{materials::Color, maths::{hit::Hit, ray::Ray}, scene::Scene, Element}, SCREEN_HEIGHT, SCREEN_WIDTH};

use self::lighting::apply_lighting;

pub mod lighting;

pub fn render_scene(scene: &Scene) -> RgbaImage {
    let camera = scene.camera();
    let rays = camera.get_rays();
    let mut img = RgbaImage::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);

    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            img.put_pixel(x as u32, y as u32, cast_ray(scene, &rays[x as usize][y as usize]).toRgba());
            println!("{}", rays[0][13]);
        }
    }
    img
}

pub fn cast_ray(scene: &Scene, ray: &Ray) -> Color {
    match get_closest_hit(scene, ray) {
        Some(hit) => apply_lighting(hit, scene),
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
    match closest {
        None => None,
        Some((t, elem)) => Some(Hit::new(closest.unwrap().1, closest.unwrap().0, ray.get_pos() + ray.get_dir() * closest.unwrap().0))
    }
}