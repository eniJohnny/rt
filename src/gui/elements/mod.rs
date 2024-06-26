use uielement::UIElement;

use crate::model::maths::vec2::Vec2;

use super::uisettings::UISettings;

pub mod ui;
pub mod uibox;
pub mod uielement;

#[derive(Debug, Clone)]
pub enum Position {
    Inline,
    Bottom,
    Relative(u32, u32),
}

pub trait Displayable {
    fn get_fields(&self, reference: &String, settings: &UISettings) -> Vec<UIElement>;
}
