extern crate image;
use display::display_scene;
use parsing::get_scene;
use winit::event::VirtualKeyCode;

pub mod display;
pub mod gui;
pub mod model;
pub mod parsing;
pub mod picker;
pub mod render;

const SCREEN_WIDTH: usize = 1600;
const SCREEN_HEIGHT: usize = 900;
const SCREEN_WIDTH_U32: u32 = SCREEN_WIDTH as u32;
const SCREEN_HEIGHT_U32: u32 = SCREEN_HEIGHT as u32;
const GUI_WIDTH: u32 = 400;
const GUI_HEIGHT: u32 = 600;
const MAX_THREADS: usize = 4;
const BASE_SIMPLIFICATION: usize = 32;
const MAX_DEPTH: u8 = 10;
const ANTIALIASING: f64 = 0.002;
const MAX_ITERATIONS: i32 = 200;

const SCENE_FOLDER: &str = "scenes";
const PICKER_LINE_HEIGHT: f64 = 30.0;
// const SCENE: &str = "scenes/sphere.json";
const FPS: u64 = 20;

const RGB_KEYS: [&str; 3] = ["colr", "colg", "colb"];
const CAM_MOVE_KEYS: [VirtualKeyCode; 10] = [
    VirtualKeyCode::W,
    VirtualKeyCode::A,
    VirtualKeyCode::S,
    VirtualKeyCode::D,
    VirtualKeyCode::Up,
    VirtualKeyCode::Left,
    VirtualKeyCode::Down,
    VirtualKeyCode::Right,
    VirtualKeyCode::Space,
    VirtualKeyCode::LShift,
];

pub fn run() {
    let path = String::from("scenes/sphere.json");
    if path != "" {
        let scene = get_scene(&path);
        display_scene(scene);
    }
}

pub fn error(msg: &str) {
    eprintln!("Error: {}", msg);
}
