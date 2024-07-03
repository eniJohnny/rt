use std::{
    sync::{Arc, RwLock},
    time::Instant,
};

use image::RgbaImage;
use pixels::Pixels;

use crate::{
    model::scene::Scene,
    ui::{
        ui::UI,
        uibox::UIBox,
        utils::{is_inside_box, UIContext},
    },
};

pub fn blend_scene_and_ui(context: &UIContext, active_box: Option<&UIBox>) -> RgbaImage {
    let mut active_hitbox: Option<((u32, u32), (u32, u32))> = None;
    if let Some(active_box) = active_box {
        active_hitbox = Some((active_box.absolute_pos, active_box.size));
    }
    let mut image = context.ui_img.clone();
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if pixel.0 == [1; 4] {
            pixel.0 = context.scene_img.get_pixel(x, y).0
        }
        if let Some(active_hitbox) = active_hitbox {
            if !is_inside_box((x, y), active_hitbox.0, active_hitbox.1) {
                pixel.0[0] >>= 2;
                pixel.0[1] >>= 2;
                pixel.0[2] >>= 2;
            }
        }
    }
    return image;
}

pub fn redraw_if_necessary(ui: &mut UI, scene: &Arc<RwLock<Scene>>, mut pixels: &mut Pixels) {
    if ui.dirty()
        || ui.context().unwrap().last_ui_draw.elapsed().as_millis()
            > ui.uisettings().ui_refresh_time as u128
    {
        ui.generate_hitboxes(&scene);
    }
    let mut context = ui.take_context();
    let ui_img = &mut context.ui_img;
    let mut redraw = false;
    if ui.dirty()
        || context.last_ui_draw.elapsed().as_millis() > ui.uisettings().ui_refresh_time as u128
    {
        ui.draw(&scene, ui_img);
        redraw = true;
        context.last_ui_draw = Instant::now();
    }
    if let Ok((render_img, final_img)) = context.receiver.try_recv() {
        context.scene_img = render_img;
        context.final_img = final_img;
        context.image_asked = false;
        redraw = true;
    }
    if redraw {
        let time = Instant::now();
        let mut img = blend_scene_and_ui(&context, ui.active_box());
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
