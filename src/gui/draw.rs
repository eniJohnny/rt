use std::sync::{Arc, RwLock};

use image::{Rgba, RgbaImage};

use crate::{
    display::utils::{display_element_infos, display_light_infos, draw_text},
    model::{
        materials::{color::Color, material::Material, texture::Texture},
        maths::vec2::Vec2,
        objects::light::{Light, PointLight},
        scene::Scene,
        shapes::{cone, cylinder, plane, sphere},
        Element,
    },
    GUI_HEIGHT, GUI_WIDTH, SCREEN_HEIGHT_U32, SCREEN_WIDTH, SCREEN_WIDTH_U32,
};

use super::{
    elements::{uibox::UIBox, Position},
    textformat::TextFormat,
    Gui,
};

pub fn draw_sphere_gui(
    img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    sphere: &sphere::Sphere,
    material: &dyn Material,
) -> Gui {
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

    let mut titles = TextFormat::default();
    let mut params = TextFormat::default();

    titles.set_size(size.clone());
    params.set_size(size.clone());
    params.set_background_color(Rgba([89, 89, 89, 255]));

    let mut gui = Gui::new();
    let color = material.color();
    let metalness = material.metalness();
    let roughness = material.roughness();
    let refraction = material.refraction();
    let emissive = material.emissive();
    let norm = material.norm();

    titles.parse_and_draw_text(img, 0, "Sphere", "");
    titles.parse_and_draw_text(img, 1, "Position:", "");
    gui.keys.push("posx".to_string());
    gui.values.push(sphere.pos().x().to_string());
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 2, " X:", &sphere.pos().x().to_string()));

    gui.keys.push("posy".to_string());
    gui.values.push(sphere.pos().y().to_string());
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 3, " Y:", &sphere.pos().y().to_string()));

    gui.keys.push("posz".to_string());
    gui.values.push(sphere.pos().z().to_string());
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 4, " Z:", &sphere.pos().z().to_string()));

    titles.parse_and_draw_text(img, 5, "Direction:", "");

    gui.keys.push("dirx".to_string());
    gui.values.push(sphere.dir().x().to_string());
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 6, " X:", &sphere.dir().x().to_string()));

    gui.keys.push("diry".to_string());
    gui.values.push(sphere.dir().y().to_string());
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 7, " Y:", &sphere.dir().y().to_string()));

    gui.keys.push("dirz".to_string());
    gui.values.push(sphere.dir().z().to_string());
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 8, " Z:", &sphere.dir().z().to_string()));

    titles.parse_and_draw_text(img, 9, "Material:", "");
    match color {
        Texture::Texture(file, _) => {
            gui.keys.push("color".to_string());
            gui.values.push(file.clone());
            gui.hitboxes
                .push(params.parse_and_draw_text(img, 10, " R:", &file));
        }
        Texture::Value(vec, _) => {
            let color = Color::from_vec3(&vec);
            gui.keys.push("colr".to_string());
            gui.keys.push("colg".to_string());
            gui.keys.push("colb".to_string());

            gui.values.push((color.r() * 255.).to_string());
            gui.values.push((color.g() * 255.).to_string());
            gui.values.push((color.b() * 255.).to_string());

            gui.hitboxes.push(params.parse_and_draw_text(
                img,
                10,
                " R:",
                &format!("{:.0}", color.r() * 255.),
            ));
            gui.hitboxes.push(params.parse_and_draw_text(
                img,
                11,
                " G:",
                &format!("{:.0}", color.g() * 255.),
            ));
            gui.hitboxes.push(params.parse_and_draw_text(
                img,
                12,
                " B:",
                &format!("{:.0}", color.b() * 255.),
            ));
        }
    }

    let metalness_str = metalness.to_string();
    gui.keys.push("metalness".to_string());
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 13, " Metalness:", &metalness_str));
    gui.values.push(metalness_str);

    let roughness_str = metalness.to_string();
    gui.keys.push("roughness".to_string());
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 14, " Roughness:", &roughness_str));
    gui.values.push(roughness_str);

    titles.parse_and_draw_text(img, 15, "Misc:", "");

    gui.keys.push("radius".to_string());
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        16,
        " Radius:",
        &sphere.radius().to_string(),
    ));
    gui.values.push(sphere.radius().to_string());

    draw_gui_buttons(img, &gui);

    gui
}

