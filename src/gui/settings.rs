use rusttype::Font;

use crate::{
    model::{materials::color::Color, objects::light::AmbientLight},
    ANTIALIASING, BASE_FONT_SIZE, FIELD_HEIGHT, FIELD_PADDING_X, FIELD_PADDING_Y, GUI_HEIGHT,
    GUI_WIDTH, INDENT_PADDING, MARGIN, MAX_DEPTH, MAX_ITERATIONS, VIEW_MODE,
};

use super::elements::{
    ui::UI,
    uielement::{Category, ElemType, Property, UIElement, Value},
    Displayable, Position,
};

#[derive(Debug)]
pub enum ViewMode {
    Simple,
    Norm,
    HighDef,
    BVH,
}

#[derive(Debug)]
pub struct Settings {
    pub reflections: bool,
    pub indirect: bool,
    pub iterations: usize,
    pub depth: usize,
    pub anti_alisaing: f64,
    pub view_mode: ViewMode,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            view_mode: VIEW_MODE,
            reflections: true,
            indirect: true,
            iterations: MAX_ITERATIONS,
            depth: MAX_DEPTH,
            anti_alisaing: ANTIALIASING,
        }
    }
}
