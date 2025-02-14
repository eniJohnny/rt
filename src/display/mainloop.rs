use super::{
    display::redraw_if_necessary,
    events::{handle_event, key_held}, ui_setup::setup_ui,
};
use pixels::Pixels;
use std::{
    sync::{Arc, RwLock}, time::{Duration, Instant}
};
use winit::{
    dpi::PhysicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use crate::{
    error, parsing::get_scene, render::render_thread::UIOrder, ui::{ui::UI, ui_setup::scene_ui::{add_scene_to_ui, change_scene}, utils::ui_utils::UIContext}, DISPLAY_WIREFRAME, SCREEN_HEIGHT, SCREEN_HEIGHT_U32, SCREEN_WIDTH, SCREEN_WIDTH_U32, SKYBOX_TEXTURE
};

pub fn load_scene(scene_path: &str, context: &mut UIContext, ui: &mut UI) {
    let path = String::from(scene_path);
    let scene = get_scene(&path);
    if let Err(err) = scene {
        error(format!("Error loading scene : {}", err).as_str());
        return ;
    }
    let mut scene = scene.unwrap();
    scene.load_texture(SKYBOX_TEXTURE, None);
    
    if DISPLAY_WIREFRAME {
        scene.add_wireframes();
    }
    scene.update_bvh();
    scene.determine_full_bvh_traversal();
    let scene = Arc::new(RwLock::new(scene));
    context.transmitter.send(UIOrder::NewScene(scene.clone())).unwrap();
    context.scene_list.insert(context.next_scene_id, scene);
    context.transmitter.send(UIOrder::AskImage(context.next_scene_id)).unwrap();
    context.image_asked = true;
    add_scene_to_ui(ui, context, context.next_scene_id, scene_path);
    change_scene(context, ui, Some(context.next_scene_id), None);
    context.next_scene_id += 1;
}

pub fn start_ui() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32))
        .with_title("RT")
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let pixels = {
        let texture = pixels::SurfaceTexture::new(SCREEN_WIDTH_U32, SCREEN_HEIGHT_U32, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, texture).unwrap()
    };
    main_loop(event_loop, pixels);
}

pub fn main_loop(event_loop: EventLoop<()>, mut pixels: Pixels) {
    let (mut ui, mut context) = setup_ui();
    let mut last_draw = Instant::now();
    let mut last_input = Instant::now();
    let mut last_scene_change = Instant::now();

    for argument in std::env::args().skip(1) {
        load_scene(argument.as_str(), &mut context, &mut ui);
    }

    event_loop
        .run(move |event, flow| {
            flow.set_control_flow(ControlFlow::WaitUntil(
                Instant::now() + Duration::from_millis(20),
            ));

            // We redraw if the ui is dirty(needs redraw), or we receive a new image from the render
            if last_draw.elapsed().as_millis() > 20 {
                redraw_if_necessary(&mut ui, &mut context, &mut pixels);
                last_draw = Instant::now();
            }

            // We handle every held inputs every 20ms. This basically is only used to handle camera movements
            if ui.editing().is_none()
                && ui.inputs().len() > 0
                && last_input.elapsed().as_millis() > 10
            {
                let inputs = ui.inputs().clone();
                for input in inputs {
                    key_held(&context, &mut ui, flow, input);
                }
                last_input = Instant::now();
            }

            // We are waiting for the render to build a decent image before asking for a new image.
            // If we asked for an image directly after noticing the render of a scene change, we would
            // only ever have low resolution image (the first one rendered).
            // Also, as to not overload the render, we don't ask for redraws too often, and we prefer to
            // keep the scene dirty for a couple loops.
            if last_scene_change.elapsed().as_millis() > 50 {
                if let Some(active_scene_index) = context.active_scene {
                    if !context.final_img && !context.image_asked {
                        // println!("UI ASK");
                        context.transmitter.send(UIOrder::AskImage(active_scene_index)).unwrap();
                        context.image_asked = true;
                    }
                    // We overlay the previous context, so the compiler drops it when we stop using it (after the transmitter send). This allows us to borrow it mutable the line after.wrap();
                    let scene = context.scene_list.get(&active_scene_index).unwrap();
                    if scene.read().unwrap().dirty() {
                        context.transmitter.send(UIOrder::SceneChange(active_scene_index)).unwrap();
                        scene.write().unwrap().set_dirty(false);
                        last_scene_change = Instant::now();
                        context.final_img = false;
                    }
                }
            }

            match event {
                Event::WindowEvent { event, .. } => {
                    handle_event(event, &mut context, &mut ui, flow);
                }
                _ => {}
            }
        })
        .expect("ERROR : Unexpected error when running the event loop");
}
