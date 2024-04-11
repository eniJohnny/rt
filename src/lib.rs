use model::scene::Scene;
use parsing::get_scene;
use render::render_scene;

pub mod gui;
pub mod model;
pub mod parsing;
pub mod render;

const WINDOW_WIDTH: usize = 1900;
const WINDOW_HEIGHT: usize = 1080;

pub fn run() {
    let scene = get_scene();
}

pub fn event_loop(scene: &Scene) {
    render_scene(scene)
}