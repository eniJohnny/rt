use std::{cmp::min, sync::{Arc, RwLock}, thread, time::Duration};

use image::Rgba;

use crate::{parsing::get_scene, render::render_threads::start_render_threads};

pub fn display_stereo_scene() {
    let mut scene = Arc::new(RwLock::new(get_scene()));

    let mut final_image = false;
    let mut left_img: image::ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::new(1, 1);
    let mut right_img: image::ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::new(1, 1);
    

    // Render the left image
    scene.write().unwrap().camera_mut().move_left_stereo();

    let (ra, tb) = start_render_threads(Arc::clone(&scene));
    tb.send(true).unwrap();
    thread::sleep(Duration::from_millis(10));
    tb.send(false).unwrap();

    while !final_image {

        if let Ok((render_img, final_img)) = ra.try_recv() {
            left_img = render_img;
            final_image = final_img;
        }

        if !final_image {
            thread::sleep(Duration::from_millis(20));
            tb.send(false).unwrap();
        }
    }

    // Render the right image
    scene.write().unwrap().camera_mut().move_right_stereo();
    scene.write().unwrap().camera_mut().move_right_stereo();
    let (ra, tb) = start_render_threads(Arc::clone(&scene));
    tb.send(true).unwrap();
    thread::sleep(Duration::from_millis(10));
    tb.send(false).unwrap();
    final_image = false;

    while !final_image {

        if let Ok((render_img, final_img)) = ra.try_recv() {
            right_img = render_img;
            final_image = final_img;
        }

        if !final_image {
            thread::sleep(Duration::from_millis(20));
            tb.send(false).unwrap();
        }
    }

    // Remove blue channel from left image
    let mut left_img_no_blue = left_img.clone();
    for (_, _, pixel) in left_img_no_blue.enumerate_pixels_mut() {
        pixel[2] = 0;
    }

    // Remove red channel from right image
    let mut right_img_no_red = right_img.clone();
    for (_, _, pixel) in right_img_no_red.enumerate_pixels_mut() {
        pixel[0] = 0;
    }

    // Mix the two images
    let mut stereo_image = image::ImageBuffer::new(left_img.width(), left_img.height());

    for (x, y, pixel) in stereo_image.enumerate_pixels_mut() {
        let left_pixel = left_img_no_blue.get_pixel(x, y);
        let right_pixel = right_img_no_red.get_pixel(x, y);

        let new_pixel: Rgba<u8> = Rgba([
            left_pixel[0],
            min(left_pixel[1] as u32 + right_pixel[1] as u32, 255) as u8,
            right_pixel[2],
            255
        ]);
        *pixel = new_pixel;
    }

    left_img_no_blue.save("left_img.png").unwrap();
    right_img_no_red.save("right_img.png").unwrap();
    stereo_image.save("stereo_image.png").unwrap();

}