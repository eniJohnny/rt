extern crate image;
extern crate pixels;
extern crate winit;

use crate::{gui::draw_gui, model::scene::Scene, render::raycasting::render_scene_threadpool, SCREEN_HEIGHT, SCREEN_WIDTH};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent, KeyboardInput},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub fn display_scene(scene: Scene) {

    // DUMMY DATA FOR TESTING 
    // let mut rgb = [128,128,128];
    // let rays = vec![vec![Rgba([rgb[0], rgb[1], rgb[2], 255]); SCREEN_WIDTH as usize]; SCREEN_HEIGHT as usize];


    // Set up window and event loop
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32))
        .with_title("Image Viewer")
        .build(&event_loop)
        .unwrap();

    // Set up pixels object
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).unwrap()
    };

    display(&mut pixels, &scene);

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
                            println!("Left released");
                        }
                        Some(VirtualKeyCode::Right) => {
                            println!("Right released");
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
                // pixels.render().unwrap();
            }

            _ => (),
        }
    });
}

fn display (pixels: &mut Pixels<Window>, scene: &Scene) {

    let perf_timer = std::time::Instant::now();
    // Render the scene
    let img = render_scene_threadpool(scene);
    println!("Render time: {}ms", perf_timer.elapsed().as_millis());

    let perf_timer = std::time::Instant::now();
    // Draw the GUI
    let img = draw_gui(img);
    println!("GUI time: {}ms", perf_timer.elapsed().as_millis());

    let perf_timer = std::time::Instant::now();
    // Copy image data to pixels buffer
    pixels.get_frame().copy_from_slice(&img);
    println!("Copy time: {}ms", perf_timer.elapsed().as_millis());

    let perf_timer = std::time::Instant::now();
    // Render the pixels buffer
    pixels.render().unwrap();
    println!("Draw time: {}ms", perf_timer.elapsed().as_millis());
}