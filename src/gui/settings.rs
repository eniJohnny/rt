use rusttype::Font;

use crate::{
    model::{materials::color::Color, objects::light::AmbientLight}, ANTIALIASING, FIELD_HEIGHT, FIELD_PADDING_X, FIELD_PADDING_Y, BASE_FONT_SIZE, GUI_HEIGHT, GUI_WIDTH, INDENT_PADDING, MAX_DEPTH, MAX_ITERATIONS, MARGIN, VIEW_MODE
};

use super::elements::{
    fields::{Category, ElemType, Property, UIElement, Value},
    ui::UI,
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