pub fn draw_cylinder_gui(
    img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    cylinder: &cylinder::Cylinder,
    material: &dyn Material,
) -> Gui {
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

    let mut titles = TextFormat::default();
    let mut params = TextFormat::default();

    titles.set_size(size.clone());
    params.set_size(size.clone());
    params.set_background_color(Rgba([89, 89, 89, 255]));

    let mut gui = Gui::new();
    let color = Color::new(0., 0., 0.);
    let metalness = material.metalness().to_string();
    let roughness = material.roughness().to_string();

    gui.keys.push("posx".to_string());
    gui.keys.push("posy".to_string());
    gui.keys.push("posz".to_string());
    gui.keys.push("dirx".to_string());
    gui.keys.push("diry".to_string());
    gui.keys.push("dirz".to_string());
    gui.keys.push("colr".to_string());
    gui.keys.push("colg".to_string());
    gui.keys.push("colb".to_string());
    gui.keys.push("metalness".to_string());
    gui.keys.push("roughness".to_string());
    gui.keys.push("radius".to_string());
    gui.keys.push("height".to_string());

    gui.values.push(cylinder.pos().x().to_string());
    gui.values.push(cylinder.pos().y().to_string());
    gui.values.push(cylinder.pos().z().to_string());
    gui.values.push(cylinder.dir().x().to_string());
    gui.values.push(cylinder.dir().y().to_string());
    gui.values.push(cylinder.dir().z().to_string());
    gui.values.push((color.r() * 255.).to_string());
    gui.values.push((color.g() * 255.).to_string());
    gui.values.push((color.b() * 255.).to_string());
    gui.values.push(metalness.to_string());
    gui.values.push(roughness.to_string());
    gui.values.push(cylinder.radius().to_string());
    gui.values.push(cylinder.height().to_string());

    titles.parse_and_draw_text(img, 0, "Cylinder", "");
    titles.parse_and_draw_text(img, 1, "Position:", "");
    titles.parse_and_draw_text(img, 5, "Direction:", "");
    titles.parse_and_draw_text(img, 9, "Material:", "");
    titles.parse_and_draw_text(img, 15, "Misc:", "");

    gui.hitboxes
        .push(params.parse_and_draw_text(img, 2, " X:", &cylinder.pos().x().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 3, " Y:", &cylinder.pos().y().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 4, " Z:", &cylinder.pos().z().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 6, " X:", &cylinder.dir().x().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 7, " Y:", &cylinder.dir().y().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 8, " Z:", &cylinder.dir().z().to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        10,
        " R:",
        &format!("{:.0}", color.r() * 255.),
    ));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        11,
        " G:",
        &format!("{:.0}", color.g() * 255.),
    ));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        12,
        " B:",
        &format!("{:.0}", color.b() * 255.),
    ));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 13, " Metalness:", &metalness.to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 14, " Roughness:", &roughness.to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        16,
        " Radius:",
        &cylinder.radius().to_string(),
    ));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        17,
        " Height:",
        &cylinder.height().to_string(),
    ));

    draw_gui_buttons(img, &gui);

    gui
}

pub fn draw_plane_gui(
    img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    plane: &plane::Plane,
    material: &dyn Material,
) -> Gui {
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

    let mut titles = TextFormat::default();
    let mut params = TextFormat::default();

    titles.set_size(size.clone());
    params.set_size(size.clone());
    params.set_background_color(Rgba([89, 89, 89, 255]));

    let mut gui = Gui::new();
    let color = Color::new(0., 0., 0.);
    let metalness = material.metalness().to_string();
    let roughness = material.roughness().to_string();

    gui.keys.push("posx".to_string());
    gui.keys.push("posy".to_string());
    gui.keys.push("posz".to_string());
    gui.keys.push("dirx".to_string());
    gui.keys.push("diry".to_string());
    gui.keys.push("dirz".to_string());
    gui.keys.push("colr".to_string());
    gui.keys.push("colg".to_string());
    gui.keys.push("colb".to_string());
    gui.keys.push("metalness".to_string());
    gui.keys.push("roughness".to_string());

    gui.values.push(plane.pos().x().to_string());
    gui.values.push(plane.pos().y().to_string());
    gui.values.push(plane.pos().z().to_string());
    gui.values.push(plane.dir().x().to_string());
    gui.values.push(plane.dir().y().to_string());
    gui.values.push(plane.dir().z().to_string());
    gui.values.push((color.r() * 255.).to_string());
    gui.values.push((color.g() * 255.).to_string());
    gui.values.push((color.b() * 255.).to_string());
    gui.values.push(metalness.to_string());
    gui.values.push(roughness.to_string());

    titles.parse_and_draw_text(img, 0, "Plane", "");
    titles.parse_and_draw_text(img, 1, "Position:", "");
    titles.parse_and_draw_text(img, 5, "Direction:", "");
    titles.parse_and_draw_text(img, 9, "Material:", "");

    gui.hitboxes
        .push(params.parse_and_draw_text(img, 2, " X:", &plane.pos().x().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 3, " Y:", &plane.pos().y().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 4, " Z:", &plane.pos().z().to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        6,
        " X:",
        &format!("{:.3}", plane.dir().x()),
    ));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        7,
        " Y:",
        &format!("{:.3}", plane.dir().y()),
    ));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        8,
        " Z:",
        &format!("{:.3}", plane.dir().z()),
    ));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        10,
        " R:",
        &format!("{:.0}", color.r() * 255.),
    ));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        11,
        " G:",
        &format!("{:.0}", color.g() * 255.),
    ));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        12,
        " B:",
        &format!("{:.0}", color.b() * 255.),
    ));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 13, " Metalness:", &metalness.to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 14, " Roughness:", &roughness.to_string()));

    draw_gui_buttons(img, &gui);

    gui
}

