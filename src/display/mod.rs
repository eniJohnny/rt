extern crate image;
extern crate pixels;
extern crate winit;

pub mod utils;

use crate::{
    gui::{
        draw::{draw_plane_gui, draw_sphere_gui},
        gui_clicked, hide_gui, hitbox_contains, Gui, TextFormat,
    },
    model::{materials::Color, maths::vec2::Vec2, scene::Scene, shapes::Shape, Element},
    render::raycasting::{get_closest_hit, get_ray},
    FPS, GUI_HEIGHT, GUI_WIDTH, SCREEN_HEIGHT, SCREEN_HEIGHT_U32, SCREEN_WIDTH, SCREEN_WIDTH_U32,
};
use image::{ImageBuffer, Rgba, RgbaImage};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread::{self, sleep},
    time::{Duration, Instant},
};

use crate::render::render_threads::start_render_threads;
use pixels::{Pixels, SurfaceTexture};
use rusttype::{Font, Scale};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use self::utils::{move_camera, update_color, update_shape};

const RGB_KEYS: [&str; 3] = ["colr", "colg", "colb"];
const CAM_MOVE_KEYS: [VirtualKeyCode; 10] = [
    VirtualKeyCode::W,
    VirtualKeyCode::A,
    VirtualKeyCode::S,
    VirtualKeyCode::D,
    VirtualKeyCode::Up,
    VirtualKeyCode::Left,
    VirtualKeyCode::Down,
    VirtualKeyCode::Right,
    VirtualKeyCode::Space,
    VirtualKeyCode::LShift,
];

