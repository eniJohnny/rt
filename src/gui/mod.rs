use image::{Rgba, RgbaImage};

use crate::{display::draw_text, model::{maths::vec2::Vec2, scene, shapes::sphere}, render::cast_ray, SCREEN_WIDTH};

pub fn get_line_position (i: u32, size: &Vec2) -> Vec2 {
    let x = SCREEN_WIDTH as f64 - size.x();
    let y = i as f64 * 30.;

    Vec2::new(x, y)
}

pub fn hide_gui (img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>, scene: &scene::Scene) {
    let width = 400;
    let x_start = img.width() - width;
    let height = 800;

    let rays = scene.camera().get_rays();

    for x in x_start..img.width() {
        for y in 0..height {
            img.put_pixel(x, y, cast_ray(scene, &rays[x as usize][y as usize]).toRgba());
        }
    }
}

pub fn draw_sphere_gui (img: &mut image::ImageBuffer<Rgba<u8>, Vec<u8>>, sphere: &sphere::Sphere) {
    let height: u32 = 400;
    let width: u32 = 400;
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

    titles.parse_and_draw_text(img, 0, "Sphere", "");
    titles.parse_and_draw_text(img, 1, "Position", "");
    params.parse_and_draw_text(img, 2, " X:", &sphere.pos().x().to_string());
    params.parse_and_draw_text(img, 3, " Y:", &sphere.pos().y().to_string());
    params.parse_and_draw_text(img, 4, " Z:", &sphere.pos().z().to_string());
    titles.parse_and_draw_text(img, 5, "Misc", "");
    params.parse_and_draw_text(img, 6, " Radius:", &sphere.radius().to_string());

}

pub struct TextFormat {
    size: Vec2,
    font_size: f32,
    font_color: Rgba<u8>,
    background_color: Rgba<u8>,
}

impl Default for TextFormat {
    fn default() -> Self {
        TextFormat {
            size: Vec2::new(0., 0.),
            font_size: 24.,
            font_color: Rgba([255, 255, 255, 255]),
            background_color: Rgba([50, 50, 50, 255]),
        }
    }
}

impl TextFormat {
    pub fn size(&self) -> &Vec2 { &self.size }
    pub fn font_size(&self) -> f32 { self.font_size }
    pub fn font_color(&self) -> &Rgba<u8> { &self.font_color }
    pub fn background_color(&self) -> &Rgba<u8> { &self.background_color }

    pub fn get_spacer(&self, text: &str, value: &str) -> String {
        let text_len = text.len();
        let value_len = value.len();
        let char_width = 10;
        let char_num = (*self.size.x() as usize) / char_width;
        let spacer_len = char_num - text_len - value_len - 5;

        " ".repeat(spacer_len)
    }

    pub fn parse_and_draw_text(&mut self, img: &mut RgbaImage, i: u32, text: &str, value: &str) {
        let spacer = self.get_spacer(text, value);
        let pos = get_line_position(i, &self.size);

        if value == "" {
            draw_text(img, &pos, text.to_string(), self);
            return;
        }

        self.font_color = self.get_axis_color(text);
        draw_text(img, &pos, text.to_string(), self);

        let offset = (spacer.len() + text.len() + 3) as f64 * 10.0;
        let pos = Vec2::new(pos.x() + offset, *pos.y());
        self.font_color = Rgba([255, 255, 255, 255]);
        draw_text(img, &pos, value.to_string(), self);
    }

    pub fn get_axis_color(&self, text: &str) -> Rgba<u8> {
        match text {
            " X:" => Rgba([255, 150, 150, 255]),
            " Y:" => Rgba([150, 255, 150, 255]),
            " Z:" => Rgba([150, 150, 255, 255]),
            _ => Rgba([255, 255, 255, 255]),
        }
    }
}