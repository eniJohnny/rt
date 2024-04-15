extern crate image;
extern crate pixels;
extern crate winit;

use std::cmp::{max, min};

use crate::{model::scene::Scene, SCREEN_HEIGHT, SCREEN_WIDTH};
use image::{GenericImage, Rgba, RgbaImage};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent, KeyboardInput},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub fn display_scene(scene: &Scene) {

    // DUMMY DATA FOR TESTING 
    let mut rgb = [128,128,128];
    let rays = vec![vec![Rgba([rgb[0], rgb[1], rgb[2], 255]); 1400]; 900];


    // Set up window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with_title("Image Viewer")
        .build(&event_loop)
        .unwrap();

    // Set up pixels object
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture).unwrap()
    };

    display(rays, &mut pixels);

    // Event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: KeyboardInput { state, virtual_keycode, .. },
                    ..
                },
                ..
            } => {
                if state == winit::event::ElementState::Released {
                    let perf_timer = std::time::Instant::now();
                    match virtual_keycode {
                        Some(VirtualKeyCode::Left) => {
                            for i in 0..3 {
                                rgb[i] = max(0, (rgb[i] as i32 - 10) as u8);
                            }
                            let rays = vec![vec![Rgba([rgb[0], rgb[1], rgb[2], 255]); 1400]; 900];
                            display(rays, &mut pixels);
                            println!("Left released, RGB value: {:?}. Redrawn in {:?}", rgb, perf_timer.elapsed());
                        }
                        Some(VirtualKeyCode::Right) => {
                            
                            for i in 0..3 {
                                rgb[i] = min(255, rgb[i] + 10);
                            }
                            let rays = vec![vec![Rgba([rgb[0], rgb[1], rgb[2], 255]); 1400]; 900];
                            display(rays, &mut pixels);
                            println!("Right released, RGB value: {:?}. Redrawn in {:?}", rgb, perf_timer.elapsed());
                        }
                        Some(VirtualKeyCode::Up) => {
                            
                        }
                        Some(VirtualKeyCode::Down) => {
                            
                        }
                        Some(VirtualKeyCode::Escape) => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => (),
                    }
                }
            }

            Event::RedrawRequested(_) => {
                pixels.render().unwrap();
            }

            _ => (),
        }
    });
}

fn display (rays: Vec<Vec<Rgba<u8>>>, pixels: &mut Pixels<Window>) {
    // Create a new RGBA image
    let width = rays[0].len() as u32;
    let height = rays.len() as u32;
    let mut img = RgbaImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, rays[y as usize][x as usize]);
        }
    }

    // Resize the image buffer for display
    let resized_img = image::imageops::resize(&img, SCREEN_WIDTH, SCREEN_HEIGHT, image::imageops::FilterType::Nearest);
    
    // Copy image data to pixels buffer
    pixels.get_frame().copy_from_slice(&resized_img.to_vec());

    // Render the pixels buffer
    pixels.render().unwrap();
}