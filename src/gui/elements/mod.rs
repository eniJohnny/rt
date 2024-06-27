use ui::UI;
use uielement::UIElement;

use crate::model::maths::vec2::Vec2;

use super::{textformat::TextFormat, uisettings::UISettings};

pub mod ui;
pub mod uibox;
pub mod uielement;
pub mod uieditbar;

#[derive(Debug, Clone)]
pub enum Position {
    Inline,
    Relative(i32, i32),
}
pub trait Displayable {
    fn get_fields(&self, reference: &String, ui: &UISettings) -> Vec<UIElement>;
}
