use super::{uielement::UIElement, utils::Displayable};
use crate::{
    ui::{
        uielement::Category,
        utils::misc::{ElemType, Property, Value}
    },
    BASE_FONT_SIZE, FIELD_PADDING_X, FIELD_PADDING_Y, GUI_HEIGHT, GUI_WIDTH, MARGIN, UI_REFRESH_TIME
};

#[derive(Clone)]
pub struct UISettings {
    pub margin: u32,
    pub font_size: u32,
    pub padding_x: u32,
    pub padding_y: u32,
    // pub indent_padding: u32,
    pub gui_height: u32,
    pub gui_width: u32,
    pub ui_refresh_time: u32,
}

impl UISettings {
    pub fn default() -> Self {
        Self {
            margin: MARGIN as u32,
            gui_height: GUI_HEIGHT as u32,
            gui_width: GUI_WIDTH as u32,
            font_size: BASE_FONT_SIZE,
            padding_x: FIELD_PADDING_X,
            padding_y: FIELD_PADDING_Y,
            // indent_padding: INDENT_PADDING,
            ui_refresh_time: UI_REFRESH_TIME,
        }
    }
}

impl Displayable for UISettings {
    fn get_fields(&self, name: &str, settings: &UISettings) -> Vec<UIElement> {
        let mut category = Category {
            collapsed: false,
            elems: vec![],
        };
        category.elems.push(UIElement::new(
            "Margin",
            "margin",
            ElemType::Property(Property::new(
                Value::Unsigned(self.margin),
                Box::new(|_, value, _, ui| {
                    if let Value::Unsigned(value) = value {
                        ui.uisettings_mut().margin = value
                    }
                }),
                Box::new(|value, _elem, _ui| {
                    if let Value::Unsigned(value) = value {
                        if value > &20 {
                            return Err("Too much margin is bad for your health".to_owned());
                        }
                    }
                    Ok(())
                }),
                settings,
            )),
            settings,
        ));

        category.elems.push(UIElement::new(
            "UI Refresh(in ms)",
            "refresh",
            ElemType::Property(Property::new(
                Value::Unsigned(self.ui_refresh_time),
                Box::new(|_, value, _, ui| {
                    if let Value::Unsigned(value) = value {
                        ui.uisettings_mut().ui_refresh_time = value
                    }
                }),
                Box::new(|value, _elem, _ui| {
                    if let Value::Unsigned(value) = value {
                        if value < &50 {
                            return Err("Don't be unreasonable".to_owned());
                        }
                    }
                    Ok(())
                }),
                settings,
            )),
            settings,
        ));

        category.elems.push(UIElement::new(
            "Display time",
            "display_time",
            ElemType::Stat(Box::new(|_, ui| {
                if let Some(context) = ui.context() {
                    return format!("{:.2}", context.draw_time_avg);
                }
                "".to_string()
            })),
            settings,
        ));

        vec![UIElement::new(
            name,
            "uisettings",
            ElemType::Category(category),
            settings,
        )]
    }
}
