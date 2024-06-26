extern crate image;
extern crate pixels;
extern crate winit;

use crate::{
    gui::{
        draw::draw_gui, uisettings::UISettings, textformat::TextFormat, utils::{gui_clicked, hide_gui, hitbox_contains}, Gui
    },
    model::{
        materials::{
            color::Color,
            texture::{Texture, TextureType},
        },
        maths::{vec2::Vec2, vec3::Vec3},
        scene::Scene,
    },
    render::{
        lighting_real::get_lighting_from_hit,
        raycasting::{get_closest_hit, get_ray, get_ray_debug, sampling_ray},
    },
    CAM_MOVE_KEYS, FPS, RGB_KEYS,
};
use chrono::{DateTime, Utc};
use image::{ImageBuffer, Rgba, RgbaImage};
use std::{
    path::Path,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, RwLock,
    },
    thread::{self, sleep},
    time::{Duration, Instant},
};

use pixels::Pixels;
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use super::update::{update_color, update_shape};
use super::utils::{display_element_infos, draw_text, get_shape, move_camera};

use super::display;

pub fn event_manager(
    event_loop: EventLoop<()>,
    scene: Arc<RwLock<Scene>>,
    mut img: RgbaImage,
    mut pixels: Pixels<Window>,
    ra: Receiver<(ImageBuffer<Rgba<u8>, Vec<u8>>, bool)>,
    tb: Sender<bool>,
) {
    let mut scene_change = false;
    let mut image_requested = true;
    let mut final_image = false;
    let settings = UISettings::default();
    let format = TextFormat::new_base_format(&settings);
    let editing_format = TextFormat::new_editing_format(&settings);
    let mut full_img: RgbaImage = img.clone();
    let mut gui = Gui::new();

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

            let mut mut_scene = scene.write().unwrap();
            if gui.is_open() {
                match gui.displaying().as_str() {
                    "element" => {
                        gui = draw_gui(
                            &mut img,
                            Some(&mut_scene.elements()[gui.element_index()]),
                            None,
                            gui.element_index(),
                        );
                    }
                    "light" => {
                        let index = (gui.light_index() + 1) as usize % mut_scene.lights().len();
                        gui = draw_gui(&mut img, None, Some(&mut_scene.lights()[index]), index);
                    }
                    _ => {}
                }
            }

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
                        let mut ray = get_ray_debug(&scene, x as usize, y as usize, true);
                        let hit = get_closest_hit(&scene, &ray);
                        if let Some(hit) = &hit {
                            //For debug purposes
                            get_lighting_from_hit(&scene, hit, &ray);
                            let proj = hit.element().shape().projection(hit);
                            dbg!(proj);
                        }

                        if gui_clicked(mouse_position, &gui) {
                            // If the GUI is clicked

                            let mut editing = false;

                            if hitbox_contains(gui.cancel_hitbox(), mouse_position) {
                                // Close GUI
                                hide_gui(&mut img, &full_img);
                                gui = Gui::new();
                                display(&mut pixels, &mut img);
                            } else if hitbox_contains(gui.apply_hitbox(), mouse_position) {
                                // Apply changes for every key
                                for i in 0..gui.keys().len() {
                                    let key = gui.keys()[i].clone();
                                    let value = gui.values()[i].clone().replace("_", "");
                                    let element_index = gui.element_index();
                                    let elem = &scene.elements()[element_index];
                                    let shape = elem.shape();

                                    if RGB_KEYS.contains(&key.as_str()) {
                                        update_color(
                                            key,
                                            value,
                                            scene.elements_as_mut()[element_index].material_mut(),
                                        );
                                    } else if key == "metalness" {
                                        let metalness = value.parse::<f64>().unwrap();
                                        scene.elements_as_mut()[element_index]
                                            .material_mut()
                                            .set_metalness(Texture::Value(
                                                Vec3::new(metalness, metalness, metalness),
                                                TextureType::Vector,
                                            ));
                                    } else if key == "roughness" {
                                        let roughness = value.parse::<f64>().unwrap();
                                        scene.elements_as_mut()[element_index]
                                            .material_mut()
                                            .set_roughness(Texture::Value(
                                                Vec3::new(roughness, roughness, roughness),
                                                TextureType::Vector,
                                            ));
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
                                let index = gui.updating_index();
                                let value = gui.values()[index].clone().replace("_", "");
                                let hitbox = gui.hitboxes()[index].clone();
                                let pos = Vec2::new(*hitbox.0.x() as f64, *hitbox.0.y() as f64);
                                let background_pos =
                                    Vec2::new(*hitbox.0.x() as f64 - 10., *hitbox.0.y() as f64);

                                let text = format!("{}", value);
                                draw_text(&mut img, &background_pos, " ".to_string(), &format);
                                draw_text(&mut img, &pos, text, &format);
                                display(&mut pixels, &mut img);
                            }
                            if gui.keys().len() > 0 {
                                for i in 0..gui.keys().len() {
                                    let hitbox = gui.hitboxes()[i].clone();
                                    if hitbox_contains(&hitbox, mouse_position) {
                                        // Reset previous value formatting
                                        if gui.updating() {
                                            let index = gui.updating_index();
                                            let value =
                                                gui.values()[index].clone().replace("_", "");
                                            let hitbox = gui.hitboxes()[index].clone();
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
                                        gui.set_updating(true);
                                        gui.set_updating_index(i);
                                        editing = true;
                                    }
                                }
                                if editing == false {
                                    let index = gui.updating_index();
                                    let value = gui.values()[index].clone().replace("_", "");
                                    let hitbox = gui.hitboxes()[index].clone();
                                    let pos =
                                        Vec2::new(*hitbox.0.x() as f64 - 10., *hitbox.0.y() as f64);

                                    let text = format!("{}", value);
                                    draw_text(&mut img, &pos, text, &format);
                                    gui.set_updating(false);
                                }
                            }

                            if gui.updating() {
                                let index = gui.updating_index();
                                let value = gui.values()[index].clone().replace("_", "");
                                let hitbox = gui.hitboxes()[index].clone();
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

                            if gui.keys().len() == 0 {
                                full_img = img.clone();
                            }

                            gui = display_element_infos(element, &mut img);
                            gui.set_element_index(element_index);
                            display(&mut pixels, &mut img);
                        } else {
                            hide_gui(&mut img, &full_img);
                            gui = Gui::new();
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
                        if gui.is_open() {
                            hide_gui(&mut img, &full_img);
                            gui = Gui::new();
                            display(&mut pixels, &mut img);
                        } else {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    Some(VirtualKeyCode::P) => {
                        // Save a screenshot
                        let date: DateTime<Utc> = Utc::now();
                        // let datestring = format!("{}", date.format("%Y-%m-%d %H:%M:%S"));
                        let datestring = format!("{}", date.format("%y%m%d_%H%M%S%3f"));
                        if Path::new("screenshots").exists() == false {
                            std::fs::create_dir("screenshots").unwrap();
                        }
                        let path = format!("screenshots/screenshot_{}.png", datestring);
                        img.save(path).unwrap();
                    }
                    c if (c >= Some(VirtualKeyCode::Numpad0)
                        && c <= Some(VirtualKeyCode::Numpad9))
                        || (c >= Some(VirtualKeyCode::Key1) && c <= Some(VirtualKeyCode::Key0)) =>
                    {
                        if gui.updating() == false {
                            return;
                        }

                        // Add c to the edited value
                        let index = gui.updating_index();
                        let hitbox = gui.hitboxes()[index].clone();
                        let new_hitbox = (
                            Vec2::new(*hitbox.0.x() - 10., *hitbox.0.y()),
                            hitbox.1.clone(),
                        );
                        let pos = new_hitbox.0.clone();

                        // -10 px for the _ character
                        let pos = Vec2::new(*pos.x() as f64 - 10., *pos.y() as f64);
                        let value = gui.values()[index].clone().replace("_", "");
                        let mut number = c.unwrap() as u8;
                        if c.unwrap() >= VirtualKeyCode::Key1 && c.unwrap() <= VirtualKeyCode::Key9
                        {
                            number = number + 1;
                        } else if c.unwrap() == VirtualKeyCode::Key0 {
                            number = 0;
                        } else {
                            number = number - VirtualKeyCode::Numpad0 as u8;
                        }

                        let value = format!("{}{:?}_", value, number);

                        gui.set_updates(index, &value, &new_hitbox);

                        draw_text(&mut img, &pos, value, &editing_format);
                        display(&mut pixels, &mut img);
                        sleep(Duration::from_millis(50));
                    }
                    Some(VirtualKeyCode::Back) => {
                        // Remove last character from the edited value
                        let index = gui.updating_index();
                        let hitbox = gui.hitboxes()[index].clone();
                        let new_hitbox = (
                            Vec2::new(*hitbox.0.x() + 10., *hitbox.0.y()),
                            hitbox.1.clone(),
                        );
                        let pos = new_hitbox.0.clone();
                        // -10 for the _
                        let pos = Vec2::new(*pos.x() as f64 - 10., *pos.y() as f64);
                        let background_pos =
                            Vec2::new(*hitbox.0.x() as f64 - 10., *hitbox.0.y() as f64);
                        let value = gui.values()[index].clone().replace("_", "");

                        if value.len() > 0 {
                            let value = value.chars().take(value.len() - 1).collect::<String>();
                            let value = format!("{}_", value);

                            gui.set_updates(index, &value, &new_hitbox);

                            draw_text(&mut img, &background_pos, " ".to_string(), &format);
                            draw_text(&mut img, &pos, value, &editing_format);
                            display(&mut pixels, &mut img);
                        }
                    }
                    Some(VirtualKeyCode::NumpadDecimal) | Some(VirtualKeyCode::Period) => {
                        // Add a comma to the edited value
                        let index = gui.updating_index();
                        let hitbox = gui.hitboxes()[index].clone();
                        let new_hitbox = (
                            Vec2::new(*hitbox.0.x() - 10., *hitbox.0.y()),
                            hitbox.1.clone(),
                        );
                        let pos = new_hitbox.0.clone();
                        // -10 for the _
                        let pos = Vec2::new(*pos.x() as f64 - 10., *pos.y() as f64);
                        let mut value = gui.values()[index].clone().replace("_", "");

                        if value.contains(".") {
                            return;
                        }

                        if value.len() == 0 {
                            value = "0._".to_string();
                        } else {
                            value = format!("{}._", value);
                        }
                        gui.set_updates(index, &value, &new_hitbox);

                        draw_text(&mut img, &pos, value, &editing_format);
                        display(&mut pixels, &mut img);
                    }
                    Some(VirtualKeyCode::NumpadSubtract) | Some(VirtualKeyCode::Minus) => {
                        // Add a minus to the edited value
                        let index = gui.updating_index();
                        let hitbox = gui.hitboxes()[index].clone();
                        let mut offset = -10.;
                        if gui.values()[index].contains("-") {
                            offset = 10.;
                        }
                        let new_hitbox = (
                            Vec2::new(*hitbox.0.x() + offset, *hitbox.0.y()),
                            hitbox.1.clone(),
                        );
                        let pos = new_hitbox.0.clone();
                        // -10 for the _
                        let pos = Vec2::new(*pos.x() as f64 - 10., *pos.y() as f64);
                        let mut value = gui.values()[index].clone().replace("_", "");

                        value.push('_');
                        if value.contains("-") {
                            value = value.replace("-", "");
                        } else {
                            value = format!("-{}", value);
                        }

                        gui.set_updates(index, &value, &new_hitbox);

                        draw_text(&mut img, &pos, value, &editing_format);
                        display(&mut pixels, &mut img);
                    }
                    Some(VirtualKeyCode::L) => {
                        let lights = scene.lights();
                        let light_index = (gui.light_index() + 1) as usize % lights.len();

                        gui = draw_gui(&mut img, None, Some(&lights[light_index]), light_index);
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
                let mut mut_scene = scene.write().unwrap();
                if gui.is_open() {
                    if gui.displaying() == "element" {
                        gui = draw_gui(
                            &mut img,
                            Some(&mut_scene.elements()[gui.element_index()]),
                            None,
                            gui.element_index(),
                        );
                    } else if gui.displaying() == "light" {
                        let index = gui.light_index() as usize;
                        gui = draw_gui(&mut img, None, Some(&mut_scene.lights()[index]), index);
                    }
                }
                if gui.keys().len() == 0 {
                    full_img = img.clone();
                }
                display(&mut pixels, &mut img);
                final_image = final_img;
                image_requested = false;

                // gui = Gui::new();
            }
        } else if !final_image {
            tb.send(false).unwrap();
            image_requested = true;
        }
    });
}
