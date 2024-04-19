extern crate image;
use display::display_scene;
use parsing::get_scene;

pub mod display;
pub mod gui;
pub mod model;
pub mod parsing;
pub mod render;

const SCREEN_WIDTH: usize = 1600;
const SCREEN_HEIGHT: usize = 900;
const SCREEN_WIDTH_U32: u32 = SCREEN_WIDTH as u32;
const SCREEN_HEIGHT_U32: u32 = SCREEN_HEIGHT as u32;
const GUI_WIDTH: u32 = 400;
const GUI_HEIGHT: u32 = 600;
const MAX_THREADS: usize = 16;
const BASE_SIMPLIFICATION: usize = 64;

pub fn run() {
    let mut scene = get_scene();
    display_scene(scene);
}

pub fn error(msg: &str) {
    eprintln!("Error: {}", msg);
}