pub fn display_scene(scene: Scene) {
    let scene = scene;
    let format = TextFormat::new(
        Vec2::new(GUI_WIDTH as f64, GUI_HEIGHT as f64),
        24.,
        Rgba([255, 255, 255, 255]),
        Rgba([89, 89, 89, 255]),
    );
    let editing_format = TextFormat::new(
        Vec2::new(400., 400.),
        24.,
        Rgba([0, 0, 0, 255]),
        Rgba([255, 255, 255, 255]),
    );

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

    // Setting up the render_threads and asking for the first image
    let scene = Arc::new(RwLock::new(scene));
    let (ra, tb) = start_render_threads(Arc::clone(&scene));
    let mut scene_change = false;
    let mut image_requested = true;
    let mut final_image = false;
    tb.send(scene_change).unwrap();

    let mut img = RgbaImage::new(SCREEN_WIDTH_U32, SCREEN_HEIGHT_U32);

    // Display the scene
    display(&mut pixels, &mut img);

    let mut current_input: Option<VirtualKeyCode> = None;
    let mut time_of_last_move = Instant::now();
    let time_between_move = Duration::from_millis(1000 / FPS);

    // Event loop (can't move it elsewhere because of the borrow checker)
    let mut mouse_position = (0.0, 0.0);
    event_loop.run(move |event, _, control_flow: &mut ControlFlow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(20));
        if scene_change {
            thread::sleep(Duration::from_millis(10));
            tb.send(false).unwrap();
            let (render_img, _) = ra.recv().unwrap();
            img = render_img;
            display(&mut pixels, &mut img);
            scene_change = false;
        }
        match event {
            Event::WindowEvent { event, .. } => match event {
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

                        /////////////////////////////////////////////////////////////////////
                        /////////////////////////// ATTENTION ///////////////////////////////
                        /////////////////////////////////////////////////////////////////////
                        // Tant que l'on maintiens une reference write du RwLock(Sorte de mutex),
                        // tous les threads de render seront bloques.
                        let mut scene = scene.write().unwrap();
                        let ray = get_ray(&scene, x as usize, y as usize);
                        let hit = get_closest_hit(&scene, &ray);

                        if gui_clicked(mouse_position, &scene.gui) {
                            // If the GUI is clicked

                            let mut editing = false;

                            if hitbox_contains(scene.gui.cancel_hitbox(), mouse_position) {
                                // Close GUI
                                hide_gui(&mut img, &scene);
                                scene.gui = Gui::new();
                                display(&mut pixels, &mut img);
                            } else if hitbox_contains(scene.gui.apply_hitbox(), mouse_position) {
                                // Apply changes for every key
                                for i in 0..scene.gui.keys().len() {
                                    let key = scene.gui.keys()[i].clone();
                                    let value = scene.gui.values()[i].clone().replace("_", "");
                                    let element_index = scene.gui.element_index();
                                    let elem = &scene.elements()[element_index];
                                    let shape = elem.shape();
                                    let material = elem.material();

                                    if RGB_KEYS.contains(&key.as_str()) {
                                        let color: Color = material.color(0, 0);
                                        let new_material = update_color(key, value, color);
                                        if new_material.is_some() {
                                            scene.elements_as_mut()[element_index]
                                                .set_material(new_material.unwrap());
                                        }
                                    } else {
                                        let new_shape = update_shape(shape, key, value);
                                        if new_shape.is_some() {
                                            scene.elements_as_mut()[element_index]
                                                .set_shape(new_shape.unwrap());
                                        }
                                    }
                                    scene_change = true;
                                }
                                display(&mut pixels, &mut img);
                            } else {
                                let index = scene.gui.updating_index();
                                let value = scene.gui.values()[index].clone().replace("_", "");
                                let hitbox = scene.gui.hitboxes()[index].clone();
                                let pos = Vec2::new(*hitbox.0.x() as f64, *hitbox.0.y() as f64);
                                let background_pos =
                                    Vec2::new(*hitbox.0.x() as f64 - 10., *hitbox.0.y() as f64);

                                let text = format!("{}", value);
                                draw_text(&mut img, &background_pos, " ".to_string(), &format);
                                draw_text(&mut img, &pos, text, &format);
                                display(&mut pixels, &mut img);
                            }
                            if scene.gui.keys().len() > 0 {
                                for i in 0..scene.gui.keys().len() {
                                    let hitbox = scene.gui.hitboxes()[i].clone();
                                    if hitbox_contains(&hitbox, mouse_position) {
                                        // Reset previous value formatting
                                        if scene.gui.updating() {
                                            let index = scene.gui.updating_index();
                                            let value =
                                                scene.gui.values()[index].clone().replace("_", "");
                                            let hitbox = scene.gui.hitboxes()[index].clone();
                                            let pos = Vec2::new(
                                                *hitbox.0.x() as f64,
                                                *hitbox.0.y() as f64,
                                            );
                                            let background_pos = Vec2::new(
                                                *hitbox.0.x() as f64 - 10.,
                                                *hitbox.0.y() as f64,
                                            );

                                            let text = format!("{}", value);
                                            draw_text(
                                                &mut img,
                                                &background_pos,
                                                " ".to_string(),
                                                &format,
                                            );
                                            draw_text(&mut img, &pos, text, &format);
                                        }

                                        // Update value
                                        scene.gui.set_updating(true);
                                        scene.gui.set_updating_index(i);
                                        editing = true;
                                    }
                                }
                                if editing == false {
                                    let index = scene.gui.updating_index();
                                    let value = scene.gui.values()[index].clone().replace("_", "");
                                    let hitbox = scene.gui.hitboxes()[index].clone();
                                    let pos =
                                        Vec2::new(*hitbox.0.x() as f64 - 10., *hitbox.0.y() as f64);

                                    let text = format!("{}", value);
                                    draw_text(&mut img, &pos, text, &format);
                                    scene.gui.set_updating(false);
                                }
                            }

                            if scene.gui.updating() {
                                let index = scene.gui.updating_index();
                                let value = scene.gui.values()[index].clone().replace("_", "");
                                let hitbox = scene.gui.hitboxes()[index].clone();
                                let pos =
                                    Vec2::new(*hitbox.0.x() as f64 - 10., *hitbox.0.y() as f64);

                                let text = format!("{}_", value);
                                draw_text(&mut img, &pos, text, &editing_format);
                                display(&mut pixels, &mut img);
                            }
                        } else if hit.is_some() {
                            let hit = hit.unwrap();
                            let element = hit.element();

                            let element_index: usize = scene
                                .elements()
                                .iter()
                                .position(|e| {
                                    let e_shape = e.shape();
                                    let element_shape = element.shape();
                                    get_shape(e_shape) == get_shape(element_shape)
                                })
                                .unwrap()
                                as usize;

                            scene.gui = display_element_infos(element, &mut img);
                            scene.gui.set_element_index(element_index);
                            display(&mut pixels, &mut img);
                        } else {
                            hide_gui(&mut img, &scene);
                            scene.gui = Gui::new();
                            display(&mut pixels, &mut img);
                        }
                    }
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    // If a key is pressed
                    if input.state == winit::event::ElementState::Released {
                        current_input = None;
                    } else if input.state == winit::event::ElementState::Pressed {
                        current_input = input.virtual_keycode;
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                image_requested = true;
            }
            _ => (),
        }
        if Instant::now() - time_of_last_move > time_between_move {
            if let Some(key_code) = current_input {
                let mut scene = scene.write().unwrap();
                match Some(key_code) {
                    c if CAM_MOVE_KEYS.contains(&c.expect("Wrong key")) => {
                        // Camera movements
                        let camera = scene.camera_mut();
                        move_camera(camera, c);
                        scene_change = true;
                    }
                    Some(VirtualKeyCode::Escape) => {
                        *control_flow = ControlFlow::Exit;
                    }
                    c if (c >= Some(VirtualKeyCode::Numpad0)
                        && c <= Some(VirtualKeyCode::Numpad9)) || (c >= Some(VirtualKeyCode::Key1) && c <= Some(VirtualKeyCode::Key0)) =>
                    {
                        if scene.gui.updating() == false {
                            return;
                        }

                        // Add c to the edited value
                        let index = scene.gui.updating_index();
                        let hitbox = scene.gui.hitboxes()[index].clone();
                        let new_hitbox = (
                            Vec2::new(*hitbox.0.x() - 10., *hitbox.0.y()),
                            hitbox.1.clone(),
                        );
                        let pos = new_hitbox.0.clone();

                        // -10 px for the _ character
                        let pos = Vec2::new(*pos.x() as f64 - 10., *pos.y() as f64);
                        let value = scene.gui.values()[index].clone().replace("_", "");
                        let mut number = c.unwrap() as u8;
                        if c.unwrap() >= VirtualKeyCode::Key1 && c.unwrap() <= VirtualKeyCode::Key9{
                            number = number + 1;
                        } else if c.unwrap() == VirtualKeyCode::Key0 {
                            number = 0;
                        } else {
                            number = number - VirtualKeyCode::Numpad0 as u8;
                        }

                        let value = format!("{}{:?}_", value, number);

                        scene.gui.set_updates(index, &value, &new_hitbox);

                        draw_text(&mut img, &pos, value, &editing_format);
                        display(&mut pixels, &mut img);
                        sleep(Duration::from_millis(50));
                    }
                    Some(VirtualKeyCode::Back) => {
                        // Remove last character from the edited value
                        let index = scene.gui.updating_index();
                        let hitbox = scene.gui.hitboxes()[index].clone();
                        let new_hitbox = (
                            Vec2::new(*hitbox.0.x() + 10., *hitbox.0.y()),
                            hitbox.1.clone(),
                        );
                        let pos = new_hitbox.0.clone();
                        // -10 for the _
                        let pos = Vec2::new(*pos.x() as f64 - 10., *pos.y() as f64);
                        let background_pos =
                            Vec2::new(*hitbox.0.x() as f64 - 10., *hitbox.0.y() as f64);
                        let value = scene.gui.values()[index].clone().replace("_", "");

                        if value.len() > 0 {
                            let value = value.chars().take(value.len() - 1).collect::<String>();
                            let value = format!("{}_", value);

                            scene.gui.set_updates(index, &value, &new_hitbox);

                            draw_text(&mut img, &background_pos, " ".to_string(), &format);
                            draw_text(&mut img, &pos, value, &editing_format);
                            display(&mut pixels, &mut img);
                        }
                    }
                    Some(VirtualKeyCode::NumpadDecimal) | Some(VirtualKeyCode::Period) => {
                        // Add a comma to the edited value
                        let index = scene.gui.updating_index();
                        let hitbox = scene.gui.hitboxes()[index].clone();
                        let new_hitbox = (
                            Vec2::new(*hitbox.0.x() - 10., *hitbox.0.y()),
                            hitbox.1.clone(),
                        );
                        let pos = new_hitbox.0.clone();
                        // -10 for the _
                        let pos = Vec2::new(*pos.x() as f64 - 10., *pos.y() as f64);
                        let mut value = scene.gui.values()[index].clone().replace("_", "");

                        if value.contains(".") {
                            return;
                        }

                        if value.len() == 0 {
                            value = "0._".to_string();
                        } else {
                            value = format!("{}._", value);
                        }
                        scene.gui.set_updates(index, &value, &new_hitbox);

                        draw_text(&mut img, &pos, value, &editing_format);
                        display(&mut pixels, &mut img);
                    }
                    Some(VirtualKeyCode::NumpadSubtract) => {
                        // Add a minus to the edited value
                        let index = scene.gui.updating_index();
                        let hitbox = scene.gui.hitboxes()[index].clone();
                        let mut offset = -10.;
                        if scene.gui.values()[index].contains("-") {
                            offset = 10.;
                        }
                        let new_hitbox = (
                            Vec2::new(*hitbox.0.x() + offset, *hitbox.0.y()),
                            hitbox.1.clone(),
                        );
                        let pos = new_hitbox.0.clone();
                        // -10 for the _
                        let pos = Vec2::new(*pos.x() as f64 - 10., *pos.y() as f64);
                        let mut value = scene.gui.values()[index].clone().replace("_", "");

                        value.push('_');
                        if value.contains("-") {
                            value = value.replace("-", "");
                        } else {
                            value = format!("-{}", value);
                        }

                        scene.gui.set_updates(index, &value, &new_hitbox);

                        draw_text(&mut img, &pos, value, &editing_format);
                        display(&mut pixels, &mut img);
                    }
                    _ => (),
                }
            }
            time_of_last_move = Instant::now();
        }
        if scene_change {
            final_image = false;
            tb.send(true).unwrap();
        } else if image_requested {
            if let Ok((render_img, final_img)) = ra.try_recv() {
                img = render_img;
                display(&mut pixels, &mut img);
                final_image = final_img;
                image_requested = false;
            }
        } else if !final_image {
            tb.send(false).unwrap();
            image_requested = true;
        }
    });
}

