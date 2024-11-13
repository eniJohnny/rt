use std::{
    sync::{Arc, RwLock}, time::{Duration, Instant}
};

use pixels::Pixels;
use winit::{
    dpi::LogicalSize,
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    model::scene::Scene,
    SCREEN_HEIGHT, SCREEN_HEIGHT_U32, SCREEN_WIDTH, SCREEN_WIDTH_U32,
};

use super::{
    display::redraw_if_necessary,
    events::{handle_event, key_held}, ui_setup::setup_ui,
};

pub fn start_scene(scene: Scene) {
    let scene = scene;

    // Set up window and event loop (can't move them elsewhere because of the borrow checker)
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32))
        .with_title("Image Viewer")
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    // Set up pixels object
    let pixels = {
        let texture = pixels::SurfaceTexture::new(SCREEN_WIDTH_U32, SCREEN_HEIGHT_U32, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, texture).unwrap()
    };

    let scene = Arc::new(RwLock::new(scene));

    main_loop(event_loop, scene, pixels);
}

pub fn main_loop(event_loop: EventLoop<()>, scene: Arc<RwLock<Scene>>, mut pixels: Pixels) {
    let mut ui: crate::ui::ui::UI = setup_ui(&scene);
    let mut last_draw = Instant::now();
    let mut last_input = Instant::now();
    let mut last_scene_change = Instant::now();

    event_loop
        .run(move |event, flow| {
            // sleep(Duration::from_millis(10));
            flow.set_control_flow(ControlFlow::WaitUntil(
                Instant::now() + Duration::from_millis(20),
            ));

            // We redraw if the ui is dirty(needs redraw), or we receive a new image from the render
            if last_draw.elapsed().as_millis() > 20 {
                redraw_if_necessary(&mut ui, &scene, &mut pixels);
                last_draw = Instant::now();
            }

            // We handle every held inputs every 20ms. This basically is only used to handle camera movements
            if ui.editing().is_none()
                && ui.inputs().len() > 0
                && last_input.elapsed().as_millis() > 10
            {
                let inputs = ui.inputs().clone();
                for input in inputs {
                    key_held(&scene, &mut ui, flow, input);
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
                if !context.final_img && !context.image_asked {
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