pub fn draw_cone_gui(
    img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    cone: &cone::Cone,
    material: &dyn Material,
) -> Gui {
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

    let mut titles = TextFormat::default();
    let mut params = TextFormat::default();

    titles.set_size(size.clone());
    params.set_size(size.clone());
    params.set_background_color(Rgba([89, 89, 89, 255]));

    let mut gui = Gui::new();
    let color = Color::new(0., 0., 0.);
    let metalness = material.metalness().to_string();
    let roughness = material.roughness().to_string();

    gui.keys.push("posx".to_string());
    gui.keys.push("posy".to_string());
    gui.keys.push("posz".to_string());
    gui.keys.push("dirx".to_string());
    gui.keys.push("diry".to_string());
    gui.keys.push("dirz".to_string());
    gui.keys.push("colr".to_string());
    gui.keys.push("colg".to_string());
    gui.keys.push("colb".to_string());
    gui.keys.push("metalness".to_string());
    gui.keys.push("roughness".to_string());
    gui.keys.push("radius".to_string());
    gui.keys.push("height".to_string());

    gui.values.push(cone.pos().x().to_string());
    gui.values.push(cone.pos().y().to_string());
    gui.values.push(cone.pos().z().to_string());
    gui.values.push(cone.dir().x().to_string());
    gui.values.push(cone.dir().y().to_string());
    gui.values.push(cone.dir().z().to_string());
    gui.values.push((color.r() * 255.).to_string());
    gui.values.push((color.g() * 255.).to_string());
    gui.values.push((color.b() * 255.).to_string());
    gui.values.push(metalness.to_string());
    gui.values.push(roughness.to_string());
    gui.values.push(cone.radius().to_string());
    gui.values.push(cone.height().to_string());

    titles.parse_and_draw_text(img, 0, "Cone", "");
    titles.parse_and_draw_text(img, 1, "Position:", "");
    titles.parse_and_draw_text(img, 5, "Direction:", "");
    titles.parse_and_draw_text(img, 9, "Material:", "");
    titles.parse_and_draw_text(img, 15, "Misc:", "");

    gui.hitboxes
        .push(params.parse_and_draw_text(img, 2, " X:", &cone.pos().x().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 3, " Y:", &cone.pos().y().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 4, " Z:", &cone.pos().z().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 6, " X:", &cone.dir().x().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 7, " Y:", &cone.dir().y().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 8, " Z:", &cone.dir().z().to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        10,
        " R:",
        &format!("{:.0}", color.r() * 255.),
    ));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        11,
        " G:",
        &format!("{:.0}", color.g() * 255.),
    ));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        12,
        " B:",
        &format!("{:.0}", color.b() * 255.),
    ));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 13, " Metalness:", &metalness.to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 14, " Roughness:", &roughness.to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 16, " Radius:", &cone.radius().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 17, " Height:", &cone.height().to_string()));

    draw_gui_buttons(img, &gui);

    gui
}

pub fn draw_pointlight_gui(
    img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    pointlight: &PointLight,
) -> Gui {
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

    let mut titles = TextFormat::default();
    let mut params = TextFormat::default();

    titles.set_size(size.clone());
    params.set_size(size.clone());
    params.set_background_color(Rgba([89, 89, 89, 255]));

    let mut gui = Gui::new();

    gui.keys.push("posx".to_string());
    gui.keys.push("posy".to_string());
    gui.keys.push("posz".to_string());
    gui.keys.push("intensity".to_string());
    gui.keys.push("colorr".to_string());
    gui.keys.push("colorg".to_string());
    gui.keys.push("colorb".to_string());

    let pos = pointlight.pos();
    let intensity = pointlight.intensity();
    let color = pointlight.color();

    gui.values.push(pos.x().to_string());
    gui.values.push(pos.y().to_string());
    gui.values.push(pos.z().to_string());
    gui.values.push(intensity.to_string());
    gui.values.push((color.r() * 255.).to_string());
    gui.values.push((color.g() * 255.).to_string());
    gui.values.push((color.b() * 255.).to_string());

    titles.parse_and_draw_text(img, 0, "Light", "");
    titles.parse_and_draw_text(img, 1, "Position:", "");
    titles.parse_and_draw_text(img, 5, "Intensity:", "");
    titles.parse_and_draw_text(img, 9, "Color:", "");

    gui.hitboxes
        .push(params.parse_and_draw_text(img, 2, " X:", &pos.x().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 3, " Y:", &pos.y().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 4, " Z:", &pos.z().to_string()));
    gui.hitboxes
        .push(params.parse_and_draw_text(img, 6, " Intensity:", &intensity.to_string()));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        10,
        " R:",
        &format!("{:.0}", color.r() * 255.),
    ));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        11,
        " G:",
        &format!("{:.0}", color.g() * 255.),
    ));
    gui.hitboxes.push(params.parse_and_draw_text(
        img,
        12,
        " B:",
        &format!("{:.0}", color.b() * 255.),
    ));

    draw_gui_buttons(img, &gui);

    gui
}

