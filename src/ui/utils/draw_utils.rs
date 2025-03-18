use super::{style::Style, HitBox};
use crate::{SCREEN_HEIGHT_U32, SCREEN_WIDTH_U32};
use image::{Rgba, RgbaImage};
use rusttype::{Font, Scale};

pub fn draw_element_text(
    img: &mut RgbaImage,
    mut text: String,
    mut pos: (i32, i32),
    size: (u32, u32),
    style: &Style,
) {
    if pos.0 < 0 {
        pos.0 = 0;
    }
    if pos.1 < 0 {
        pos.1 = 0;
    }
    let pos = (pos.0 as u32, pos.1 as u32);
    draw_background(img, pos, size, style);

    let mut padding_left = style.padding_left;
    if style.text_center {
        let available_width = size.0 - style.padding_left - style.padding_right;
        let text_allowed_len = available_width as usize / (style.font_size() as usize / 2) - 1;
        if text.len() > text_allowed_len {
            if text_allowed_len < 2 {
                text = "".to_string();
            } else {
                text.truncate(text_allowed_len - 2);
                text = text + ".."
            }
        }
        let text_width = style.font_size() as u32 / 2 * (text.len() as u32 + 1);
        padding_left += (available_width - text_width) / 2;
    }
    
    draw_text(
        img,
        (pos.0 + padding_left, pos.1 + style.padding_top),
        text,
        style,
    );
}


fn draw_text(image: &mut RgbaImage, pos: (u32, u32), text: String, format: &Style) {
    // Load font
    let font_data = include_bytes!("../../assets/JetBrainsMono-Regular.ttf");
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
    pos: (i32, i32),
    size: (u32, u32),
    value: bool,
    style: &Style,
) {
    let pos = (pos.0 as u32, pos.1 as u32);
    let checkbox_size = (18, 18);
    let height = size.1;
    if size.1 == 0 {
        return;
    }
    let checkbox_pos = (
        pos.0 + size.0 - style.padding_right - checkbox_size.0,
        pos.1 + (height - checkbox_size.1) / 2,
    );

    draw_box(
        img,
        checkbox_pos,
        checkbox_size,
        Rgba([40, 10, 90, 255]),
        style.border_radius,
    );
    draw_box(
        img,
        (checkbox_pos.0 + 2, checkbox_pos.1 + 2),
        (checkbox_size.0 - 4, checkbox_size.1 - 4),
        Rgba([255, 255, 255, 255]),
        style.border_radius,
    );
    if value {
        draw_box(
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
    style: &Style
) {
    if style.border_top > 0 || style.border_bot > 0 || style.border_left > 0 || style.border_right > 0 {
        if let Some(color) = style.border_color {
            draw_box(img, pos, size, color, style.border_radius);
            if let Some(color) = style.bg_color {
                let pos = (pos.0 + style.border_left, pos.1 + style.border_top);
                let size = (size.0 - style.border_left - style.border_right, size.1 - style.border_top - style.border_bot);
                draw_box(img, pos, size, color, style.border_radius);
            }
        }
    } else if let Some(color) = style.bg_color {
        draw_box(img, pos, size, color, style.border_radius);
    }
}

pub fn draw_box(
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


pub fn is_inside_box(to_check: (u32, u32), box_pos: (u32, u32), box_size: (u32, u32)) -> bool {
    return to_check.0 > box_pos.0
        && to_check.0 < box_pos.0 + box_size.0
        && to_check.1 > box_pos.1
        && to_check.1 < box_pos.1 + box_size.1;
}



pub fn get_needed_height(hitbox_vec: &Vec<HitBox>) -> u32 {
    let mut max_needed_height = 0;
    for hitbox in hitbox_vec {
        if !hitbox.disabled && hitbox.visible{
            let needed_height = hitbox.pos.1 as u32 + hitbox.size.1;
            if needed_height > max_needed_height {
                max_needed_height = needed_height;
            }
        }
    }
    max_needed_height
}

pub fn get_size(text: &String, style: &Style, max_size: (u32, u32)) -> (u32, u32) {
    let mut height = style.font_size() as u32 + style.padding_bot + style.padding_top;
    let mut width = (style.font_size() / 2. * text.len() as f32) as u32
        + style.padding_left
        + style.padding_right;

    let wanted_width = style.width.max(max_size.0 * style.fill_width as u32);
    let wanted_height = style.height;

    if wanted_height > height {
        height = wanted_height;
    }
    if wanted_width > width {
        width = wanted_width;
    }
    if width > max_size.0 && style.multilines {
        let lines = split_in_lines(text.clone(), style.width, style);
        height += (style.font_size() as u32 + style.padding_bot) * lines.len() as u32;
    }
    (width.min(max_size.0), height.min(max_size.1))
}

pub fn split_in_lines(str: String, available_width: u32, format: &Style) -> Vec<String> {
    let mut current_width = 0;
    let mut lines = vec![];
    let mut txt_split = str.split(" ");
    let mut line = String::from("");
    while let Some(str) = txt_split.next() {
        let word_width = format.font_size() as u32 / 2 * (str.len() + 1) as u32;
        if current_width + word_width > available_width {
            current_width = word_width;
            lines.push(line.clone());
            line = str.to_string() + " ";
        } else {
            line += str;
            line += " ";
            current_width += word_width;
        }
    }
    lines.push(line);
    lines
}