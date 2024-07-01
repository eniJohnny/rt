use std::{sync::{Arc, RwLock}, time::Instant};

use image::RgbaImage;
use pixels::Pixels;

use crate::{ui::{ui::UI, utils::UIContext}, model::scene::Scene};



pub fn blend_scene_and_ui(context: &UIContext) -> RgbaImage {
    let mut image = context.ui_img.clone();
    for i in image.enumerate_pixels_mut() {
        if i.2 .0 == [1; 4] {
            i.2 .0 = context.scene_img.get_pixel(i.0, i.1).0
        }
    }
    return image;
}

pub fn redraw_if_necessary(ui: &mut UI, scene: &Arc<RwLock<Scene>>, mut pixels: &mut Pixels) {
    if ui.dirty() {
        ui.process(&scene);
    }
    let mut context = ui.take_context();
    let ui_img = &mut context.ui_img;
    let mut redraw = false;
    if ui.dirty() {
        ui.draw(&scene, ui_img);
        redraw = true;
    }
    if let Ok((render_img, final_img)) = context.receiver.try_recv() {
        context.scene_img = render_img;
        context.final_img = final_img;
        context.image_asked = false;
        redraw = true;
    }
    if redraw {
        let time = Instant::now();
        let mut img = blend_scene_and_ui(&context);
        display(&mut pixels, &mut img);
        let nb_samples = context.draw_time_samples as f64;
        context.draw_time_avg = nb_samples * context.draw_time_avg / (nb_samples + 1.)
            + time.elapsed().as_millis() as f64 / (nb_samples + 1.);
        context.draw_time_samples += 1;
        
    }
    ui.give_back_context(context);
}


pub fn display(pixels: &mut Pixels, img: &mut RgbaImage) {
    pixels.frame_mut().copy_from_slice(&img);

    // Render the pixels buffer
    pixels.render().unwrap();
}