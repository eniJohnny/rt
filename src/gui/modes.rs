use std::sync::RwLockWriteGuard;

use image::{ImageBuffer, Rgba, RgbaImage};

use crate::{model::{maths::vec2::Vec2, scene::Scene}, MODES_KEYS, MODES_LABELS};

use super::textformat::TextFormat;


pub fn draw_modes_menu(scene: RwLockWriteGuard<Scene>, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for i in 0..MODES_KEYS.len() {
        let key = MODES_KEYS[i];
        let label = MODES_LABELS[i];
        let toggled = scene.get_value(key);

        println!("{}: {}", label, toggled);

        draw_mode(label, toggled, i, image);
    }
}

fn draw_mode(label: &str, toggled: bool, i: usize, image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let line_height = 20.0;

    let pos = &Vec2::new(10.0, 10.0 + (i as f64 * (line_height + 5.)));
    let slider_pos = &Vec2::new(70., *pos.y());

    let format = TextFormat::new_base_format();

    draw_text(image, pos, label.to_string(), &format);
    draw_slider(image, slider_pos, toggled);
}

fn draw_text(image: &mut RgbaImage, pos: &Vec2, text: String, format: &TextFormat) {
    
}

fn draw_slider(image: &mut RgbaImage, pos: &Vec2, toggled: bool) {
    let width = 30;
    let height = 20;

    // Draw a slider toggle, which is an oval about thrice as large as it is high, the oval is black and there's a white circle aligned to the left when OFF, the oval is blue and the circle is to the right when ON
    let mut color = Rgba([0, 0, 0, 255]);
    let mut circle_pos = Vec2::new(pos.x() + 5., pos.y() + 5.);
    if toggled {
        color = Rgba([0, 0, 255, 255]);
        circle_pos = Vec2::new(pos.x() + 15., pos.y() + 5.);
    }

    for x in 0..width {
        for y in 0..height {
            let x_f = x as f64;
            let y_f = y as f64;
            let x_centered = x_f - (width as f64) / 2.;
            let y_centered = y_f - (height as f64) / 2.;
            let x_normalized = x_centered / (width as f64 / 2.);
            let y_normalized = y_centered / (height as f64 / 2.);
            let distance = x_normalized * x_normalized + y_normalized * y_normalized;
            if distance <= 1. {
                image.put_pixel((pos.x() + x_f) as u32, (pos.y() + y_f) as u32, color);
            }
        }
    }

    for x in 0..5 {
        for y in 0..5 {
            let x_f = x as f64;
            let y_f = y as f64;
            let x_centered = x_f - 2.;
            let y_centered = y_f - 2.;
            let x_normalized = x_centered / 2.;
            let y_normalized = y_centered / 2.;
            let distance = x_normalized * x_normalized + y_normalized * y_normalized;
            if distance <= 1. {
                image.put_pixel((circle_pos.x() + x_f) as u32, (circle_pos.y() + y_f) as u32, Rgba([255, 255, 255, 255]));
            }
        }
    }

    
    
}