use nannou::{draw, prelude::*};
use crate::{SCREEN_WIDTH, SCREEN_HEIGHT};

pub(crate) fn draw_gui(draw: &draw::Draw) {

    let gui_x_start: f32 = (SCREEN_WIDTH / 2 - 200) as f32;
    let gui_x_end: f32 = (SCREEN_WIDTH  / 2) as f32;
    let gui_y_start: f32 = (SCREEN_HEIGHT / 2) as f32;
    let gui_y_end: f32 = (SCREEN_HEIGHT as i32 / 2 * -1) as f32;

    line_put(Point2::new(gui_x_start, gui_y_start), Point2::new(gui_x_start, gui_y_end), nannou::color::WHITE, &draw);

    let test_button: [Point2; 2] = [
        Point2::new(gui_x_start + 20., gui_y_start - 20.),
        Point2::new(gui_x_end - 20., gui_y_start - 70.)
    ];

    rect_put(test_button[0], test_button[1], nannou::color::GRAY, &draw);
}

fn pixel_put(x: i32, y: i32, color: nannou::color::Rgb<u8>, draw: &draw::Draw) {
    draw.rect()
        .w_h(1.0, 1.0)
        .x_y(x as f32, y as f32)
        .color(color);
}

#[allow(dead_code)]
fn line_put(start: Point2, end: Point2, color: nannou::color::Rgb<u8>, draw: &draw::Draw) {

    let x = end.x - start.x;
    let y = end.y - start.y;
    let max = if abs(x) > abs(y) { abs(x) } else { abs(y) };
    let x_step = x / max;
    let y_step = y / max;
    let mut current = start;
    for _ in 0..max as i32 {
        pixel_put(current.x as i32, current.y as i32, color, draw);
        current.x += x_step;
        current.y += y_step;
    }
}

#[allow(dead_code)]
fn rect_put(start: Point2, end: Point2, color: nannou::color::Rgb<u8>, draw: &draw::Draw) {
    let mut upper_left = Point2::default();
    let mut lower_right = Point2::default();

    upper_left.x = if start.x < end.x { start.x } else { end.x };
    upper_left.y = if start.y < end.y { start.y } else { end.y };
    lower_right.x = if start.x > end.x { start.x } else { end.x };
    lower_right.y = if start.y > end.y { start.y } else { end.y };

    let upper_right = Point2::new(lower_right.x, upper_left.y);
    let lower_left = Point2::new(upper_left.x, lower_right.y);

    line_put(upper_left, upper_right, color, draw);
    line_put(upper_right, lower_right, color, draw);
    line_put(lower_right, lower_left, color, draw);
    line_put(lower_left, upper_left, color, draw);
}

#[allow(dead_code)]
fn filled_rect_put(start: Point2, end: Point2, color: nannou::color::Rgb<u8>, draw: &draw::Draw) {
    let mut upper_left = Point2::default();
    let mut lower_right = Point2::default();

    upper_left.x = if start.x < end.x { start.x } else { end.x };
    upper_left.y = if start.y < end.y { start.y } else { end.y };
    lower_right.x = if start.x > end.x { start.x } else { end.x };
    lower_right.y = if start.y > end.y { start.y } else { end.y };

    for y in upper_left.y as i32..lower_right.y as i32 {
        line_put(Point2::new(upper_left.x, y as f32), Point2::new(lower_right.x, y as f32), color, draw);
    }
}