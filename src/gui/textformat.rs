use image::{Rgba, RgbaImage};

use crate::{display::utils::draw_text, model::maths::vec2::Vec2, FIELD_PADDING_X, FIELD_PADDING_Y, GUI_HEIGHT, GUI_WIDTH};

use super::{uisettings::UISettings, utils::get_line_position};


pub struct TextFormat {
    size: Vec2,
    font_size: f32,
    font_color: Rgba<u8>,
    background_color: Rgba<u8>,
    pub padding_left: u32,
    pub padding_right: u32,
    pub padding_top: u32, 
    pub padding_bot: u32
}

impl Default for TextFormat {
    fn default() -> Self {
        TextFormat {
            size: Vec2::new(0., 0.),
            font_size: 24.,
            font_color: Rgba([255, 255, 255, 255]),
            background_color: Rgba([50, 50, 50, 255]),
            padding_left: FIELD_PADDING_X,
            padding_right: FIELD_PADDING_X,
            padding_bot: FIELD_PADDING_Y,
            padding_top: FIELD_PADDING_Y,
        }
    }
}

pub trait Formattable {
    fn base_format(&self, settings: &UISettings) -> TextFormat {
        TextFormat::default()
    }
}

impl TextFormat {
    pub fn size(&self) -> &Vec2 {
        &self.size
    }
    pub fn font_size(&self) -> f32 {
        self.font_size
    }
    pub fn font_color(&self) -> &Rgba<u8> {
        &self.font_color
    }
    pub fn background_color(&self) -> &Rgba<u8> {
        &self.background_color
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }
    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
    }
    pub fn set_font_color(&mut self, font_color: Rgba<u8>) {
        self.font_color = font_color;
    }
    pub fn set_background_color(&mut self, background_color: Rgba<u8>) {
        self.background_color = background_color;
    }

    pub fn new(
        size: Vec2,
        font_size: f32,
        font_color: Rgba<u8>,
        background_color: Rgba<u8>,
        padding_left: u32,
        padding_right: u32,
        padding_top: u32, 
        padding_bot: u32
    ) -> Self {
        Self {
            size,
            font_size,
            font_color,
            background_color,
            padding_left,
            padding_right,
            padding_top, 
            padding_bot
        }
    }

    pub fn new_base_format(settings: &UISettings) -> Self {
        Self {
            size: Vec2::new(settings.gui_width as f64, settings.gui_height as f64),
            font_size: settings.font_size as f32,
            font_color: Rgba([255, 255, 255, 255]),
            background_color: Rgba([89, 89, 89, 255]),
            padding_left: settings.padding_x,
            padding_right: settings.padding_x,
            padding_top: settings.padding_y,
            padding_bot: settings.padding_y,
        }
    }
    pub fn new_editing_format(settings: &UISettings) -> Self {
        Self {
            size: Vec2::new(400., 400.),
            font_size: settings.font_size as f32,
            font_color: Rgba([0, 0, 0, 255]),
            background_color: Rgba([255, 255, 255, 255]),
            padding_left: settings.padding_x,
            padding_right: settings.padding_x,
            padding_top: settings.padding_y,
            padding_bot: settings.padding_y,
        }
    }

    pub fn get_spacer(&self, text: &str, value: &str) -> String {
        let text_len = text.len();
        let value_len = value.len();
        let char_width = 10;
        let char_num = (*self.size.x() as usize) / char_width;
        let spacer_len = char_num - text_len - value_len - 5;

        " ".repeat(spacer_len)
    }

    pub fn parse_and_draw_text(
        &mut self,
        img: &mut RgbaImage,
        i: u32,
        text: &str,
        value: &str,
    ) -> (Vec2, Vec2) {
        let spacer = self.get_spacer(text, value);
        let pos = get_line_position(i, &self.size);

        if value == "" {
            draw_text(img, &pos, text.to_string(), self);
            return (Vec2::new(0., 0.), Vec2::new(0., 0.));
        }

        self.font_color = self.get_axis_color(text);
        draw_text(img, &pos, text.to_string(), self);

        let offset = (spacer.len() + text.len() + 3) as f64 * 10.0;
        let pos = Vec2::new(pos.x() + offset, *pos.y());
        self.font_color = Rgba([255, 255, 255, 255]);
        draw_text(img, &pos, value.to_string(), self);

        let end_pos = Vec2::new(pos.x() + (value.len() + 1) as f64 * 10.0, *pos.y() + 26.);
        (pos, end_pos)
    }

    pub fn get_axis_color(&self, text: &str) -> Rgba<u8> {
        match text {
            " X:" | " R:" => Rgba([255, 150, 150, 255]),
            " Y:" | " G:" => Rgba([150, 255, 150, 255]),
            " Z:" | " B:" => Rgba([150, 150, 255, 255]),
            _ => Rgba([255, 255, 255, 255]),
        }
    }
}
