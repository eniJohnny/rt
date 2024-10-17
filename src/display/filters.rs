use crate::FILTER;
use image::ImageBuffer;

pub fn apply_filter(image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
    match FILTER {
        "none" => {}
        "grayscale" => { greyscale(image); }
        "sepia" => { sepia(image); }
        "cartoon" => { cartoon(image); }
        _ => {}
    }
}

fn cartoon(image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
    let steps = 256 / 8;
    for pixel in image.pixels_mut() {
        let mut r = pixel[0] as u32 + 1;
        let mut g = pixel[1] as u32 + 1;
        let mut b = pixel[2] as u32 + 1;

        r = r / steps * steps;
        g = g / steps * steps;
        b = b / steps * steps;

        pixel[0] = r.min(255) as u8;
        pixel[1] = g.min(255) as u8;
        pixel[2] = b.min(255) as u8;
    }
    // greyscale(image);
}

fn greyscale(image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
    for pixel in image.pixels_mut() {
        let gray = (0.3 * pixel[0] as f32 + 0.59 * pixel[1] as f32 + 0.11 * pixel[2] as f32) as u8;
        pixel[0] = gray;
        pixel[1] = gray;
        pixel[2] = gray;
    }
}

fn sepia(image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
    for pixel in image.pixels_mut() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        let new_r = 0.393 * r + 0.769 * g + 0.189 * b;
        let new_g = 0.349 * r + 0.686 * g + 0.168 * b;
        let new_b = 0.272 * r + 0.534 * g + 0.131 * b;
        pixel[0] = new_r.min(255.) as u8;
        pixel[1] = new_g.min(255.) as u8;
        pixel[2] = new_b.min(255.) as u8;
    }
}