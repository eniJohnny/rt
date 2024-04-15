use image::Rgba;

use crate::GUI_WIDTH;

pub fn draw_gui (img: image::ImageBuffer<Rgba<u8>, Vec<u8>>) -> image::ImageBuffer<Rgba<u8>, Vec<u8>>{
    // Create a new RGBA image
    let width = img.width();
    let x_start = width - GUI_WIDTH;
    let height = img.height();
    let mut img = img;

    // Draw the GUI
    for x in x_start..width {
        for y in 0..height {
            if is_border(x, y, x_start, width, height) {
                img.put_pixel(x, y, Rgba([230, 230, 230, 255]));
            } else {
                img.put_pixel(x, y, Rgba([255, 255, 255, 255]));
            }
        }
    }

    img
}

fn is_border (x: u32, y: u32, x_start: u32, width: u32, height: u32) -> bool {
    let border_width = 4;

    if x < x_start + border_width {
        return true;
    } else if x >= width - border_width {
        return true;
    } else if y < border_width {
        return true;
    } else if y > height - border_width {
        return true;
    } else {
        return false;
    }
}