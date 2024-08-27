extern crate image;
use display::mainloop::start_scene;
use image::flat::View;
use parsing::get_scene;
use render::settings::ViewMode;
use winit::keyboard::KeyCode;

pub mod display;
pub mod ui;
pub mod model;
pub mod parsing;
pub mod picker;
pub mod render;

const SCREEN_WIDTH: usize = 1600;
const SCREEN_HEIGHT: usize = 900;
const SCREEN_WIDTH_U32: u32 = SCREEN_WIDTH as u32;
const SCREEN_HEIGHT_U32: u32 = SCREEN_HEIGHT as u32;
const MAX_THREADS: usize = 6;
const BASE_SIMPLIFICATION: usize = 32;
const MAX_DEPTH: usize = 10;
const ANTIALIASING: f64 = 0.001;
const MAX_ITERATIONS: usize = 500;
const MAX_EMISSIVE: f64 = 100.;

/********* Default UISettings *********/
const MARGIN: usize = 3;
const GUI_WIDTH: u32 = 600;
const GUI_HEIGHT: u32 = 600;
const VIEW_MODE: ViewMode = ViewMode::HighDef;
const FIELD_PADDING_X: u32 = 10;
const FIELD_PADDING_Y: u32 = 3;
const INDENT_PADDING: u32 = 10;
const BASE_FONT_SIZE: u32 = 16;
const BUTTON_FONT_SIZE: u32 = 36;
const UI_REFRESH_TIME: u32 = 1000;

/*********** UI default references *********/
const UISETTINGS: &str = "uisettings";
const SETTINGS: &str = "settings";
const TOOLBAR: &str = "toolbar";

const SCENE_FOLDER: &str = "scenes";
const PICKER_LINE_HEIGHT: f64 = 30.0;
// const SCENE: &str = "scenes/sphere.json";
const FPS: u64 = 20;

const RGB_KEYS: [&str; 3] = ["colr", "colg", "colb"];
const CAM_MOVE_KEYS: [KeyCode; 10] = [
    KeyCode::KeyW,
    KeyCode::KeyA,
    KeyCode::KeyS,
    KeyCode::KeyD,
    KeyCode::ArrowUp,
    KeyCode::ArrowLeft,
    KeyCode::ArrowDown,
    KeyCode::ArrowRight,
    KeyCode::Space,
    KeyCode::ShiftLeft,
];

pub fn run() {
    let path = String::from("scenes/metalrough.json");
    if path != "" {
        let mut scene = get_scene(&path);
        scene.add_skysphere_texture("skysphere.jpg");
        start_scene(scene);
    }
}

pub fn error(msg: &str) {
    eprintln!("Error: {}", msg);
}
