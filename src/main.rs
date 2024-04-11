use rt::run;

use nannou::{draw, prelude::*, state::mouse};
use std::collections::HashMap;

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;

struct Model {}

fn main() {
    // run();
    // Create window
    // nannou::sketch(view).size(screen_width, screen_height).event(event).run();
    nannou::app(model).size(SCREEN_WIDTH, SCREEN_HEIGHT).view(view).event(event).run();

    // Parse JSON
    // parse_json();
}

fn window_event(_app: &App, _model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(_key) => {}
        KeyReleased(_key) => {}
        MouseMoved(_pos) => {}
        MousePressed(_button) => {}
        MouseReleased(_button) => {}
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_amount, _phase) => {}
        Moved(_pos) => {}
        Resized(_size) => {}
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}


fn raw_window_event(_app: &App, _model: &mut Model, _event: &nannou::winit::event::WindowEvent) {}

fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {}
fn key_released(_app: &App, _model: &mut Model, _key: Key) {}

fn mouse_moved(_app: &App, _model: &mut Model, _pos: Point2) {}

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {
    // Get mouse position
    let mouse_pos = _app.mouse.position(); // Access the mouse function
    println!("Mouse position: {:?}", mouse_pos);
}

fn mouse_released(_app: &App, _model: &mut Model, _button: MouseButton) {}

fn mouse_wheel(_app: &App, _model: &mut Model, _dt: MouseScrollDelta, _phase: TouchPhase) {}

fn mouse_entered(_app: &App, _model: &mut Model) {}

fn mouse_exited(_app: &App, _model: &mut Model) {}

fn touch(_app: &App, _model: &mut Model, _touch: TouchEvent) {}

fn touchpad_pressure(_app: &App, _model: &mut Model, _pressure: TouchpadPressure) {}

fn window_moved(_app: &App, _model: &mut Model, _pos: Point2) {}

fn window_resized(_app: &App, _model: &mut Model, _dim: Vec2) {}

fn window_focused(_app: &App, _model: &mut Model) {}

fn window_unfocused(_app: &App, _model: &mut Model) {}

fn window_closed(_app: &App, _model: &mut Model) {}

fn hovered_file(_app: &App, _model: &mut Model, _path: std::path::PathBuf) {}

fn hovered_file_cancelled(_app: &App, _model: &mut Model) {}

fn dropped_file(_app: &App, _model: &mut Model, _path: std::path::PathBuf) {}

fn model(app: &App) -> Model {
    app.new_window()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .event(window_event)
        .raw_event(raw_window_event)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .mouse_moved(mouse_moved)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_wheel(mouse_wheel)
        .mouse_entered(mouse_entered)
        .mouse_exited(mouse_exited)
        .touch(touch)
        .touchpad_pressure(touchpad_pressure)
        .moved(window_moved)
        .resized(window_resized)
        .hovered_file(hovered_file)
        .hovered_file_cancelled(hovered_file_cancelled)
        .dropped_file(dropped_file)
        .focused(window_focused)
        .unfocused(window_unfocused)
        .closed(window_closed)
        .build()
        .unwrap();
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn event(_app: &App, _model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            //raw: _,
            simple: _,
        } => {}
        Event::DeviceEvent(_device_id, _event) => {}
        Event::Update(_dt) => {}
        Event::Suspended => {}
        Event::Resumed => {}
    }
}

fn parse_json() {
    let content = std::fs::read_to_string("scenes/scene.json").unwrap();
    let mut objects: Vec<HashMap<String, String>> = Vec::new();
    let mut i = 0;

    println!("\n---Start---\n");
    while i < content.len() && content[i..].find('{') != None {
        let mut object: HashMap<String, String> = HashMap::new();
        let remaining = &content[i..];
        let start = remaining.find('{').unwrap();
        let end = remaining.find("\n    }").unwrap() + 6;
        let object_str = &remaining[start..end];
        i += end;

        for prop in object_str.split(",\n        \"") {
            let prop = prop.trim();
            let mut prop = prop.split(": ");
            let key: String = prop.next().unwrap().trim_matches(['"', ' ', '\n', '{', '}']).to_string();
            let value: String = prop.next().unwrap().trim_matches(['{', '"', ' ', '\n', '}']).to_string();

            if value.contains('[') {
                let str = value.trim_matches(['[', ']']).replace(", ", ",");
                let tmp: Vec<&str> = str.split(",").collect();

                if key == "position" {
                    object.insert("position_x".to_string(), tmp[0].to_string());
                    object.insert("position_y".to_string(), tmp[1].to_string());
                    object.insert("position_z".to_string(), tmp[2].to_string());
                } else if key == "direction" {
                    object.insert("direction_x".to_string(), tmp[0].to_string());
                    object.insert("direction_y".to_string(), tmp[1].to_string());
                    object.insert("direction_z".to_string(), tmp[2].to_string());
                } else if key == "color" {
                    object.insert("color_r".to_string(), tmp[0].to_string());
                    object.insert("color_g".to_string(), tmp[1].to_string());
                    object.insert("color_b".to_string(), tmp[2].to_string());
                }

            } else {
                object.insert(key, value);
            }
        }
        objects.push(object);
    }
    // Here, objects is a vector of HashMaps, each representing an object in the scene.
    for object in objects {
        println!("{:?}", object);
    }
}

fn draw_gui(draw: &draw::Draw) {
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

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    draw_gui(&draw);

    // line_put(Point2::new(0.0, 0.0), Point2::new(20.0, 20.0), nannou::color::WHITE, &draw);
    // rect_put(Point2::new(-150.0, -350.0), Point2::new(150.0, -250.0), nannou::color::WHITE, &draw);
    // filled_rect_put(Point2::new(150.0, 350.0), Point2::new(-150.0, 250.0), nannou::color::WHITE, &draw);

    draw.to_frame(app, &frame).unwrap();
}

fn pixel_put(x: i32, y: i32, color: nannou::color::Rgb<u8>, draw: &draw::Draw) {
    draw.rect()
        .w_h(1.0, 1.0)
        .x_y(x as f32, y as f32)
        .color(color);
}

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
    run()
}