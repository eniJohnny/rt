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
    border_radius: u32,
) -> bool {
    if x < x_start + border_radius && y < y_start + border_radius {
        return true;
    }
    if x < x_start + border_radius && y > y_end - border_radius {
        return true;
    }
    if x > x_end - border_radius && y < y_start + border_radius {
        return true;
    }
    if x > x_end - border_radius && y > y_end - border_radius {
        return true;
    }

    false
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
