extern crate image;
extern crate minifb;

use std::f64::consts::PI;
use display::display_scene;
use model::scene::Scene;
use parsing::get_scene;


pub mod gui;
pub mod model;
pub mod parsing;
pub mod render;
pub mod events;
pub mod display;

const SCREEN_WIDTH: usize = 1600;
const SCREEN_HEIGHT: usize = 900;

const VFOV: i32 = 90;
const VFOV_RAD: f64 = VFOV as f64 * 2. * PI / 360.;

pub fn run() {
    let scene = get_scene();
    
    // print_scene(&scene);
    event_loop(&scene);
}

pub fn event_loop(scene: &Scene) {
    display_scene(scene)
}

pub fn error(msg: &str) {
    eprintln!("Error: {}", msg);
}