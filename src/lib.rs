extern crate image;
use display::mainloop::start_scene;
use parsing::get_scene;
use render::settings::ViewMode;

pub mod display;
pub mod ui;
pub mod model;
pub mod parsing;
pub mod picker;
pub mod render;
pub mod bvh;


const USING_BVH: bool = true;
const SCENE_FOLDER: &str = "scenes";
const SCENE: &str = "empty";
/* DISPLACEMENT */
const DISPLACEMENT: bool = false;
const PLANE_DISPLACED_DISTANCE: f64 = 0.25;
const PLANE_DISPLACEMENT_STEP: f64 = 0.1;
const SPHERE_DISPLACED_DISTANCE: f64 = 0.05;
const SPHERE_DISPLACEMENT_STEP: f64 = 0.1;

const AABB_OPACITY: f64 = 0.0;
const AABB_STEPS_NB: usize = 10;
const DISPLAY_WIREFRAME: bool = true;
const WIREFRAME_THICKNESS: f64 = 0.05;
const ERROR_MARGIN: f64 = 0.000000000001;
const SCREEN_WIDTH: usize = 1600;
const SCREEN_HEIGHT: usize = 900;
const SCREEN_WIDTH_U32: u32 = SCREEN_WIDTH as u32;
const SCREEN_HEIGHT_U32: u32 = SCREEN_HEIGHT as u32;
const MAX_THREADS: usize = 6;
const BASE_SIMPLIFICATION: usize = 32;
const MAX_DEPTH: usize = 10;
const ANTIALIASING: f64 = 0.001;
const MAX_ITERATIONS: usize = 1500;

/************* Modifiers **************/
const ANAGLYPH: bool = false;
const ANAGLYPH_OFFSET_X: isize = 80;
const ANAGLYPH_OFFSET_Y: isize = 16;
// Available filters: none, grayscale, sepia
const FILTER: &str = "grayscale";

/********* Default UISettings *********/
const MARGIN: usize = 3;
const GUI_WIDTH: u32 = 600;
const GUI_HEIGHT: u32 = 600;
const VIEW_MODE: ViewMode = ViewMode::HighDef;
const FIELD_PADDING_X: u32 = 10;
const FIELD_PADDING_Y: u32 = 3;
const INDENT_PADDING: u32 = 10;
const BASE_FONT_SIZE: u32 = 16;
const UI_REFRESH_TIME: u32 = 1000;

/*********** UI default references *********/
const UISETTINGS: &str = "uisettings";
const SETTINGS: &str = "settings";
const TOOLBAR: &str = "toolbar";
const ELEMENT: &str = "element";

const PICKER_LINE_HEIGHT: f64 = 30.0;
pub fn run() {
    let path = String::from(format!("{}/{}.json", SCENE_FOLDER, SCENE));
    if path != "" {
        let mut scene = get_scene(&path);
        scene.add_skysphere_texture("skysphere.jpg");
        
        if DISPLAY_WIREFRAME {
            scene.add_wireframes();
        }

        scene.update_bvh();
        start_scene(scene);
    }
}

pub fn error(msg: &str) {
    eprintln!("Error: {}", msg);
}