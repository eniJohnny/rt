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
const SCREEN_WIDTH_U32: u32 = SCREEN_WIDTH as u32;
const SCREEN_HEIGHT_U32: u32 = SCREEN_HEIGHT as u32;
const GUI_WIDTH: u32 = 400;
const GUI_HEIGHT: u32 = 600;
const MAX_THREADS: usize = 16;

pub fn run() {
    let mut scene = get_scene();
    let camera = scene.camera_mut();

    generate_rays(camera);

    display_scene(scene);
}

pub fn error(msg: &str) {
    eprintln!("Error: {}", msg);
}