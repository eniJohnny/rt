use model::Scene;
use parsing::get_scene;
use render::display_scene;

pub mod gui;
pub mod model;
pub mod parsing;
pub mod render;
pub mod events;

pub const SCREEN_WIDTH: u32 = 1600;
pub const SCREEN_HEIGHT: u32 = 900;

const WINDOW_WIDTH: usize = 1900;
const WINDOW_HEIGHT: usize = 1080;

pub fn run() {
    let scene = get_scene();
    
    event_loop(&scene);
}

pub fn event_loop(scene: &Scene) {
    display_scene(scene)
}
