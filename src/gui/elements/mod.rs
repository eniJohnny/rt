use fields::UIElement;

use crate::model::maths::vec2::Vec2;

use super::settings::UISettings;

pub mod fields;
pub mod ui;
pub mod window;

#[derive(Debug, Clone)]
pub enum Position {
    Inline,
    Bottom,
}

pub trait Displayable {
    fn get_fields(&self, settings: &UISettings) -> Vec<UIElement>;
}
