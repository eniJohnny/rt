use ui_utils::UIContext;

use super::{uielement::UIElement, uisettings::UISettings};

pub mod draw_utils;
pub mod misc;
pub mod style;
pub mod ui_utils;

#[derive(Debug, Clone)]

pub struct HitBox {
    pub reference: String,
    pub pos: (u32, u32),
    pub size: (u32, u32),
    pub disabled: bool,
}

impl HitBox {
    pub fn x(&self) -> u32 {
        self.pos.0
    }
    pub fn y(&self) -> u32 {
        self.pos.1
    }
    pub fn width(&self) -> u32 {
        self.size.0
    }
    pub fn height(&self) -> u32 {
        self.size.1
    }
}

pub trait Displayable {
    fn get_fields(&self, name: &str, context: &UIContext, ui: &UISettings) -> Vec<UIElement>;
}
