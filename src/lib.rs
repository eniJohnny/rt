extern crate image;
use display::mainloop::start_scene;
use parsing::get_scene;

pub mod display;
pub mod ui;
pub mod model;
pub mod parsing;
pub mod picker;
pub mod render;
pub mod bvh;

const USING_BVH: bool = true;
const SCENE_FOLDER: &str = "scenes";
const SCENE: &str = "composed_shapes";

/************* Camera **************/
const STEP: f64 = 0.2;
const LOOK_STEP: f64 = 0.05;

/************* Displacement **************/
const DISPLACEMENT: bool = false;
const PLANE_DISPLACED_DISTANCE: f64 = 0.25;
const PLANE_DISPLACEMENT_STEP: f64 = 0.1;
const SPHERE_DISPLACED_DISTANCE: f64 = 0.05;
const SPHERE_DISPLACEMENT_STEP: f64 = 0.1;

const AABB_OPACITY: f64 = 0.0;
const AABB_STEPS_NB: usize = 10;
const DISPLAY_WIREFRAME: bool = false;
const WIREFRAME_THICKNESS: f64 = 0.05;
const ERROR_MARGIN: f64 = 0.000000000001;
const SCREEN_WIDTH: usize = 1600;
const SCREEN_HEIGHT: usize = 900;
const SCREEN_WIDTH_U32: u32 = SCREEN_WIDTH as u32;
const SCREEN_HEIGHT_U32: u32 = SCREEN_HEIGHT as u32;
const MAX_THREADS: usize = 4;
const BASE_SIMPLIFICATION: usize = 32;
const MAX_DEPTH: usize = 10;
const ANTIALIASING: f64 = 0.001;
const MAX_ITERATIONS: usize = 10;

/************* Modifiers **************/
const ANAGLYPH: bool = false;
const ANAGLYPH_OFFSET_X: isize = 80;
const ANAGLYPH_OFFSET_Y: isize = 16;
// Available filters: cartoon, grayscale, sepia, none
const FILTERS: [&str; 3] = ["cartoon", "grayscale", "sepia"];
const FILTER: &str = "none";
const EDGE_THRESHOLD: u32 = 100;

/********* Default UISettings *********/
const MARGIN: usize = 3;
const GUI_WIDTH: u32 = 600;
const GUI_HEIGHT: u32 = 600;
// Available view modes: Simple, HighDef, Norm, BVH
const VIEW_MODE: &str = "Simple";
const FIELD_PADDING_X: u32 = 10;
const FIELD_PADDING_Y: u32 = 3;
const _INDENT_PADDING: u32 = 10;
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
        // if OBJ {
        //     scene.add_obj(String::from("obj/cat.obj"));
        //     println!("Number of triangles: {}", scene.elements().len());
        // }
        scene.update_bvh();
        start_scene(scene);
    }
}

pub fn error(msg: &str) {
    eprintln!("Error: {}", msg);
}