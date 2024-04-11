use crate::model::Scene;
use crate::gui::draw_gui;
use crate::events::{Model, event, model};
use nannou::prelude::*;

pub const SCREEN_WIDTH: u32 = 1600;
pub const SCREEN_HEIGHT: u32 = 900;

pub mod camera;
pub mod light;

pub fn render_scene(scene: &Scene) {
    nannou::app(model).view(view).event(event).run();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw_gui(&draw);

    // line_put(Point2::new(0.0, 0.0), Point2::new(20.0, 20.0), nannou::color::WHITE, &draw);
    // rect_put(Point2::new(-150.0, -350.0), Point2::new(150.0, -250.0), nannou::color::WHITE, &draw);
    // filled_rect_put(Point2::new(150.0, 350.0), Point2::new(-150.0, 250.0), nannou::color::WHITE, &draw);

    draw.to_frame(app, &frame).unwrap();
}