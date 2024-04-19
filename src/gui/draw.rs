use image::Rgba;

use crate::{display::draw_text, model::{maths::vec2::Vec2, shapes::{plane, sphere}}, GUI_HEIGHT, GUI_WIDTH};

use super::{Gui, TextFormat};


pub fn draw_sphere_gui (img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>, sphere: &sphere::Sphere) -> Gui {
    let height: u32 = GUI_HEIGHT;
    let width: u32 = GUI_WIDTH;
    let size: Vec2 = Vec2::new(width as f64, height as f64);

    let x_start: u32 = (img.width() - width) as u32;
    let x_end: u32 = img.width();
    let y_start: u32 = 0;
    let y_end: u32 = height;

    for x in x_start..x_end {
        for y in y_start..y_end {
            img.put_pixel(x, y, Rgba([50, 50, 50, 255]));
        }
    }

    let mut titles = TextFormat {
        size: size.clone(),
        ..Default::default()
    };

    let mut params = TextFormat {
        size: size.clone(),
        background_color: Rgba([89, 89, 89, 255]),
        ..Default::default()
    };

    let mut gui = Gui::new();

    gui.keys.push("posx".to_string());
    gui.keys.push("posy".to_string());
    gui.keys.push("posz".to_string());
    gui.keys.push("dirx".to_string());
    gui.keys.push("diry".to_string());
    gui.keys.push("dirz".to_string());
    gui.keys.push("radius".to_string());

    gui.values.push(sphere.pos().x().to_string());
    gui.values.push(sphere.pos().y().to_string());
    gui.values.push(sphere.pos().z().to_string());
    gui.values.push(sphere.dir().x().to_string());
    gui.values.push(sphere.dir().y().to_string());
    gui.values.push(sphere.dir().z().to_string());
    gui.values.push(sphere.radius().to_string());

    titles.parse_and_draw_text(img, 0, "Sphere", "");
    titles.parse_and_draw_text(img, 1, "Position:", "");
    titles.parse_and_draw_text(img, 5, "Direction:", "");
    titles.parse_and_draw_text(img, 9, "Misc:", "");

    gui.hitboxes.push(params.parse_and_draw_text(img, 2, " X:", &sphere.pos().x().to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(img, 3, " Y:", &sphere.pos().y().to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(img, 4, " Z:", &sphere.pos().z().to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(img, 6, " X:", &sphere.dir().x().to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(img, 7, " Y:", &sphere.dir().y().to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(img, 8, " Z:", &sphere.dir().z().to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(img, 10, " Radius:", &sphere.radius().to_string()));

    draw_gui_buttons(img, &gui);

    gui
}

pub fn draw_plane_gui (img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>, plane: &plane::Plane) -> Gui {
    let height: u32 = GUI_HEIGHT;
    let width: u32 = GUI_WIDTH;
    let size: Vec2 = Vec2::new(width as f64, height as f64);

    let x_start: u32 = (img.width() - width) as u32;
    let x_end: u32 = img.width();
    let y_start: u32 = 0;
    let y_end: u32 = height;

    for x in x_start..x_end {
        for y in y_start..y_end {
            img.put_pixel(x, y, Rgba([50, 50, 50, 255]));
        }
    }

    let mut titles = TextFormat {
        size: size.clone(),
        ..Default::default()
    };

    let mut params = TextFormat {
        size: size.clone(),
        background_color: Rgba([89, 89, 89, 255]),
        ..Default::default()
    };

    let mut gui = Gui::new();

    gui.keys.push("posx".to_string());
    gui.keys.push("posy".to_string());
    gui.keys.push("posz".to_string());
    gui.keys.push("dirx".to_string());
    gui.keys.push("diry".to_string());
    gui.keys.push("dirz".to_string());

    gui.values.push(plane.pos().x().to_string());
    gui.values.push(plane.pos().y().to_string());
    gui.values.push(plane.pos().z().to_string());
    gui.values.push(plane.dir().x().to_string());
    gui.values.push(plane.dir().y().to_string());
    gui.values.push(plane.dir().z().to_string());

    titles.parse_and_draw_text(img, 0, "Plane", "");
    titles.parse_and_draw_text(img, 1, "Position:", "");
    titles.parse_and_draw_text(img, 5, "Direction:", "");
    
    gui.hitboxes.push(params.parse_and_draw_text(img, 2, " X:", &plane.pos().x().to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(img, 3, " Y:", &plane.pos().y().to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(img, 4, " Z:", &plane.pos().z().to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(img, 6, " X:", &format!("{:.3}", plane.dir().x())));
    gui.hitboxes.push(params.parse_and_draw_text(img, 7, " Y:", &format!("{:.3}", plane.dir().y())));
    gui.hitboxes.push(params.parse_and_draw_text(img, 8, " Z:", &format!("{:.3}", plane.dir().z())));

    draw_gui_buttons(img, &gui);

    gui
}

fn is_corner(x: u32, y: u32, x_start: u32, y_start: u32, x_end: u32, y_end: u32) -> bool {
    let start_offset = 2;
    let end_offset = 3;

    if x < x_start + start_offset && y < y_start + start_offset {
        return true;
    }
    if x < x_start + start_offset && y > y_end - end_offset {
        return true;
    }
    if x > x_end - end_offset && y < y_start + start_offset {
        return true;
    }
    if x > x_end - end_offset && y > y_end - end_offset {
        return true;
    }

    false
}

pub fn draw_button_background(img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>, hitbox: &(Vec2, Vec2), color: Rgba<u8>) {
    let upper_left_corner = &hitbox.0;
    let lower_right_corner = &hitbox.1;

    let x_start = *upper_left_corner.x() as u32;
    let x_end = *lower_right_corner.x() as u32;
    let y_start = *upper_left_corner.y() as u32;
    let y_end = *lower_right_corner.y() as u32;

    for x in x_start..x_end {
        for y in y_start..y_end {
            if is_corner(x, y, x_start, y_start, x_end, y_end) == false {
                img.put_pixel(x, y, color);
            }
        }
    }
}

pub fn draw_gui_buttons (img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>, gui: &Gui) {
    let buttons = TextFormat {
        size: Vec2::new(GUI_WIDTH as f64, GUI_HEIGHT as f64),
        font_size: 36.,
        ..Default::default()
    };

    let apply_pos = &gui.apply_hitbox().0;
    let cancel_pos = &gui.cancel_hitbox().0;

    draw_button_background(img, gui.apply_hitbox(), Rgba([70, 125, 70, 255]));
    draw_button_background(img, gui.cancel_hitbox(), Rgba([125, 70, 70, 255]));
    draw_text(img, apply_pos, "Apply".to_string(), &buttons);
    draw_text(img, cancel_pos, "Cancel".to_string(), &buttons);
}