use std::{
    path::Path, sync::{Arc, RwLock}, time::{Duration, Instant}
};

use chrono::{DateTime, Utc};
use pixels::Pixels;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    keyboard::{Key, NamedKey},
};

use crate::{
    gui::{draw::{blend_scene_and_ui, redraw_if_necessary}, elements::{
        ui::{ui_clicked, Editing, UI},
        uibox::UIBox,
        Displayable,
    }},
    model::scene::Scene,
    render::render_threads::start_render_threads,
};

use super::display;

pub fn setup_edit_bar(ui: &mut UI, scene: &Arc<RwLock<Scene>>) {
    let mut settings_box = UIBox::default(&ui, "uisettings".to_string());
    settings_box.add_elements(
        ui.uisettings()
            .get_fields(&settings_box.reference, ui.uisettings()),
    );
    settings_box.add_elements(
        scene
            .read()
            .unwrap()
            .settings()
            .get_fields(&settings_box.reference, ui.uisettings()),
    );
    settings_box.set_edit_bar(ui.uisettings(), None);

    let index = ui.add_box(settings_box);
    ui.set_active_box(index);
}

pub fn setup_ui(scene: &Arc<RwLock<Scene>>) -> UI {
    let (ra, tb) = start_render_threads(Arc::clone(&scene));
    tb.send(true).unwrap();
    let mut ui = UI::default(ra, tb);
    setup_edit_bar(&mut ui, scene);
    ui
}

pub fn main_loop(event_loop: EventLoop<()>, scene: Arc<RwLock<Scene>>, mut pixels: Pixels) {
    let mut ui = setup_ui(&scene);
    let mut last_draw = Instant::now();
    let mut last_input = Instant::now();
    let mut last_scene_change = Instant::now();

    event_loop
        .run(move |event, flow| {
            flow.set_control_flow(ControlFlow::WaitUntil(
                Instant::now() + Duration::from_millis(20),
            ));

            // We redraw if the ui is dirty(needs redraw), or we receive a new image from the render
            if last_draw.elapsed().as_millis() > 20 {
                redraw_if_necessary(&mut ui, &scene, &mut pixels);
                last_draw = Instant::now();
            }

            // We handle every held inputs every 20ms. This basically is only used to handle camera movements
            if ui.editing().is_none() && ui.inputs().len() > 0 && last_input.elapsed().as_millis() > 20 {
                let inputs = ui.inputs().clone();
                for input in inputs {
                    key_hold(&scene, &mut ui, flow, input);
                }
                last_input = Instant::now();
            }

            // We are waiting for the render to build a decent image before asking for a new image.
            // If we asked for an image directly after noticing the render of a scene change, we would
            // only ever have low resolution image (the first one rendered).
            // Also, as to not overload the render, we don't ask for redraws too often, and we prefer to
            // keep the scene dirty for a couple loops.
            if last_scene_change.elapsed().as_millis() > 50 {
                let context = ui.context().unwrap();
                if !context.final_img && !context.image_asked{
                    context.transmitter.send(false).unwrap();
                    ui.context_mut().unwrap().image_asked = true;
                }
                // We overlay the previous context, so the compiler drops it when we stop using it (after the transmitter send). This allows us to borrow it mutable the line after.
                let context = ui.context().unwrap();
                if scene.read().unwrap().dirty() {
                    context.transmitter.send(true).unwrap();
                    scene.write().unwrap().set_dirty(false);
                    last_scene_change = Instant::now();
                    ui.context_mut().unwrap().final_img = false;
                }
            }

            match event {
                Event::WindowEvent { event, .. } => {
                    handle_event(event, &scene, &mut ui, flow);
                }
                _ => {}
            }
        })
        .expect("ERROR : Unexpected error when running the event loop");
}

fn handle_event(
    event: WindowEvent,
    scene: &Arc<RwLock<Scene>>,
    ui: &mut UI,
    flow: &EventLoopWindowTarget<()>,
) {
    match event {
        WindowEvent::CursorMoved { position, .. } => {
            ui.set_mouse_position((position.x as u32, position.y as u32))
        }
        WindowEvent::MouseInput { state, .. } => {
            if state == winit::event::ElementState::Released {
                let pos = ui.mouse_position();
                if !ui_clicked(pos, scene, ui) {
                    ui.set_editing(None);
                }
                ui.set_dirty()
            }
        }
        WindowEvent::KeyboardInput { event, .. } => {
            if event.state == winit::event::ElementState::Released {
                ui.input_released(event.logical_key);
            } else if event.state == winit::event::ElementState::Pressed {
                ui.input_pressed(event.logical_key.clone());
                handle_keyboard_press(scene, ui, flow, event.logical_key);
            }
        }
        WindowEvent::CloseRequested => {
            flow.exit();
        }
        _ => {}
    }
}

