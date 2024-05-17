extern crate image;
extern crate pixels;
extern crate winit;

pub mod utils;
pub mod update;
pub mod events;
pub mod stereo;

use crate::{
    model::scene::Scene,
    SCREEN_HEIGHT, SCREEN_HEIGHT_U32, SCREEN_WIDTH, SCREEN_WIDTH_U32,
};
use image::RgbaImage;
use std::
    sync::{Arc, RwLock}
;

use crate::render::render_threads::start_render_threads;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize, event_loop::EventLoop, platform::unix::WindowBuilderExtUnix, window::{Window, WindowBuilder}
};

pub fn display_scene(scene: Scene) {
    let scene = scene;

    // Set up window and event loop (can't move them elsewhere because of the borrow checker)
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32))
        .with_resizable(false)
        .with_base_size(LogicalSize::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32))
        .with_title("Image Viewer")
        .build(&event_loop)
        .unwrap();

    // Set up pixels object
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).unwrap()
    };

    // Setting up the render_threads and asking for the first image
    let scene = Arc::new(RwLock::new(scene));
    let (ra, tb) = start_render_threads(Arc::clone(&scene));
    let scene_change = false;
    tb.send(scene_change).unwrap();

    let mut img = RgbaImage::new(SCREEN_WIDTH_U32, SCREEN_HEIGHT_U32);

    // Display the scene
    display(&mut pixels, &mut img);

    // Set up event manager
    events::event_manager(event_loop, scene, img, pixels, ra, tb);
}

pub fn display(pixels: &mut Pixels<Window>, img: &mut RgbaImage) {
    // Copy image data to pixels buffer
    pixels.get_frame().copy_from_slice(&img);

    // Render the pixels buffer
    pixels.render().unwrap();
}
