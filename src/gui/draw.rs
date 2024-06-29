use std::sync::{Arc, RwLock};

use image::{Rgba, RgbaImage};

use crate::{
    model::{
        materials::{color::Color, material::Material, texture::Texture},
        maths::vec2::Vec2,
        objects::light::{Light, PointLight},
        scene::Scene,
        shapes::{cone, cylinder, plane, sphere},
        Element,
    },
    GUI_HEIGHT, GUI_WIDTH, SCREEN_HEIGHT_U32, SCREEN_WIDTH, SCREEN_WIDTH_U32,
};

use super::{elements::uibox::UIBox, textformat::Style, Gui};

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
        (x_start + radius, y_start + radius),     // Top-left
        (x_end - radius - 1, y_start + radius),   // Top-right
        (x_start + radius, y_end - radius - 1),   // Bottom-left
        (x_end - radius - 1, y_end - radius - 1), // Bottom-right
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
        return false; // Top and bottom edges
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

pub fn draw_button_background(
    img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    hitbox: &(Vec2, Vec2),
    color: Rgba<u8>,
) {
    let upper_left_corner = &hitbox.0;
    let lower_right_corner = &hitbox.1;

    let x_start = *upper_left_corner.x() as u32;
    let x_end = *lower_right_corner.x() as u32;
    let y_start = *upper_left_corner.y() as u32;
    let y_end = *lower_right_corner.y() as u32;

    for x in x_start..x_end {
        for y in y_start..y_end {
            if is_corner(x, y, x_start, y_start, x_end, y_end, 2) == false {
                img.put_pixel(x, y, color);
            }
        }
    }
}
