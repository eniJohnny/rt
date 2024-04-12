use model::scene::Scene;
use parsing::get_scene;
use display::display_scene;

pub mod gui;
pub mod model;
pub mod parsing;
pub mod render;
pub mod events;
pub mod display;

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;

pub fn run() {
    let scene = get_scene();
    
    event_loop(&scene);
}

pub fn event_loop(scene: &Scene) {
    display_scene(scene)
}

pub fn error(msg: &str) {
    eprintln!("Error: {}", msg);
}