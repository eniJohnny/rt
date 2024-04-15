extern crate image;
extern crate pixels;
extern crate winit;

use crate::{gui::draw_gui, model::scene::Scene, GUI_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH};
use image::{Rgba, RgbaImage};
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
    let rays = vec![vec![Rgba([rgb[0], rgb[1], rgb[2], 255]); SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];


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
                    match virtual_keycode {
                        Some(VirtualKeyCode::Left) => {
                            let perf_timer = std::time::Instant::now();
                            for i in 0..3 {
                                if rgb[i] > 10 {
                                    rgb[i] -= 10;
                                } else {
                                    rgb[i] = 0;
                                }
                            }
                            let rays = vec![vec![Rgba([rgb[0], rgb[1], rgb[2], 255]); SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];

                            display(rays, &mut pixels);
                            println!("Left released, RGB value: {:?}. Redrawn in {:?}", rgb, perf_timer.elapsed());
                        }
                        Some(VirtualKeyCode::Right) => {
                            for i in 0..3 {
                                if rgb[i] < 245 {
                                    rgb[i] += 10;
                                } else {
                                    rgb[i] = 255;
                                }
                            }
                            let rays = vec![vec![Rgba([rgb[0], rgb[1], rgb[2], 255]); SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];
                            
                            let perf_timer = std::time::Instant::now();
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
    let width = rays[0].len() as u32 ;
    let height = rays.len() as u32;
    let mut img: image::ImageBuffer<Rgba<u8>, Vec<u8>> = RgbaImage::new(width, height);

    for x in 0..width - GUI_WIDTH  {
        for y in 0..height {
            img.put_pixel(x, y, rays[y as usize][x as usize]);
        }
    }

    img = draw_gui(img);
    // Copy image data to pixels buffer
    pixels.get_frame().copy_from_slice(&img);

    // Render the pixels buffer
    pixels.render().unwrap();
}