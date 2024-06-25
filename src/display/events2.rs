use std::{
    sync::{
        mpsc::{Receiver, Sender},
        Arc, RwLock,
    },
    time::{Duration, Instant},
};

use image::{ImageBuffer, Rgba, RgbaImage};
use pixels::Pixels;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use crate::model::scene::Scene;

pub fn main_loop(event_loop: EventLoop<()>, scene: Arc<RwLock<Scene>>) {
    event_loop.run(move |event, _, control_flow: &mut ControlFlow| {
        *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(20));

        match event {
            Event::WindowEvent { window_id, event } => {
                handle_event(event, &scene, control_flow);
            }
            _ => {}
        }
    })
}

fn handle_event(event: WindowEvent, scene: &Arc<RwLock<Scene>>, flow: &mut ControlFlow) {
    match event {
        WindowEvent::CursorMoved { position, .. } => {}
        WindowEvent::CloseRequested => {
            // Close the window
            *flow = ControlFlow::Exit;
        }
        _ => {}
    }
}
