use super::{display::blend_scene_and_ui, ui_setup::setup_element_ui};
use chrono::{DateTime, Utc};
use std::
    path::Path
;
use winit::{
    event::{MouseScrollDelta, WindowEvent},
    event_loop::EventLoopWindowTarget,
    keyboard::{Key, NamedKey},
};
use crate::{
    render::raycasting::{get_closest_hit, get_lighting_from_ray, get_ray_debug}, ui::{
        ui::{ui_clicked, ui_scrolled, UI}, uielement::UIElement, utils::{misc::{ElemType, Value}, ui_utils::{get_parent_ref, Editing, UIContext}}
    }
};

pub fn handle_event(
    event: WindowEvent,
    context: &mut UIContext,
    ui: &mut UI,
    flow: &EventLoopWindowTarget<()>,
) {
    match event {
        WindowEvent::CursorMoved { position, .. } => {
            ui.set_mouse_position((position.x as u32, position.y as u32))
        }
        WindowEvent::MouseWheel { delta, .. } => {
            if let MouseScrollDelta::LineDelta(_, y) = delta {
                ui_scrolled(ui.mouse_position(), -y, context, ui);
            }
        }
        WindowEvent::MouseInput { state, .. } => {
            if state == winit::event::ElementState::Released {
                let pos = ui.mouse_position();
                if !ui_clicked(pos, context, ui) {
                    if let None = ui.editing() {
                        if ui.active_box_reference() == "" {
                            if let Some(scene) = match context.active_scene {
                                Some(active_scene_index) => Some(context.scene_list.get(&active_scene_index).unwrap()),
                                None => None,
                            } {
                                let scene_read = scene.read().unwrap();
                                let mut ray = get_ray_debug(&scene_read, pos.0 as usize, pos.1 as usize, true);
                                get_lighting_from_ray(&scene_read, &ray);
                                ray.debug = false;
                                if let Some(hit) = get_closest_hit(&scene_read, &ray) {
                                    setup_element_ui(hit.element(), ui, scene);
                                }
                            }
                        }
                    } else {
                        ui.set_editing(None);
                    }
                }
            }
            ui.set_dirty();
        }
        WindowEvent::KeyboardInput { event, .. } => {
            if event.state == winit::event::ElementState::Released {
                ui.input_released(event.logical_key);
            } else if event.state == winit::event::ElementState::Pressed {
                ui.input_pressed(event.logical_key.clone());
                handle_keyboard_press(ui, context, flow, event.logical_key);
            }
        }
        WindowEvent::CloseRequested => {
            flow.exit();
        }
        _ => {}
    }
}

fn handle_keyboard_press(
    ui: &mut UI,
    context: &UIContext,
    flow: &EventLoopWindowTarget<()>,
    input: Key,
) {
    if let Some(edit) = ui.editing().clone() {
        key_pressed_editing(ui, flow, &input, edit);
    } else {
        key_pressed_non_editing(ui, context, &input);
    }
}

fn validate_input(ui: &mut UI, edit_ref: String, value: String) {
    let mut err = None;
    let mut value_to_set: Option<Value> = None;
    if let Some(property) = ui.get_property(&edit_ref) {
        match property.get_value_from_string(value.clone()) {
            Err(error) => {
                err = Some(error);
            }
            Ok(value) => {
                if let Some(elem) = ui.get_element(edit_ref.clone()) {
                    if let Err(e) = (property.fn_validate)(&value, elem, ui) {
                        err = Some(e.to_string());
                    } else {
                        value_to_set = Some(value);
                    }
                }
            }
        }
    }
    if let Some(property) = ui.get_property_mut(&edit_ref) {
        if let Some(value) = value_to_set {
            property.initial_value = property.value.clone();
            property.value = value;
        }
    }
    let tmp_ref = edit_ref.clone();
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

fn get_next_property<'a>(ui: &'a UI, elem: &UIElement) -> Option<&'a UIElement> {
    let parent_ref = get_parent_ref(elem.reference.clone());
    let parent = ui.get_element(parent_ref.clone());
    if let Some(parent) = parent {
        match &parent.elem_type {
            ElemType::Category(cat) => {
                for (i, sibling) in cat.elems.iter().enumerate() {
                    if sibling.reference == elem.reference {
                        if i + 1 < cat.elems.len() {
                            let next_elem = &cat.elems[i + 1];
                            if let ElemType::Property(property) = &next_elem.elem_type {
                                match property.value {
                                    Value::Text(_)
                                    | Value::Unsigned(_)
                                    | Value::Float(_) => { return Some(next_elem); }
                                    _ => {}
                                };
                            }
                        }
                    }
                }
            },
            _ => {}
        }
    }
    None
}

fn edit_next_property(ui: &mut UI, edit_ref: String) {
    let edit  = ui.get_element(edit_ref.clone());
    if let Some(edit) = edit {
        if let Some(next_elem) = get_next_property(ui, edit) {
            if let ElemType::Property(property) = &next_elem.elem_type {
                let value = &property.value;
                if let Value::Bool(_) = value {
                    return ;
                } else {
                    ui.set_editing(Some(Editing {
                        reference: next_elem.reference.clone(),
                        value: value.to_string(),
                    }));
                }
            }
        }
    }
}

fn key_pressed_editing(
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
            if value.len() > 0 {
                value.truncate(value.len() - 1);
                ui.set_editing(Some(Editing {
                    reference: edit.reference,
                    value,
                }));
            }
        }
        Key::Named(NamedKey::Enter) => {
            validate_input(ui, edit.reference, value);
        }
        Key::Named(NamedKey::Tab) => {
            validate_input(ui, edit.reference.clone(), value);
            edit_next_property(ui, edit.reference);
        }
        Key::Character(char) => {
            if char.len() == 1 {
                let c = char.chars().next().unwrap();
                if c.is_alphanumeric() || c == '.' || c == '-' {
                    if value.len() <= 15 {
                        value += &c.to_string();
                        ui.set_editing(Some(Editing {
                            reference: edit.reference,
                            value,
                        }));
                    }
                }
            }
        }
        _ => {}
    }
}

pub fn key_held(context: &UIContext, _: &mut UI, _: &EventLoopWindowTarget<()>, input: Key) {
    if context.active_scene.is_none() {
        return;
    }
    let scene = context.scene_list.get(&context.active_scene.unwrap()).unwrap();
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
    ui: &mut UI,
    context: &UIContext,
    input: &Key,
) {
    match input {
        Key::Named(NamedKey::Escape) => {
            if ui.active_box().is_some() {
                ui.destroy_box(&ui.active_box_reference().clone());
                ui.set_dirty();
            } else {
                ui.destroy_last_box();
                ui.set_dirty();
            }
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
                    blend_scene_and_ui(context, ui.active_box())
                        .save(path)
                        .unwrap();
                }
            }
        }
        _ => (),
    }
}
