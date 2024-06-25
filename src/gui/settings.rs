use crate::{
    model::{materials::color::Color, objects::light::AmbientLight},
    ANTIALIASING, FIELD_HEIGHT, GUI_HEIGHT, GUI_WIDTH, MAX_DEPTH, MAX_ITERATIONS, PADDING,
    VIEW_MODE,
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

// impl Displayable for ViewMode {
//     fn get_fields(&self, settings: &UISettings) -> Vec<UIElement> {
//         let vec = vec![];
//         let simple_view = UIElement::new(
//             "simple_view",
//             ElemType::Property(Property {
//                 value: Value::Bool(settings.view_mode),
//                 on_change: Box::new(|value, scene, ui| {
//                     if let Value::Bool(on) = value {
//                         ui.settings_mut().view_mode = ViewMode::Simple;
//                     }
//                 }),
//             }),
//             Position::Inline,
//         );
//         vec.push(simple_view);
//         vec
//     }
// }

pub struct UISettings {
    pub padding: usize,
    pub field_height: usize,
    pub gui_height: usize,
    pub gui_width: usize,
}

impl UISettings {
    pub fn default() -> Self {
        Self {
            padding: PADDING,
            field_height: FIELD_HEIGHT,
            gui_height: GUI_HEIGHT as usize,
            gui_width: GUI_WIDTH as usize,
        }
    }
}

impl Displayable for UISettings {
    fn get_fields(&self, settings: &UISettings) -> Vec<UIElement> {
        let mut fields = vec![];
        fields.push(UIElement::new(
            "padding",
            ElemType::Property(Property {
                value: Value::Usize(self.padding),
                on_change: Box::new(|value, scene, ui| {
                    if let Value::Usize(value) = value {
                        ui.settings_mut().padding = value
                    }
                }),
            }),
            Position::Inline,
        ));

        fields
    }
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

// impl Displayable for Settings {
//     fn get_fields(&self, settings: &UISettings) -> Vec<UIElement> {
//         let vec = vec![UIElement::new("", ElemType::Category(), pos)];
//     }
// }
