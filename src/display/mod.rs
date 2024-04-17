extern crate image;
extern crate pixels;
extern crate winit;

use image::{ImageBuffer, Rgba, RgbaImage};
use rusttype::{Font, Scale};
use pixels::{Pixels, SurfaceTexture};
use crate::parsing::get_scene;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use crate::{
    gui::{draw_sphere_gui, hide_gui, TextFormat},
    model::{maths::vec2::Vec2, Element},
    render::{get_closest_hit, render_scene, raycasting::render_scene_threadpool},
    SCREEN_HEIGHT,
    SCREEN_WIDTH
};

pub fn display_scene() {
    // Load scene
    let scene = get_scene();

    // Set up window and event loop (can't move them elsewhere because of the borrow checker)
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

    let mut img = render_scene(&scene);

    // Display the scene
    display(&mut pixels, &mut img);

    // Event loop (can't move it elsewhere because of the borrow checker)
    let mut mouse_position = (0.0, 0.0);
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::CloseRequested => {
                    // Close the window
                    *control_flow = ControlFlow::Exit
                }
                WindowEvent::CursorMoved { position, .. } => {
                    // Update the mouse position
                    mouse_position = (position.x, position.y);
                }
                WindowEvent::MouseInput { state, .. } => {
                    // If the mouse is clicked
                    if state == winit::event::ElementState::Released {
                        // Get element clicked
                        let x = mouse_position.0 as u32;
                        let y = mouse_position.1 as u32;

                        let rays = scene.camera().get_rays();
                        let ray = &rays[x as usize][y as usize];
                        let hit = get_closest_hit(&scene, ray);
                        if hit.is_some() {
                            let hit = hit.unwrap();
                            let element = hit.element();

                            display_element_infos(element, &mut img);
                            display(&mut pixels, &mut img);
                        } else {
                            hide_gui(&mut img, &scene);
                            display(&mut pixels, &mut img);
                        }
                    }
                }
                WindowEvent::KeyboardInput {input, ..} => {
                    // If a key is pressed
                    if input.state == winit::event::ElementState::Released{
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::Left) => {}
                            Some(VirtualKeyCode::Right) => {}
                            Some(VirtualKeyCode::Up) => {}
                            Some(VirtualKeyCode::Down) => {}
                            Some(VirtualKeyCode::Escape) => {
                                *control_flow = ControlFlow::Exit;
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
            Event::RedrawRequested(_) => {
                pixels.render().unwrap();
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

    // Copy image data to pixels buffer
    pixels.get_frame().copy_from_slice(&img);

    // Render the pixels buffer
    pixels.render().unwrap();
}

fn display_element_infos(element: &Element, img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let img = img;
    let shape = element.shape();

    if shape.as_sphere().is_some() {
        let sphere = shape.as_sphere().unwrap();
        draw_sphere_gui(img, sphere);
    } else if shape.as_plane().is_some() {
        let plane = shape.as_plane().unwrap();
        let pos = plane.pos();
        let dir = plane.dir();

        println!("Plane: pos: {:?}, dir: {:?}", pos, dir);
    } else if shape.as_cylinder().is_some() {
        let cylinder = shape.as_cylinder().unwrap();
        let pos = cylinder.pos();
        let dir = cylinder.dir();
        let radius = cylinder.radius();
        let height = cylinder.height();

        println!("Cylinder: pos: {:?}, dir: {:?}, radius: {}, height: {}", pos, dir, radius, height);
    } else if shape.as_cone().is_some() {
        let cone = shape.as_cone().unwrap();
        let pos = cone.pos();
        let dir = cone.dir();
        let radius = cone.radius();
        let height = cone.height();

        println!("Cone: pos: {:?}, dir: {:?}, radius: {}, height: {}", pos, dir, radius, height);
    }
}

pub fn draw_text (image: &mut RgbaImage, pos: &Vec2, text: String, format: &TextFormat) {

    let x = *pos.x() as u32 + 8;
    let y = *pos.y() as u32;

    // Load font
    let font_data = include_bytes!("../assets/JetBrainsMono-Regular.ttf");
    let font = &Font::try_from_bytes(font_data as &[u8]).expect("Error loading font");

    // Set font size and color
    let scale = Scale::uniform(format.font_size());
    let background_color = *format.background_color();
    let color = format.font_color();

    if background_color != Rgba([50, 50, 50, 255]) {
        draw_text_background(image, pos, format.size(), background_color, format.font_size() as u32);
    }

    // Draw text
    let v_metrics = font.v_metrics(scale);
    let offset = rusttype::point(x as f32, y as f32 + v_metrics.ascent);

    for glyph in font.layout(&text, scale, offset) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
                    let pixel = image.get_pixel_mut(x as u32, y as u32);
                    *pixel = blend(color, pixel, v);
                }
            });
        }
    }
}

// Blend function to combine text color with background color
fn blend(text_color: &Rgba<u8>, background_color: &Rgba<u8>, alpha: f32) -> Rgba<u8> {
    let inv_alpha = 1.0 - alpha;
    let r = (text_color[0] as f32 * alpha + background_color[0] as f32 * inv_alpha) as u8;
    let g = (text_color[1] as f32 * alpha + background_color[1] as f32 * inv_alpha) as u8;
    let b = (text_color[2] as f32 * alpha + background_color[2] as f32 * inv_alpha) as u8;
    let a = (text_color[3] as f32 * alpha + background_color[3] as f32 * inv_alpha) as u8;
    Rgba([r, g, b, a])
}

fn draw_text_background (image: &mut RgbaImage, pos: &Vec2, size: &Vec2, color: Rgba<u8>, height: u32) {
    let mut width = *size.x() as u32;

    if *pos.x() as u32 + width > image.width() as u32 {
        width = image.width() - *pos.x() as u32;
    }

    let x = image.width() - width;
    let y = *pos.y() as u32;

    for x in x..x + width {
        for y in y..y + height {
            image.put_pixel(x, y, color);
        }
    }
}