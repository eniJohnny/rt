use std::{
    path::Path,
    sync::{Arc, RwLock},
};

use chrono::{DateTime, Utc};
use winit::{
    event::WindowEvent,
    event_loop::EventLoopWindowTarget,
    keyboard::{Key, NamedKey},
};

use crate::{
    model::scene::Scene,
    ui::{
        ui::{ui_clicked, UI},
        uisettings::UISettings, utils::ui_utils::Editing,
    },
};

use super::display::blend_scene_and_ui;

pub fn handle_event(
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
            if let Some(property) = ui.get_property_mut(&edit.reference) {
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
            let uibox = ui.get_box_mut(&box_ref);
            if let Some(uibox) = uibox {
                if let Some(edit_bar) = &mut uibox.edit_bar {
                    if let Some(err) = err {
                        edit_bar.text.0 = Some(err);
                    } else {
                        edit_bar.text.0 = None
                    }
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

pub fn key_held(scene: &Arc<RwLock<Scene>>, _: &mut UI, _: &EventLoopWindowTarget<()>, input: Key) {
    match input {
        Key::Named(NamedKey::ArrowDown) => {
            scene.write().unwrap().camera_mut().look_down();
            scene.write().unwrap().set_dirty(true);
        }
        Key::Named(NamedKey::ArrowLeft) => {
            scene.write().unwrap().camera_mut().look_left();
            scene.write().unwrap().set_dirty(true);
        }
        Key::Named(NamedKey::ArrowRight) => {
            scene.write().unwrap().camera_mut().look_right();
            scene.write().unwrap().set_dirty(true);
        }
        Key::Named(NamedKey::ArrowUp) => {
            scene.write().unwrap().camera_mut().look_up();
            scene.write().unwrap().set_dirty(true);
        }
        Key::Named(NamedKey::Shift) => {
            scene.write().unwrap().camera_mut().move_up();
            scene.write().unwrap().set_dirty(true);
        }
        Key::Named(NamedKey::Space) => {
            scene.write().unwrap().camera_mut().move_down();
            scene.write().unwrap().set_dirty(true);
        }
        Key::Character(c) => {
            if c.len() == 1 {
                let c = c.chars().next().unwrap();
                match c {
                    'w' => {
                        scene.write().unwrap().camera_mut().move_forward();
                        scene.write().unwrap().set_dirty(true);
                    }
                    's' => {
                        scene.write().unwrap().camera_mut().move_backward();
                        scene.write().unwrap().set_dirty(true);
                    }
                    'a' => {
                        scene.write().unwrap().camera_mut().move_left();
                        scene.write().unwrap().set_dirty(true);
                    }
                    'd' => {
                        scene.write().unwrap().camera_mut().move_right();
                        scene.write().unwrap().set_dirty(true);
                    }
                    _ => (),
                }
            }
        }
        _ => (),
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
                    blend_scene_and_ui(ui.context().unwrap(), ui.active_box())
                        .save(path)
                        .unwrap();
                }
            }
        }
        _ => (),
    }
}