fn display(pixels: &mut Pixels<Window>, img: &mut RgbaImage) {
    // Copy image data to pixels buffer
    pixels.get_frame().copy_from_slice(&img);

    // Render the pixels buffer
    pixels.render().unwrap();
}

fn display_element_infos(element: &Element, img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Gui {
    let img = img;
    let shape = element.shape();
    let material = element.material();

    if shape.as_sphere().is_some() {
        let sphere = shape.as_sphere().unwrap();
        return draw_sphere_gui(img, sphere, material);
    } else if shape.as_plane().is_some() {
        let plane = shape.as_plane().unwrap();
        return draw_plane_gui(img, plane, material);
    } else {
        return Gui::new();
    }
}

pub fn draw_text(image: &mut RgbaImage, pos: &Vec2, text: String, format: &TextFormat) {
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
        draw_text_background(
            image,
            pos,
            format.size(),
            background_color,
            format.font_size() as u32,
        );
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

fn draw_text_background(
    image: &mut RgbaImage,
    pos: &Vec2,
    size: &Vec2,
    color: Rgba<u8>,
    height: u32,
) {
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

fn get_shape(shape: &dyn Shape) -> String {
    if let Some(sphere) = shape.as_sphere() {
        return format!(
            "Sphere: pos: {:#?}, radius: {}",
            sphere.pos(),
            sphere.radius()
        );
    } else if let Some(plane) = shape.as_plane() {
        return format!("Plane: pos: {:#?}, dir: {:#?}", plane.pos(), plane.dir());
    } else if let Some(cylinder) = shape.as_cylinder() {
        return format!(
            "Cylinder: pos: {:#?}, dir: {:#?}, radius: {}, height: {}",
            cylinder.pos(),
            cylinder.dir(),
            cylinder.radius(),
            cylinder.height()
        );
    } else if let Some(cone) = shape.as_cone() {
        return format!(
            "Cone: pos: {:#?}, dir: {:#?}, radius: {}, height: {}",
            cone.pos(),
            cone.dir(),
            cone.radius(),
            cone.height()
        );
    } else {
        return String::from("Unknown shape");
    }
}
