use image::RgbaImage;
use minifb::{Key, Window, WindowOptions};

use crate::{model::scene::Scene, render::render_scene, SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn display_scene(scene: &Scene) {
    let mut window = Window::new("name", SCREEN_WIDTH, SCREEN_HEIGHT, WindowOptions::default()).expect("Window opening failed");
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let img: Vec<u32> = render_scene(scene)
            .pixels()
            .map(|p| {
                let r = p[0] as u32;
                let g = p[1] as u32;
                let b = p[2] as u32;
                let a = p[3] as u32;
                (a << 24) | (r << 16) | (g << 8) | b
            }).collect();
        window.update_with_buffer(&img, SCREEN_WIDTH, SCREEN_HEIGHT).expect("Probleme updating window with the buffered image");
    }
}