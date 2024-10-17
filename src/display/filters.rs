use crate::FILTER;
use image::ImageBuffer;

pub fn apply_filter(image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
    match FILTER {
        "none" => {}
        "grayscale" => {
            for pixel in image.pixels_mut() {
                let gray = (0.3 * pixel[0] as f32 + 0.59 * pixel[1] as f32 + 0.11 * pixel[2] as f32) as u8;
                pixel[0] = gray;
                pixel[1] = gray;
                pixel[2] = gray;
            }
        }
        "sepia" => {
            for pixel in image.pixels_mut() {
                let r = pixel[0] as f32;
                let g = pixel[1] as f32;
                let b = pixel[2] as f32;
                let gray = 0.3 * r + 0.59 * g + 0.11 * b;
                pixel[0] = (gray + 40.0) as u8;
                pixel[1] = (gray + 20.0) as u8;
                pixel[2] = gray as u8;
            }
        }
        _ => {}
    }
}
