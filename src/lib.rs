extern crate image;
use display::mainloop::start_ui;

pub mod display;
pub mod ui;
pub mod model;
pub mod parsing;
pub mod render;
pub mod bvh;


/************* General settings ***********/
const SCREEN_WIDTH: usize = 1800;
const SCREEN_HEIGHT: usize = 900;
const SCREEN_WIDTH_U32: u32 = SCREEN_WIDTH as u32;
const SCREEN_HEIGHT_U32: u32 = SCREEN_HEIGHT as u32;
const SCENE_FOLDER: &str = "scenes";
// const SCENE: &str = "cornell_box";
const TEXTURE_FOLDER: &str = "textures";
const SKYBOX_TEXTURE: &str = "skybox/skybox_night.jpg";

/************* Camera **************/
const STEP: f64 = 0.2;
const LOOK_STEP: f64 = 0.05;

/************* Displacement **************/
const DISPLACEMENT: bool = false;
const PLANE_DISPLACED_DISTANCE: f64 = 0.25;
const PLANE_DISPLACEMENT_STEP: f64 = 0.1;
const SPHERE_DISPLACED_DISTANCE: f64 = 0.05;
const SPHERE_DISPLACEMENT_STEP: f64 = 0.1;

/************ Render settings ************/
const MAX_THREADS: usize = 4;
const BASE_SIMPLIFICATION: usize = 8;
const TILE_SIZE: usize = 8;
const MAX_DEPTH: usize = 5;
const ANTIALIASING: f64 = 0.001;
const MAX_ITERATIONS: usize = 1500;
const BOUNCE_OFFSET: f64 = 0.0001;
const ERROR_MARGIN: f64 = 0.000001;


/*************** BVH Settings ************/
const USING_BVH: bool = true;
const BVH_SPLIT_STEPS: usize = 50;


//Debug settings
const DISPLAY_WIREFRAME: bool = false;
const WIREFRAME_THICKNESS: f64 = 0.05;

/************* Modifiers **************/
const ANAGLYPH_OFFSET_X: isize = 80;
const ANAGLYPH_OFFSET_Y: isize = 16;
// Available filters: cartoon, grayscale, sepia, none
const EDGE_THRESHOLD: u32 = 100;

/********* UISettings *********/
const MARGIN: usize = 3;
const GUI_WIDTH: u32 = 600;
const GUI_HEIGHT: u32 = 600;
// Available view modes: Simple, HighDef, Norm, BVH, Phong
const VIEW_MODE: &str = "Simple";
const FIELD_PADDING_X: u32 = 10;
const FIELD_PADDING_Y: u32 = 3;
const BASE_FONT_SIZE: u32 = 16;
const UI_REFRESH_TIME: u32 = 1000;
const SCROLL_PIXEL_AMOUNT: u32 = 20;

/*********** UI default references *********/
const SETTINGS: &str = "settings";
const OBJECTS: &str = "objects";
const TOOLBAR: &str = "toolbar";
const SCENE_TOOLBAR: &str = "scene_toolbar";
const ELEMENT: &str = "element";

pub fn run() {
    start_ui();
}

pub fn error(msg: &str) {
    eprintln!("Error: {}", msg);
}