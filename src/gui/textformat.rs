use image::{Rgba, RgbaImage};

use crate::{
    display::utils::draw_text, model::maths::vec2::Vec2, FIELD_PADDING_X, FIELD_PADDING_Y,
    GUI_HEIGHT, GUI_WIDTH,
};

use super::{settings, uisettings::UISettings, utils::get_line_position};

pub struct TextFormat {
    size: Vec2,
    pub font_size: f32,
    font_color: Rgba<u8>,
    background_color: Rgba<u8>,
    pub width: u32,
    pub height: u32,
    pub bg_color: Option<Rgba<u8>>,
    pub border_radius: u32,
    pub padding_left: u32,
    pub padding_right: u32,
    pub padding_top: u32,
    pub padding_bot: u32,
    pub margin_left: u32,
}

pub struct FormatBuilder {
    format: TextFormat,
    settings: UISettings,
}

impl FormatBuilder {
    pub fn default(settings: &UISettings) -> Self {
        Self {
            settings: settings.clone(),
            format: TextFormat::base_format(settings)
        }   
    }
    pub fn from_btn(settings: &UISettings) -> Self {
        FormatBuilder::default(settings)
            .bg_color(Some(Rgba([200, 200, 200, 255])))
            .font_color(Rgba([0, 0, 0, 255]))
            .border_radius(3)
            .margin_left(30)
    }
    pub fn padding_left(mut self, padding_left: u32) -> Self {
        self.format.padding_left = padding_left;
        self
    }
    pub fn padding_right(mut self, padding_right: u32) -> Self {
        self.format.padding_right = padding_right;
        self
    }
    pub fn padding_top(mut self, padding_top: u32) -> Self {
        self.format.padding_top = padding_top;
        self
    }
    pub fn padding_bot(mut self, padding_bot: u32) -> Self {
        self.format.padding_bot = padding_bot;
        self
    }
    pub fn margin_left(mut self, margin_left: u32) -> Self {
        self.format.margin_left = margin_left;
        self
    }
    pub fn border_radius(mut self, border_radius: u32) -> Self {
        self.format.border_radius = border_radius;
        self
    }
    pub fn width(mut self, width: u32) -> Self {
        self.format.width = width;
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.format.height = height;
        self
    }
    pub fn bg_color(mut self, bg_color: Option<Rgba<u8>>) -> Self {
        self.format.bg_color = bg_color;
        self
    }
    pub fn font_color(mut self, font_color: Rgba<u8>) -> Self {
        self.format.font_color = font_color;
        self
    }
    pub fn build(self) -> TextFormat {
        self.format
    }
}

impl Default for TextFormat {
    fn default() -> Self {
        TextFormat {
            size: Vec2::new(0., 0.),
            width: 0,
            height: 0,
            font_size: 24.,
            font_color: Rgba([255, 255, 255, 255]),
            background_color: Rgba([50, 50, 50, 255]),
            bg_color: None,
            border_radius: 0,
            margin_left: 0,
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
        bg_color: Option<Rgba<u8>>,
        padding_left: u32,
        padding_right: u32,
        padding_top: u32,
        padding_bot: u32,
        margin_left: u32
    ) -> Self {
        Self {
            size,
            width: 0,
            height: 0,
            font_size,
            font_color,
            background_color: Rgba([0, 0, 0, 0]),
            bg_color: bg_color,
            border_radius: 0,
            padding_left,
            padding_right,
            padding_top,
            padding_bot,
            margin_left
        }
    }

    pub fn base_format(settings: &UISettings) -> Self {
        Self {
            size: Vec2::new(0., 0.),
            width: 0,
            height: 0,
            font_size: settings.font_size as f32,
            font_color: Rgba([255, 255, 255, 255]),
            background_color: Rgba([89, 89, 89, 255]),
            bg_color: Some(Rgba([89, 89, 89, 255])),
            border_radius: 0,
            padding_left: settings.padding_x,
            padding_right: settings.padding_x,
            padding_top: settings.padding_y,
            padding_bot: settings.padding_y,
            margin_left: 0
        }
    }

    pub fn field_format(settings: &UISettings) -> Self {
        FormatBuilder::default(settings)
            .width(settings.gui_width).build()
    }

    pub fn new_editing_format(settings: &UISettings) -> Self {
        FormatBuilder::default(settings)
            .bg_color(Some(Rgba([255, 255, 255, 255])))
            .font_color(Rgba([0, 0, 0, 255]))
            .build()
    }

    pub fn new_category_format(settings: &UISettings) -> Self {
        FormatBuilder::default(settings)
            .width(settings.gui_width)
            .bg_color(Some(Rgba([40, 40, 40, 255])))
            .font_color(Rgba([200, 200, 200, 255]))
            .build()
    }

    pub fn new_btn_format(settings: &UISettings) -> Self {
        FormatBuilder::from_btn(settings).build()
    }

    pub fn new_btn_apply_format(settings: &UISettings) -> Self {
        FormatBuilder::from_btn(settings)
            .bg_color(Some(Rgba([70, 125, 70, 255])))
            .margin_left(0)
            .build()
    }

    pub fn new_btn_cancel_format(settings: &UISettings) -> Self {
        FormatBuilder::from_btn(settings)
            .bg_color(Some(Rgba([125, 70, 70, 255])))
            .margin_left(0)
            .build()
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