fn is_corner(x: u32, y: u32, x_start: u32, y_start: u32, x_end: u32, y_end: u32, border_radius: u32) -> bool {
    if x < x_start + border_radius && y < y_start + border_radius {
        return true;
    }
    if x < x_start + border_radius && y > y_end - border_radius {
        return true;
    }
    if x > x_end - border_radius && y < y_start + border_radius {
        return true;
    }
    if x > x_end - border_radius && y > y_end - border_radius {
        return true;
    }

    false
}

pub fn draw_background(
    img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    pos: (u32, u32),
    size: (u32, u32),
    color: Rgba<u8>,
    border_radius: u32
) {
    let x_start = pos.0;
    let x_end = (pos.0 + size.0).min(SCREEN_WIDTH_U32 - 1);
    let y_start = pos.1;
    let y_end = (pos.1 + size.1).min(SCREEN_HEIGHT_U32 - 1);

    for x in x_start..x_end {
        for y in y_start..y_end {
            if is_corner(x, y, x_start, y_start, x_end, y_end, border_radius) == false {
                img.put_pixel(x, y, color);
            }
        }
    }
}

pub fn draw_button_background(
    img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    hitbox: &(Vec2, Vec2),
    color: Rgba<u8>,
) {
    let upper_left_corner = &hitbox.0;
    let lower_right_corner = &hitbox.1;

    let x_start = *upper_left_corner.x() as u32;
    let x_end = *lower_right_corner.x() as u32;
    let y_start = *upper_left_corner.y() as u32;
    let y_end = *lower_right_corner.y() as u32;

    for x in x_start..x_end {
        for y in y_start..y_end {
            if is_corner(x, y, x_start, y_start, x_end, y_end, 2) == false {
                img.put_pixel(x, y, color);
            }
        }
    }
}

pub fn draw_gui_buttons(img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>, gui: &Gui) {
    let mut info_format = TextFormat::default();
    let mut buttons_format = TextFormat::default();
    buttons_format.set_size(Vec2::new(GUI_WIDTH as f64, GUI_HEIGHT as f64));
    buttons_format.set_font_size(36.);
    info_format.set_font_size(20.);

    let apply_pos = &gui.apply_hitbox().0;
    let cancel_pos = &gui.cancel_hitbox().0;
    let info_msg_pos_l1 = Vec2::new(1230., 480.);
    let info_msg_pos_l2 = Vec2::new(1260., 500.);

    draw_button_background(img, gui.apply_hitbox(), Rgba([70, 125, 70, 255]));
    draw_button_background(img, gui.cancel_hitbox(), Rgba([125, 70, 70, 255]));
    draw_text(img, apply_pos, "Apply".to_string(), &buttons_format);
    draw_text(img, cancel_pos, "Cancel".to_string(), &buttons_format);
    draw_text(
        img,
        &info_msg_pos_l1,
        "Please wait for the image to finish".to_string(),
        &info_format,
    );
    draw_text(
        img,
        &info_msg_pos_l2,
        "loading before making changes".to_string(),
        &info_format,
    );
}

pub fn draw_gui(
    img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    element: Option<&Element>,
    light: Option<&Box<dyn Light + Sync + Send>>,
    index: usize,
) -> Gui {
    let is_element = element.is_some();

    if is_element == true {
        let element = element.unwrap();
        let mut gui = display_element_infos(element, img);

        gui.set_element_index(index);
        gui.set_is_open(true);
        gui.set_displaying(&"element".to_string());

        gui
    } else {
        let light = light.unwrap();
        let mut gui = display_light_infos(light, img);

        gui.set_light_index(index as i32);
        gui.set_is_open(true);
        gui.set_displaying(&"light".to_string());

        println!("Light index: {}", index);

        gui
    }
}

pub fn draw_sub_window(img: &mut RgbaImage, scene: &Arc<RwLock<Scene>>, win: UIBox) {
    let scene = scene.read().unwrap();

    // if win.visible {
    //     if let Position::Absolute(pos, _) = win.pos {}
    // }
}
