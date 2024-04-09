use rt::run;

use nannou::{draw, prelude::*};
use std::collections::HashMap;

use rt::run;

fn main() {
    run();
    // Create window
    // nannou::sketch(view).size(400, 800).run();

    // Parse JSON
    parse_json();
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

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    // line_put(Point2::new(0.0, 0.0), Point2::new(20.0, 20.0), nannou::color::WHITE, &draw);
    rect_put(Point2::new(-150.0, -350.0), Point2::new(150.0, -250.0), nannou::color::WHITE, &draw);
    filled_rect_put(Point2::new(150.0, 350.0), Point2::new(-150.0, 250.0), nannou::color::WHITE, &draw);

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