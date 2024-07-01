use image::{Rgba, RgbaImage};
use rusttype::{Font, Scale};

use crate::{ui::style::Style, SCREEN_HEIGHT_U32, SCREEN_WIDTH_U32};



pub fn draw_element_text(
    img: &mut RgbaImage,
    text: String,
    pos: (u32, u32),
    size: (u32, u32),
    format: &Style,
) {
    if let Some(color) = format.bg_color {
        draw_background(img, pos, size, color, format.border_radius);
    }
    draw_text2(
        img,
        (pos.0 + format.padding_left, pos.1 + format.padding_top),
        text,
        format,
    );
}


pub fn draw_text2(image: &mut RgbaImage, pos: (u32, u32), text: String, format: &Style) {
    // Load font
    let font_data = include_bytes!("../assets/JetBrainsMono-Regular.ttf");
    let font = &Font::try_from_bytes(font_data as &[u8]).expect("Error loading font");

    // Set font size and color
    let scale = Scale::uniform(format.font_size());
    let color = format.font_color();

    // Draw text
    let v_metrics = font.v_metrics(scale);
    let offset = rusttype::point(pos.0 as f32, pos.1 as f32 + v_metrics.ascent);

    for glyph in font.layout(&text, scale, offset) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
                    let pixel = image.get_pixel_mut(x as u32, y as u32);
                    *pixel = blend(color, pixel, v);
                }
            });
        }
    }
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


// Blend function to combine text color with background color
pub fn blend(text_color: &Rgba<u8>, background_color: &Rgba<u8>, alpha: f32) -> Rgba<u8> {
    let inv_alpha = 1.0 - alpha;
    let r = (text_color[0] as f32 * alpha + background_color[0] as f32 * inv_alpha) as u8;
    let g = (text_color[1] as f32 * alpha + background_color[1] as f32 * inv_alpha) as u8;
    let b = (text_color[2] as f32 * alpha + background_color[2] as f32 * inv_alpha) as u8;
    let a = (text_color[3] as f32 * alpha + background_color[3] as f32 * inv_alpha) as u8;
    Rgba([r, g, b, a])
}