use ui::UI;
use uielement::UIElement;

use crate::model::maths::vec2::Vec2;

use super::{textformat::Style, uisettings::UISettings};

pub mod ui;
pub mod uibox;
pub mod uieditbar;
pub mod uielement;
pub mod utils;

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
    fn get_fields(&self, reference: &String, ui: &UISettings) -> Vec<UIElement>;
}
