use image::{ImageBuffer, Rgba, RgbaImage};
use rusttype::{Font, Scale};
use winit::event::VirtualKeyCode;
use crate::{
    gui::{draw::{draw_cone_gui, draw_cylinder_gui, draw_plane_gui, draw_sphere_gui}, textformat::TextFormat, Gui},
    model::{maths::vec2::Vec2, objects::camera::Camera, shapes::Shape, Element}
};

pub fn move_camera(camera: &mut Camera, c: Option<VirtualKeyCode>) {

    match c {
        Some(VirtualKeyCode::W) => camera.move_forward(),
        Some(VirtualKeyCode::S) => camera.move_backward(),
        Some(VirtualKeyCode::A) => camera.move_left(),
        Some(VirtualKeyCode::D) => camera.move_right(),
        Some(VirtualKeyCode::Up) => camera.look_up(),
        Some(VirtualKeyCode::Down) => camera.look_down(),
        Some(VirtualKeyCode::Left) => camera.look_left(),
        Some(VirtualKeyCode::Right) => camera.look_right(),
        Some(VirtualKeyCode::LShift) => camera.move_up(),
        Some(VirtualKeyCode::Space) => camera.move_down(),
        _ => (),
    }
    // camera.debug_print();
}

pub fn display_element_infos(element: &Element, img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) -> Gui {
    let img = img;
    let shape = element.shape();
    let material = element.material();

    if shape.as_sphere().is_some() {
        let sphere = shape.as_sphere().unwrap();
        return draw_sphere_gui(img, sphere, material);
    } else if shape.as_plane().is_some() {
        let plane = shape.as_plane().unwrap();
        return draw_plane_gui(img, plane, material);
    } else if shape.as_cylinder().is_some() {
        let cylinder = shape.as_cylinder().unwrap();
        return draw_cylinder_gui(img, cylinder, material);
    } else if shape.as_cone().is_some() {
        return draw_cone_gui(img, shape.as_cone().unwrap(), material);
    } else {
        return Gui::new();
    }
}

pub fn draw_text(image: &mut RgbaImage, pos: &Vec2, text: String, format: &TextFormat) {
    let x = *pos.x() as u32 + 8;
    let y = *pos.y() as u32;

    // Load font
    let font_data = include_bytes!("../assets/JetBrainsMono-Regular.ttf");
    let font = &Font::try_from_bytes(font_data as &[u8]).expect("Error loading font");

    // Set font size and color
    let scale = Scale::uniform(format.font_size());
    let background_color = *format.background_color();
    let color = format.font_color();

    if background_color != Rgba([50, 50, 50, 255]) {
        draw_text_background(
            image,
            pos,
            format.size(),
            background_color,
            format.font_size() as u32,
        );
    }

    // Draw text
    let v_metrics = font.v_metrics(scale);
    let offset = rusttype::point(x as f32, y as f32 + v_metrics.ascent);

    for glyph in font.layout(&text, scale, offset) {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
                    let pixel = image.get_pixel_mut(x as u32, y as u32);
                    *pixel = blend(color, pixel, v);
                }
            });
        }
    }
}

// Blend function to combine text color with background color
pub fn blend(text_color: &Rgba<u8>, background_color: &Rgba<u8>, alpha: f32) -> Rgba<u8> {
    let inv_alpha = 1.0 - alpha;
    let r = (text_color[0] as f32 * alpha + background_color[0] as f32 * inv_alpha) as u8;
    let g = (text_color[1] as f32 * alpha + background_color[1] as f32 * inv_alpha) as u8;
    let b = (text_color[2] as f32 * alpha + background_color[2] as f32 * inv_alpha) as u8;
    let a = (text_color[3] as f32 * alpha + background_color[3] as f32 * inv_alpha) as u8;
    Rgba([r, g, b, a])
}

pub fn draw_text_background(
    image: &mut RgbaImage,
    pos: &Vec2,
    size: &Vec2,
    color: Rgba<u8>,
    height: u32,
) {
    let mut width = *size.x() as u32;

    if *pos.x() as u32 + width > image.width() as u32 {
        width = image.width() - *pos.x() as u32;
    }

    let x = image.width() - width;
    let y = *pos.y() as u32;

    for x in x..x + width {
        for y in y..y + height {
            image.put_pixel(x, y, color);
        }
    }
}

pub fn get_shape(shape: &dyn Shape) -> String {
    if let Some(sphere) = shape.as_sphere() {
        return format!(
            "Sphere: pos: {:#?}, radius: {}",
            sphere.pos(),
            sphere.radius()
        );
    } else if let Some(plane) = shape.as_plane() {
        return format!("Plane: pos: {:#?}, dir: {:#?}", plane.pos(), plane.dir());
    } else if let Some(cylinder) = shape.as_cylinder() {
        return format!(
            "Cylinder: pos: {:#?}, dir: {:#?}, radius: {}, height: {}",
            cylinder.pos(),
            cylinder.dir(),
            cylinder.radius(),
            cylinder.height()
        );
    } else if let Some(cone) = shape.as_cone() {
        return format!(
            "Cone: pos: {:#?}, dir: {:#?}, radius: {}, height: {}",
            cone.pos(),
            cone.dir(),
            cone.radius(),
            cone.height()
        );
    } else {
        return String::from("Unknown shape");
    }
}
