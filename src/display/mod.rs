extern crate image;
extern crate pixels;
extern crate winit;

use image::{ImageBuffer, Rgba, RgbaImage};
use rusttype::{Font, Scale};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use crate::{
    gui::{draw_sphere_gui, hide_gui, TextFormat, gui_clicked, hitbox_contains, Gui},
    parsing::get_scene,
    model::{
        maths::{vec2::Vec2, vec3::Vec3},
        shapes::{sphere, Shape},
        Element
    },
    render::raycasting::{get_closest_hit, render_scene_threadpool},
    SCREEN_HEIGHT,
    SCREEN_WIDTH
};

pub fn display_scene() {
    // Load scene
    let mut scene = get_scene();
    let format = TextFormat::new(Vec2::new(400., 400.), 24., Rgba([255, 255, 255, 255]), Rgba([89, 89, 89, 255]));
    let editing_format = TextFormat::new(Vec2::new(400., 400.), 24., Rgba([0, 0, 0, 255]), Rgba([255, 255, 255, 255]));

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

    let mut img = render_scene_threadpool(&scene);

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

                        let rays = scene.camera().rays();
                        let ray = &rays[x as usize][y as usize];
                        let hit = get_closest_hit(&scene, ray);
                        if hit.is_some() {
                            let hit = hit.unwrap();
                            let element = hit.element();

                            let element_index: usize = scene.elements().iter().position(|e| {
                                let e_shape = e.shape();
                                let element_shape = element.shape();
                                get_shape(e_shape) == get_shape(element_shape)
                            }).unwrap() as usize;

                            scene.gui = display_element_infos(element, &mut img);
                            scene.gui.set_element_index(element_index);

                            display(&mut pixels, &mut img);
                        } else if gui_clicked(mouse_position, &scene.gui) {
                            // If the GUI is clicked
                            
                            let mut editing = false;
                            
                            if scene.gui.keys().len() > 0 {
                                for i in 0..scene.gui.keys().len() {
                                    let hitbox = scene.gui.hitboxes()[i].clone();
                                    if hitbox_contains(&hitbox, mouse_position) {
                                        // Reset previous value formatting
                                        if scene.gui.updating() {
                                            let index = scene.gui.updating_index();
                                            let value = scene.gui.values()[index].clone().replace("_", "");
                                            let hitbox = scene.gui.hitboxes()[index].clone();
                                            let pos = Vec2::new(*hitbox.0.x() as f64, *hitbox.0.y() as f64);
                                            let background_pos = Vec2::new(*hitbox.0.x() as f64 - 10., *hitbox.0.y() as f64);

                                            let text = format!("{}", value);
                                            draw_text(&mut img, &background_pos, " ".to_string(), &format);
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
                                    let pos = Vec2::new(*hitbox.0.x() as f64 - 10., *hitbox.0.y() as f64);

                                    let text = format!("{}", value);
                                    draw_text(&mut img, &pos, text, &format);
                                    scene.gui.set_updating(false);
                                }
                            }
                            
                            if scene.gui.updating() {
                                let index = scene.gui.updating_index();
                                let value = scene.gui.values()[index].clone().replace("_", "");
                                let hitbox = scene.gui.hitboxes()[index].clone();
                                let pos = Vec2::new(*hitbox.0.x() as f64 - 10., *hitbox.0.y() as f64);

                                let text = format!("{}_", value);
                                draw_text(&mut img, &pos, text, &editing_format);
                                display(&mut pixels, &mut img);
                            }

                        } else {
                            hide_gui(&mut img, &scene);
                            scene.gui = Gui::new();
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
                            x if x >= Some(VirtualKeyCode::Numpad0) && x <= Some(VirtualKeyCode::Numpad9) => {
                                // Add x to the edited value
                                let index = scene.gui.updating_index();
                                let hitbox = scene.gui.hitboxes()[index].clone();
                                let new_hitbox = (Vec2::new(*hitbox.0.x() - 10., *hitbox.0.y()), hitbox.1.clone());
                                let pos = new_hitbox.0.clone();
                                // -10 for the _
                                let pos = Vec2::new(*pos.x() as f64 - 10., *pos.y() as f64);
                                let value = scene.gui.values()[index].clone().replace("_", "");
                                let number = x.unwrap() as u8 - VirtualKeyCode::Numpad0 as u8;

                                let value = format!("{}{:?}_", value, number);

                                scene.gui.set_updates(index, &value, &new_hitbox);

                                draw_text(&mut img, &pos, value, &editing_format);
                                display(&mut pixels, &mut img);
                            }
                            Some(VirtualKeyCode::Back) => {
                                // Remove last character from the edited value
                                let index = scene.gui.updating_index();
                                let hitbox = scene.gui.hitboxes()[index].clone();
                                let new_hitbox = (Vec2::new(*hitbox.0.x() + 10., *hitbox.0.y()), hitbox.1.clone());
                                let pos = new_hitbox.0.clone();
                                // -10 for the _
                                let pos = Vec2::new(*pos.x() as f64 - 10., *pos.y() as f64);
                                let background_pos = Vec2::new(*hitbox.0.x() as f64 - 10., *hitbox.0.y() as f64);
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
                            Some(VirtualKeyCode::NumpadDecimal) => {
                                // Add a comma to the edited value
                                let index = scene.gui.updating_index();
                                let hitbox = scene.gui.hitboxes()[index].clone();
                                let new_hitbox = (Vec2::new(*hitbox.0.x() - 10., *hitbox.0.y()), hitbox.1.clone());
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
                                let new_hitbox = (Vec2::new(*hitbox.0.x() + offset, *hitbox.0.y()), hitbox.1.clone());
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
                            Some(VirtualKeyCode::NumpadEnter) => {
                                // Exit editing mode and update the scene
                                let index = scene.gui.updating_index();
                                let element_index = scene.gui.element_index();
                                let value = scene.gui.values()[index].clone().replace("_", "");

                                scene.gui.set_updating(false);
                                let test = &scene.elements()[element_index];
                                let shape = test.shape();

                                if shape.as_sphere().is_some() {
                                    let sphere = shape.as_sphere().unwrap();
                                    let key = scene.gui.keys()[index].clone();

                                    let mut pos = sphere.pos().clone();
                                    let mut radius = sphere.radius();
                                    let dir = sphere.dir().clone();
                                    
                                    match key.as_str() {
                                        "posx" => {
                                            pos = Vec3::new(value.parse::<f64>().unwrap(), *pos.y(), *pos.z());
                                        }
                                        "posy" => {
                                            pos = Vec3::new(*pos.x(), value.parse::<f64>().unwrap(), *pos.z());
                                        }
                                        "posz" => {
                                            pos = Vec3::new(*pos.x(), *pos.y(), value.parse::<f64>().unwrap());
                                        }
                                        "radius" => {
                                            radius = value.parse::<f64>().unwrap();
                                        }
                                        _ => (),
                                    }
                                    let sphere = sphere::Sphere::new(pos, dir, radius);
                                    let sphere_for_gui = sphere.clone();
                                    
                                    scene.elements_as_mut()[element_index].set_shape(Box::new(sphere));
                                    
                                    img = render_scene_threadpool(&scene);
                                    draw_sphere_gui(&mut img, &sphere_for_gui);
                                    display(&mut pixels, &mut img);
                                }
                            }
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
            Event::RedrawRequested(_) => {
                // pixels.render().unwrap();
            }
            _ => (),
        }
    });
}

fn display (pixels: &mut Pixels<Window>, img: &mut RgbaImage) {

    // Copy image data to pixels buffer
    pixels.get_frame().copy_from_slice(&img);

    // Render the pixels buffer
    pixels.render().unwrap();
}

fn display_element_infos(element: &Element, img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Gui {
    let img = img;
    let shape = element.shape();

    if shape.as_sphere().is_some() {
        let sphere = shape.as_sphere().unwrap();
        return draw_sphere_gui(img, sphere);
    } else {
        return Gui::new();
    }
    // else if shape.as_plane().is_some() {
    //     let plane = shape.as_plane().unwrap();
    //     let pos = plane.pos();
    //     let dir = plane.dir();

    //     println!("Plane: pos: {:?}, dir: {:?}", pos, dir);
    // } else if shape.as_cylinder().is_some() {
    //     let cylinder = shape.as_cylinder().unwrap();
    //     let pos = cylinder.pos();
    //     let dir = cylinder.dir();
    //     let radius = cylinder.radius();
    //     let height = cylinder.height();

    //     println!("Cylinder: pos: {:?}, dir: {:?}, radius: {}, height: {}", pos, dir, radius, height);
    // } else if shape.as_cone().is_some() {
    //     let cone = shape.as_cone().unwrap();
    //     let pos = cone.pos();
    //     let dir = cone.dir();
    //     let radius = cone.radius();
    //     let height = cone.height();

    //     println!("Cone: pos: {:?}, dir: {:?}, radius: {}, height: {}", pos, dir, radius, height);
    // }
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

fn get_shape(shape: &dyn Shape) -> String {
    if let Some(sphere) = shape.as_sphere() {
        return format!("Sphere: pos: {:#?}, radius: {}", sphere.pos(), sphere.radius());
    } else if let Some(plane) = shape.as_plane() {
        return format!("Plane: pos: {:#?}, dir: {:#?}", plane.pos(), plane.dir());
    } else if let Some(cylinder) = shape.as_cylinder() {
        return format!("Cylinder: pos: {:#?}, dir: {:#?}, radius: {}, height: {}", cylinder.pos(), cylinder.dir(), cylinder.radius(), cylinder.height());
    } else if let Some(cone) = shape.as_cone() {
        return format!("Cone: pos: {:#?}, dir: {:#?}, radius: {}, height: {}", cone.pos(), cone.dir(), cone.radius(), cone.height());
    } else {
        return String::from("Unknown shape");
    }
}