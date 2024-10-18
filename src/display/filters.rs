use crate::{EDGE_THRESHOLD, FILTER};
use image::ImageBuffer;

pub fn apply_filter(image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
    match FILTER {
        "grayscale" => { grayscale(image); }
        "sepia" => { sepia(image); }
        "cartoon" => { cartoon(image); }
        _ => {}
    }
}

fn cartoon(image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
    let steps = 256 / 8;
    let mut gray = image.clone();

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
    grayscale(&mut gray);
    sobel(&mut gray, image);
}

fn grayscale(image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
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
fn sobel(gray:&mut ImageBuffer<image::Rgba<u8>, Vec<u8>>, image: &mut ImageBuffer<image::Rgba<u8>, Vec<u8>>) {
    let (width, height) = image.dimensions();
    let mut edge: ImageBuffer<image::Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    // Sobel kernels
    let sobel_x: [[i32; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let sobel_y: [[i32; 3]; 3] = [[1, 2, 1], [0, 0, 0], [-1, -2, -1]];

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let mut gx = 0;
            let mut gy = 0;

            for ky in -1..=1 {
                for kx in -1..=1 {
                    let ky_index = (ky + 1) as usize;
                    let kx_index = (kx + 1) as usize;

                    let pixel = gray.get_pixel((x as i32 + kx) as u32, (y as i32 + ky) as u32);
                    let intensity = pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32 / 3;

                    gx += intensity as i32 * sobel_x[ky_index][kx_index];
                    gy += intensity as i32 * sobel_y[ky_index][kx_index];
                }
            }

            // gradient magnitude
            let g = ((gx * gx + gy * gy) as f64).sqrt().min(255.) as u8;

            edge.put_pixel(x, y, image::Rgba([g, g, g, 255]));
        }
    }

    for y in 0..height {
        for x in 0..width {
            let edge_pixel = edge.get_pixel(x, y);
            let original_pixel = image.get_pixel_mut(x, y);

            let edge_intensity = (edge_pixel[0] as u32 + edge_pixel[1] as u32 + edge_pixel[2] as u32) / 3;
            // let average_color = (original_pixel[0] as u32 + original_pixel[1] as u32 + original_pixel[2] as u32) / 3;
            // let simplified_color = if average_color >= 128 { 255 } else { 0 };

            if edge_intensity > EDGE_THRESHOLD {
                *original_pixel = image::Rgba([0, 0, 0, 255]);
            }
        }
    }
}