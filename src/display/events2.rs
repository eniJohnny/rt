use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use pixels::Pixels;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    keyboard::{Key, NamedKey},
};

use crate::{
    gui::elements::{
        ui::{ui_clicked, Editing, UI},
        uibox::UIBox,
        Displayable,
    },
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

    event_loop
        .run(move |event, window_target| {
            window_target.set_control_flow(ControlFlow::WaitUntil(
                Instant::now() + Duration::from_millis(20),
            ));

            if last_draw.elapsed().as_millis() > 20 {
                redraw_if_necessary(&mut ui, &scene, &mut pixels);
                last_draw = Instant::now();
            }
            if scene.read().unwrap().dirty() {
                let context = ui.take_context();
                context.transmitter.send(true).unwrap();
                ui.give_back_context(context);
                scene.write().unwrap().set_dirty(false);
            }

            match event {
                Event::WindowEvent { event, .. } => {
                    handle_event(event, &scene, &mut ui, window_target);
                }
                _ => {}
            }
        })
        .expect("ERROR : Unexpected error when running the event loop");
}

fn redraw_if_necessary(ui: &mut UI, scene: &Arc<RwLock<Scene>>, mut pixels: &mut Pixels) {
    if ui.dirty() {
        ui.process(&scene);
    }
    let mut context = ui.take_context();
    let ui_img = &mut context.ui_img;
    let mut redraw = false;
    if ui.dirty() {
        ui.draw(&scene, ui_img);
        redraw = true;
    }
    if let Ok((render_img, final_img)) = context.receiver.try_recv() {
        context.scene_img = render_img;
        if !final_img {
            context.transmitter.send(false).unwrap();
        }
        redraw = true;
    }
    if redraw {
        let time = Instant::now();
        let mut image = ui_img.clone();
        for i in image.enumerate_pixels_mut() {
            if i.2 .0 == [1; 4] {
                i.2 .0 = context.scene_img.get_pixel(i.0, i.1).0
            }
        }
        display(&mut pixels, &mut image);
        let nb_samples = context.draw_time_samples as f64;
        context.draw_time_avg = nb_samples * context.draw_time_avg / (nb_samples + 1.)
            + time.elapsed().as_millis() as f64 / (nb_samples + 1.);
        context.draw_time_samples += 1;
    }
    ui.give_back_context(context);
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
                // ui.input_pressed(keycode);
            } else if event.state == winit::event::ElementState::Pressed {
                handle_keyboard_press(scene, ui, flow, event.logical_key);
                // ui.input_released(keycode);
            }
            // handle_inputs_long_press(scene, ui, flow);
        }
        WindowEvent::CloseRequested => {
            // Close the window
            flow.exit();
        }
        _ => {}
    }
}

fn key_pressed_editing(
    scene: &Arc<RwLock<Scene>>,
    ui: &mut UI,
    flow: &EventLoopWindowTarget<()>,
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

fn key_pressed_non_editing(
    scene: &Arc<RwLock<Scene>>,
    ui: &mut UI,
    flow: &EventLoopWindowTarget<()>,
    input: &Key,
) {
}

fn handle_keyboard_press(
    scene: &Arc<RwLock<Scene>>,
    ui: &mut UI,
    flow: &EventLoopWindowTarget<()>,
    input: Key,
) {
    if let Some(edit) = ui.editing().clone() {}
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
