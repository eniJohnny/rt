use nannou::prelude::*;
use crate::events::{Model, event, model};
use crate::model::scene::Scene;
use crate::gui::draw_gui;

pub fn display_scene(scene: &Scene) {
    nannou::app(model).view(view).event(event).run();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw_gui(&draw);

    draw.to_frame(app, &frame).expect("Could not draw frame")
}