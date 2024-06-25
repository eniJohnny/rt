use image::{Pixel, Rgba, RgbaImage};

use crate::{model::maths::vec2::Vec2, GUI_HEIGHT, GUI_WIDTH, SCREEN_WIDTH, SCREEN_WIDTH_U32};

use super::Gui;

pub fn get_line_position(i: u32, size: &Vec2) -> Vec2 {
    let x = SCREEN_WIDTH as f64 - size.x();
    let y = i as f64 * 26.;

    Vec2::new(x, y)
}

pub fn hide_gui(img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>, full_img: &RgbaImage) {
    let width = GUI_WIDTH;
    let x_start = img.width() - width;
    let height = GUI_HEIGHT;

    for x in x_start..img.width() {
        for y in 0..height {
            let pixel = full_img.get_pixel(x, y).to_rgba();
            img.put_pixel(x, y, pixel);
        }
    }
}

pub fn gui_clicked(pos: (f64, f64), gui: &Gui) -> bool {
    let x = pos.0 as u32;
    let y = pos.1 as u32;

    if x >= SCREEN_WIDTH_U32 - GUI_WIDTH && x <= SCREEN_WIDTH_U32 {
        if y <= GUI_HEIGHT {
            return true;
        }
    }

    false
}

pub fn hitbox_contains(hitbox: &(Vec2, Vec2), pos: (f64, f64)) -> bool {
    let x = pos.0 as u32;
    let y = pos.1 as u32;

    if x >= *hitbox.0.x() as u32 && x <= *hitbox.1.x() as u32 {
        if y >= *hitbox.0.y() as u32 && y <= *hitbox.1.y() as u32 {
            return true;
        }
    }

    false
}
