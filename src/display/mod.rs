use nannou::prelude::*;
use crate::events::{Model, event, model};
use crate::model::materials::Color;
use crate::model::scene::Scene;
use crate::gui::{draw_gui, pixel_put};
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn display_scene(scene: &Scene) {
    nannou::app(model).view(view).event(event).run();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw_gui(&draw);

    let rays = vec![vec![Color::new(255, 255, 255); 1400]; 900];
    display_rays(rays, &draw);

    draw.to_frame(app, &frame).expect("Could not draw frame")
}

fn display_rays(rays: Vec<Vec<Color>>, draw: &Draw) {
    for y in 0..rays.len() {
        for x in 0..rays[y].len() {
            let red = rays[y][x].r();
            let green = rays[y][x].g();
            let blue = rays[y][x].b();
            let color = Rgb::new(red, green, blue);

            let half_width = SCREEN_WIDTH as i32 / 2;
            let half_height = SCREEN_HEIGHT as i32 / 2;
            let pos_x: i32 = x as i32 - half_width;
            let pos_y: i32 = y as i32 - half_height;
            
            pixel_put(pos_x, pos_y, color, &draw);
        }
    }
}