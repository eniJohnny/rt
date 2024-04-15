use image::RgbaImage;

use crate::{model::{materials::Color, maths::{hit::Hit, ray::Ray}, scene::Scene, Element}, SCREEN_HEIGHT, SCREEN_WIDTH, GUI_WIDTH};

use self::lighting::apply_lighting;

pub mod lighting;

pub fn render_scene(scene: &Scene) -> RgbaImage {
    let camera = scene.camera();
    let perf_timer = std::time::Instant::now();
    let rays = camera.get_rays();
    println!("Ray generation time: {:?}", perf_timer.elapsed());
    let width = SCREEN_WIDTH - GUI_WIDTH;
    let mut img = RgbaImage::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let perf_timer = std::time::Instant::now();
    for x in 0..width {
        for y in 0..SCREEN_HEIGHT {
            img.put_pixel(x as u32, y as u32, cast_ray(scene, &rays[x as usize][y as usize]).toRgba());
        }
    }
    println!("Image building time: {:?}", perf_timer.elapsed());
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
            if let Some((tmin, _)) = &closest {
                if &t[0] < tmin {
                    closest = Some((t[0], element));
                }
            } else {
                closest = Some((t[0], element))
            }
        }
    }
    match closest {
        None => None,
        Some((t, elem)) => Some(Hit::new(elem,t,ray.get_pos() + ray.get_dir() * t))
    }
}
