use std::{sync::{Arc, RwLock}, time::Instant};

use image::{Rgba, RgbaImage};
use pixels::Pixels;

use crate::{
    display::display, model::{
        materials::{color::Color, material::Material, texture::Texture},
        maths::vec2::Vec2,
        objects::light::{Light, PointLight},
        scene::Scene,
        shapes::{cone, cylinder, plane, sphere},
        Element,
    }, GUI_HEIGHT, GUI_WIDTH, SCREEN_HEIGHT_U32, SCREEN_WIDTH, SCREEN_WIDTH_U32
};

use super::{elements::{ui::{UIContext, UI}, uibox::UIBox}, textformat::Style, Gui};

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

fn is_corner(
    x: u32,
    y: u32,
    x_start: u32,
    y_start: u32,
    x_end: u32,
    y_end: u32,
    radius: u32,
) -> bool {
    let corners = [
        (x_start + radius, y_start + radius),
        (x_end - radius - 1, y_start + radius),
        (x_start + radius, y_end - radius - 1),
        (x_end - radius - 1, y_end - radius - 1),
    ];

    for &(cx, cy) in &corners {
        let dx = cx as isize - x as isize;
        let dy = cy as isize - y as isize;
        if dx * dx + dy * dy <= (radius * radius) as isize {
            return false;
        }
    }

    if (x >= x_start + radius && x < x_end - radius)
        || (y >= y_start + radius && y < y_end - radius)
        || (x < x_start + radius && y >= y_start + radius && y < y_end - radius)
        || (x >= x_end - radius && y >= y_start + radius && y < y_end - radius)
    {
        return false;
    }

    true
}

pub fn draw_checkbox(
    img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    pos: (u32, u32),
    size: (u32, u32),
    value: bool,
    style: &Style,
) {
    let checkbox_size = (18, 18);
    let height = size.1;
    let checkbox_pos = (
        pos.0 + size.0 - style.padding_right - checkbox_size.0,
        pos.1 + (height - checkbox_size.1) / 2,
    );

    draw_background(
        img,
        checkbox_pos,
        checkbox_size,
        Rgba([40, 10, 90, 255]),
        style.border_radius,
    );
    draw_background(
        img,
        (checkbox_pos.0 + 2, checkbox_pos.1 + 2),
        (checkbox_size.0 - 4, checkbox_size.1 - 4),
        Rgba([255, 255, 255, 255]),
        style.border_radius,
    );
    if value {
        draw_background(
            img,
            (checkbox_pos.0 + 4, checkbox_pos.1 + 4),
            (checkbox_size.0 - 8, checkbox_size.1 - 8),
            Rgba([40, 10, 90, 255]),
            style.border_radius,
        );
    }
}

pub fn draw_background(
    img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    pos: (u32, u32),
    size: (u32, u32),
    color: Rgba<u8>,
    border_radius: u32,
) {
    let x_start = pos.0;
    let x_end = (pos.0 + size.0).min(SCREEN_WIDTH_U32 - 1);
    let y_start = pos.1;
    let y_end = (pos.1 + size.1).min(SCREEN_HEIGHT_U32 - 1);

    for x in x_start..x_end {
        for y in y_start..y_end {
            if is_corner(x, y, x_start, y_start, x_end, y_end, border_radius) == false {
                img.put_pixel(x, y, color);
            }
        }
    }
}
