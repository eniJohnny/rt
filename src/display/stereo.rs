use std::{sync::{mpsc::{Receiver, Sender}, Arc, RwLock}, thread, time::Duration};

use image::{ImageBuffer, Rgba};


pub fn get_stereo_image(scene: Arc<RwLock<crate::model::scene::Scene>>, ra: &Receiver<(ImageBuffer<Rgba<u8>, Vec<u8>>, bool)>, tb: &Sender<bool>) -> image::ImageBuffer<Rgba<u8>, Vec<u8>>{
    let mut final_image = false;
    let mut final_image2 = false;
    let mut left_img: image::ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::new(1, 1);
    let mut right_img: image::ImageBuffer<Rgba<u8>, Vec<u8>> = image::ImageBuffer::new(1, 1);
    
    // Render the left image
    scene.write().unwrap().camera_mut().move_left_stereo();

    tb.send(true).unwrap();
    thread::sleep(Duration::from_millis(10));
    tb.send(false).unwrap();

    while !final_image {

        if let Ok((render_img, final_img)) = ra.recv() {
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
    tb.send(true).unwrap();
    while let Ok((_, _)) = ra.try_recv() {};
    thread::sleep(Duration::from_millis(10));
    tb.send(false).unwrap();

    while !final_image2 {

        if let Ok((render_img, final_img)) = ra.recv() {
            right_img = render_img;
            final_image2 = final_img;
        }

        if !final_image2 {
            thread::sleep(Duration::from_millis(20));
            tb.send(false).unwrap();
        }
    }
    scene.write().unwrap().camera_mut().move_left_stereo();

    // Mix the two images
    let mut stereo_image = image::ImageBuffer::new(left_img.width(), left_img.height());

    for (x, y, pixel) in stereo_image.enumerate_pixels_mut() {
        let left_pixel = left_img.get_pixel(x, y);
        let right_pixel = right_img.get_pixel(x, y);

        let new_pixel: Rgba<u8> = Rgba([
            left_pixel[0],
            right_pixel[1],
            right_pixel[2],
            255
        ]);
        *pixel = new_pixel;
    }

    return stereo_image;
}