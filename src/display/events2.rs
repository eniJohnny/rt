use std::{
    path::Path,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, RwLock,
    },
    time::{Duration, Instant},
};

use chrono::{DateTime, Utc};
use image::{ImageBuffer, Rgba, RgbaImage};
use pixels::Pixels;
use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::{
    gui::elements::{
        ui::{Editing, UI},
        uibox::UIBox,
        uielement::ElemType,
        Displayable,
    },
    model::scene::Scene,
    SCREEN_HEIGHT_U32, SCREEN_WIDTH_U32,
};

use super::display;

pub fn main_loop(event_loop: EventLoop<()>, scene: Arc<RwLock<Scene>>, mut pixels: Pixels<Window>) {
    let mut ui = UI::default();
    let mut settings_box = UIBox::default(&ui, "uisettings".to_string());
    settings_box.set_edit_bar(ui.settings());
    let mut img = RgbaImage::new(SCREEN_WIDTH_U32, SCREEN_HEIGHT_U32);

    settings_box.add_elements(
        ui.settings()
            .get_fields(&settings_box.reference, ui.settings()),
    );

    let index = ui.add_box(settings_box);
    ui.set_active_box(index);

    event_loop.run(move |event, _, control_flow: &mut ControlFlow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(20));

        if ui.dirty() {
            ui.draw(&scene, &mut img);
            display(&mut pixels, &mut img);
        }

        match event {
            Event::WindowEvent { window_id, event } => {
                handle_event(event, &scene, &mut ui, control_flow);
            }
            _ => {}
        }
    })
}

fn handle_event(
    event: WindowEvent,
    scene: &Arc<RwLock<Scene>>,
    ui: &mut UI,
    flow: &mut ControlFlow,
) {
    match event {
        WindowEvent::CursorMoved { position, .. } => {
            ui.set_mouse_position((position.x as u32, position.y as u32))
        }
        WindowEvent::MouseInput { state, .. } => {
            if state == winit::event::ElementState::Released {
                let pos = ui.mouse_position();
                if let Some(elem) = ui.clicked(pos) {
                    if let Some(edit) = ui.editing() {
                        if &edit.reference != &elem.reference {
                            if let ElemType::Property(property) = &elem.elem_type {
                                ui.set_editing(Some(Editing {
                                    reference: elem.reference.clone(),
                                    value: property.value.to_string(),
                                }));
                            }
                        }
                    } else {
                        if let ElemType::Property(property) = &elem.elem_type {
                            ui.set_editing(Some(Editing {
                                reference: elem.reference.clone(),
                                value: property.value.to_string(),
                            }));
                        }
                    }
                } else if ui.editing().is_some() {
                    // Drops current value if clicked outside
                    ui.set_editing(None);
                }
            }
        }
        WindowEvent::KeyboardInput { input, .. } => {
            if let Some(keycode) = input.virtual_keycode {
                if input.state == winit::event::ElementState::Released {
                    // ui.input_pressed(keycode);
                } else if input.state == winit::event::ElementState::Pressed {
                    handle_keyboard_press(scene, ui, flow, keycode);
                    // ui.input_released(keycode);
                }
            }
            handle_inputs_long_press(scene, ui, flow);
        }
        WindowEvent::CloseRequested => {
            // Close the window
            *flow = ControlFlow::Exit;
        }
        _ => {}
    }
}

fn handle_keyboard_press(
    scene: &Arc<RwLock<Scene>>,
    ui: &mut UI,
    flow: &mut ControlFlow,
    input: VirtualKeyCode,
) {
    if let Some(edit) = ui.editing().clone() {
        let mut value = edit.value;
        match input {
            num if (num >= VirtualKeyCode::Numpad0 && num <= VirtualKeyCode::Numpad9) => {
                let num = num as u8 - 80;
                value += &num.to_string();
                ui.set_editing(Some(Editing {
                    reference: edit.reference,
                    value,
                }));
            }
            num if num >= VirtualKeyCode::Key1 && num <= VirtualKeyCode::Key9 => {
                value += &(num as u8 + 1).to_string();
                ui.set_editing(Some(Editing {
                    reference: edit.reference,
                    value,
                }));
            }
            VirtualKeyCode::Key0 => {
                value += "0";
                ui.set_editing(Some(Editing {
                    reference: edit.reference,
                    value,
                }));
            }
            c if (c >= VirtualKeyCode::A && c <= VirtualKeyCode::Z) => {
                let char_u8 = (c as u32) + 87;
                let ch = char::from_u32(char_u8).expect("Not a valid char");
                value.push(ch);
                ui.set_editing(Some(Editing {
                    reference: edit.reference,
                    value,
                }));
            }
            VirtualKeyCode::Escape => {
                ui.set_editing(None);
            }
            VirtualKeyCode::Back => {
                value.truncate(value.len() - 1);
                ui.set_editing(Some(Editing {
                    reference: edit.reference,
                    value,
                }));
            }
            VirtualKeyCode::NumpadEnter | VirtualKeyCode::Return => {
                if let Some(property) = ui.get_property_by_reference(&edit.reference) {
                    property.set_value_from_string(value);
                }
                ui.set_editing(None);
            }
            _ => {}
        }
    }
    match input {
        VirtualKeyCode::Escape => {
            if ui.active_box().is_none() {
                ui.set_active_box("".to_string());
            } else {
                *flow = ControlFlow::Exit;
            }
        }
        _ => {}
    }
}

fn handle_inputs_long_press(scene: &Arc<RwLock<Scene>>, ui: &mut UI, flow: &mut ControlFlow) {}

fn handle_inputs(scene: &Arc<RwLock<Scene>>, ui: &mut UI, flow: &mut ControlFlow) {
    let inputs = ui.inputs();
    for input in inputs {
        match input {
            _ => {}
        }
    }
}
