use model::Scene;
use parsing::getScene;
use render::render_scene;

pub mod gui;
pub mod model;
pub mod parsing;
pub mod render;

pub fn run() {
    let scene = getScene();
}

pub fn event_loop(scene: &Scene) {
    render_scene(scene)
}