fn key_pressed_editing(
    _: &Arc<RwLock<Scene>>,
    ui: &mut UI,
    _: &EventLoopWindowTarget<()>,
    input: &Key,
    edit: Editing,
) {
    let mut value = edit.value;
    match input {
        Key::Named(NamedKey::Escape) => {
            ui.set_editing(None);
        }
        Key::Named(NamedKey::Backspace) => {
            value.truncate(value.len() - 1);
            ui.set_editing(Some(Editing {
                reference: edit.reference,
                value,
            }));
        }
        Key::Named(NamedKey::Enter) => {
            let mut err = None;
            if let Some(property) = ui.get_property_by_reference(&edit.reference) {
                match property.get_value_from_string(value.clone()) {
                    Err(error) => {
                        err = Some(error);
                    }
                    Ok(value) => {
                        if let Err(e) = (property.fn_validate)(&value) {
                            err = Some(e.to_string());
                        } else {
                            property.initial_value = property.value.clone();
                            property.value = value;
                        }
                    }
                }
            }
            let tmp_ref = edit.reference.clone();
            let box_ref = tmp_ref.split(".").next().unwrap().to_string();
            let uibox = ui.get_box_mut(box_ref);
            if let Some(edit_bar) = &mut uibox.edit_bar {
                if let Some(err) = err {
                    edit_bar.text.0 = Some(err);
                } else {
                    edit_bar.text.0 = None
                }
            }
            ui.set_editing(None);
        }
        Key::Character(char) => {
            if char.len() == 1 {
                let c = char.chars().next().unwrap();
                if c.is_alphanumeric() || c == '.' {
                    value += &c.to_string();
                    ui.set_editing(Some(Editing {
                        reference: edit.reference,
                        value,
                    }));
                }
            }
        }
        _ => {}
    }
}

fn key_hold(scene: &Arc<RwLock<Scene>>,
    _: &mut UI,
    _: &EventLoopWindowTarget<()>,
    input: Key) {
    let mut scene_mut = scene.write().unwrap();
    let camera = scene_mut.camera_mut();
    match input {
        Key::Named(NamedKey::ArrowDown) => {
            camera.look_down();
            scene_mut.set_dirty(true);
        }
        Key::Named(NamedKey::ArrowLeft) => {
            camera.look_left();
            scene_mut.set_dirty(true);
        }
        Key::Named(NamedKey::ArrowRight) => {
            camera.look_right();
            scene_mut.set_dirty(true);
        }
        Key::Named(NamedKey::ArrowUp) => {
            camera.look_up();
            scene_mut.set_dirty(true);
        }
        Key::Named(NamedKey::Shift) => {
            camera.move_up();
            scene_mut.set_dirty(true);
        }
        Key::Named(NamedKey::Space) => {
            camera.move_down();
            scene_mut.set_dirty(true);
        }
        Key::Character(c) => {
            if c.len() == 1 {
                let c = c.chars().next().unwrap();
                match c {
                    'w' => {
                        camera.move_forward();
                        scene_mut.set_dirty(true);
                    },
                    's' => {
                        camera.move_backward();
                        scene_mut.set_dirty(true);
                    },
                    'a' => {
                        camera.move_left();
                        scene_mut.set_dirty(true);
                    },
                    'd' => {
                        camera.move_right();
                        scene_mut.set_dirty(true);
                    },
                    _ => ()
                }
            }
        }
        _ => ()
    }
}

fn key_pressed_non_editing(
    _: &Arc<RwLock<Scene>>,
    ui: &mut UI,
    flow: &EventLoopWindowTarget<()>,
    input: &Key,
) {
    match input {
        Key::Named(NamedKey::Escape) => {
            flow.exit();
        }
        Key::Character(c) => {
            if c.len() == 1 {
                let c = c.chars().next().unwrap();
                if c == 'p' {
                    // Save a screenshot
                    let date: DateTime<Utc> = Utc::now();
                    let datestring = format!("{}", date.format("%y%m%d_%H%M%S%3f"));
                    if Path::new("screenshots").exists() == false {
                        std::fs::create_dir("screenshots").unwrap();
                    }
                    let path = format!("screenshots/screenshot_{}.png", datestring);
                    blend_scene_and_ui(ui.context().unwrap()).save(path).unwrap();
                }
            }
        }
        _ => ()
    }
}

fn handle_keyboard_press(
    scene: &Arc<RwLock<Scene>>,
    ui: &mut UI,
    flow: &EventLoopWindowTarget<()>,
    input: Key,
) {
    if let Some(edit) = ui.editing().clone() {
        key_pressed_editing(scene, ui, flow, &input, edit);
    } else {
        key_pressed_non_editing(scene, ui, flow, &input);
    }
}

// fn handle_inputs_long_press(scene: &Arc<RwLock<Scene>>, ui: &mut UI, flow: &mut ControlFlow) {}

fn handle_inputs(scene: &Arc<RwLock<Scene>>, ui: &mut UI, flow: &mut ControlFlow) {
    // let inputs = ui.inputs();
    // for input in inputs {
    //     match input {
    //         _ => {}
    //     }
    // }
}
