extern crate image;
use display::display_scene;
use parsing::get_scene;
use render::raycasting::generate_rays;

pub mod gui;
pub mod model;
pub mod parsing;
pub mod render;
pub mod display;

const SCREEN_WIDTH: usize = 1600;
const SCREEN_HEIGHT: usize = 900;
const GUI_WIDTH: u32 = 200;
const MAX_THREADS: usize = 16;

pub fn run() {
    let mut scene = get_scene();
    generate_rays(scene.camera_mut());
    display_scene();
}

pub fn error(msg: &str) {
    eprintln!("Error: {}", msg);
}