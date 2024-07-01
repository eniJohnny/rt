use crate::{
    ui::elements::{uielement::Category, utils::{ElemType, Property, Value}}, BASE_FONT_SIZE, FIELD_PADDING_X, FIELD_PADDING_Y, GUI_HEIGHT, GUI_WIDTH, INDENT_PADDING, MARGIN, SCREEN_HEIGHT_U32, SCREEN_WIDTH_U32
};

use super::elements::{
    uielement::UIElement, Displayable
};

#[derive(Clone)]
pub struct UISettings {
    pub margin: u32,
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
        let mut category = Category {
            collapsed: false,
            elems: vec![],
        };
        category.elems.push(UIElement::new(
            "Margin",
            "margin",
            ElemType::Property(Property::new(
                Value::Unsigned(self.margin),
                Box::new(|value, _, ui| {
                    if let Value::Unsigned(value) = value {
                        ui.uisettings_mut().margin = value
                    }
                }),
                Box::new(|value| {
                    if let Value::Unsigned(value) = value {
                        if value > &20 {
                            return Err("Too much padding is bad for your health");
                        }
                    }
                    Ok(())
                }),
                settings,
            )),
            settings,
        ));

        category.elems.push(UIElement::new(
            "UI Width",
            "uiwidth",
            ElemType::Property(Property::new(
                Value::Unsigned(self.gui_width),
                Box::new(|value, _, ui| {
                    if let Value::Unsigned(value) = value {
                        ui.uisettings_mut().gui_width = value;
                        let uibox = ui.get_box_mut("uisettings".to_string());
                        uibox.pos = (SCREEN_WIDTH_U32 - value, uibox.pos.1);
                    }
                }),
                Box::new(|value| {
                    if let Value::Unsigned(value) = value {
                        if *value > 800 {
                            return Err("Cannot be more that the window width.");
                        }
                    }
                    Ok(())
                }),
                settings,
            )),
            settings,
        ));

        let mut btn_bar_vec = vec![];

        btn_bar_vec.push(UIElement::new(
            "Test",
            "btn_test",
            ElemType::Button(Box::new(|scene, ui| {
                if let Some(edit_bar) = &mut ui.active_box_mut().unwrap().edit_bar {
                    edit_bar.text.0 = Some("Test reussi".to_string());
                }
            })),
            settings,
        ));
        btn_bar_vec.push(UIElement::new(
            "Test2",
            "btn_test2",
            ElemType::Button(Box::new(|scene, ui| {
                if let Some(edit_bar) = &mut ui.active_box_mut().unwrap().edit_bar {
                    edit_bar.text.0 = Some("Test 2 reussi".to_string());
                }
            })),
            settings,
        ));

        category.elems.push(UIElement::new(
            "Fps",
            "fps",
            ElemType::Stat(Box::new(|_, ui| {
                if let Some(context) = ui.context() {
                    return context.draw_time_avg.to_string();
                }
                "".to_string()
            })),
            settings,
        ));

        category.elems.push(UIElement::new(
            "",
            "btnbar",
            ElemType::Row(btn_bar_vec),
            settings,
        ));

        vec![UIElement::new(
            "UI Settings",
            "uisettings",
            ElemType::Category(category),
            settings,
        )]
    }
}
