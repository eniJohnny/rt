use crate::{
    BASE_FONT_SIZE, FIELD_HEIGHT, FIELD_PADDING_X, FIELD_PADDING_Y, GUI_HEIGHT, GUI_WIDTH,
    INDENT_PADDING, MARGIN,
};

use super::elements::{
    uielement::{Category, ElemType, Property, UIElement, Value},
    Displayable, Position,
};

pub struct UISettings {
    pub margin: u32,
    pub field_height: u32,
    pub font_size: u32,
    pub padding_x: u32,
    pub padding_y: u32,
    pub indent_padding: u32,
    pub gui_height: u32,
    pub gui_width: u32,
}

impl UISettings {
    pub fn default() -> Self {
        Self {
            margin: MARGIN as u32,
            field_height: FIELD_HEIGHT as u32,
            gui_height: GUI_HEIGHT as u32,
            gui_width: GUI_WIDTH as u32,
            font_size: BASE_FONT_SIZE,
            padding_x: FIELD_PADDING_X,
            padding_y: FIELD_PADDING_Y,
            indent_padding: INDENT_PADDING,
        }
    }
}

impl Displayable for UISettings {
    fn get_fields(&self, reference: &String, settings: &UISettings) -> Vec<UIElement> {
        let mut fields = vec![];
        fields.push(UIElement::new(
            "Padding",
            reference.clone(),
            ElemType::Property(Property::new(
                Value::Unsigned(self.margin),
                Box::new(|value, _, ui| {
                    if let Value::Unsigned(value) = value {
                        ui.settings_mut().margin = value
                    }
                }),
                Box::new(|value| {
                    if let Value::Unsigned(value) = value {
                        if *value > 20 {
                            return Err("Too much padding is bad for your health");
                        }
                    }
                    Ok(())
                }),
                settings,
            )),
            Position::Inline,
            settings,
        ));

        fields
    }
}
