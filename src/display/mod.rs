extern crate image;
extern crate pixels;
extern crate winit;

pub mod events2;
pub mod utils;

use crate::{
    model::scene::Scene, SCREEN_HEIGHT, SCREEN_HEIGHT_U32, SCREEN_WIDTH, SCREEN_WIDTH_U32,
};
use events2::main_loop;
use image::RgbaImage;
use std::{
    ptr::copy_nonoverlapping,
    sync::{Arc, RwLock},
};

use crate::render::render_threads::start_render_threads;
use pixels::Pixels;
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub fn display_scene(scene: Scene) {
    let scene = scene;

    // Set up window and event loop (can't move them elsewhere because of the borrow checker)
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32))
        .with_title("Image Viewer")
        .build(&event_loop)
        .unwrap();

    // Set up pixels object
    let pixels = {
        let texture = pixels::SurfaceTexture::new(SCREEN_WIDTH_U32, SCREEN_HEIGHT_U32, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, texture).unwrap()
    };

    let scene = Arc::new(RwLock::new(scene));

    main_loop(event_loop, scene, pixels);
}

pub fn display(pixels: &mut Pixels, img: &mut RgbaImage) {
    // Copy image data to pixels buffer

    // unsafe {
    //     copy_nonoverlapping(img_data.as_ptr(), frame.as_mut_ptr(), img_data.len());
    // }

    pixels.frame_mut().copy_from_slice(&img);

    // Render the pixels buffer
    pixels.render().unwrap();
}
