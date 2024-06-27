use ui::UI;
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
    Relative(i32, i32),
}
pub trait Displayable {
    fn get_fields(&self, reference: &String, ui: &UISettings) -> Vec<UIElement>;
}
