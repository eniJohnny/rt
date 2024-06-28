use crate::{
    gui::{
        textformat::{self, TextFormat},
        uisettings::UISettings,
    },
    model::{maths::hit::Hit, scene::Scene},
};

use super::{
    ui::UI,
    uielement::{ElemType, UIElement},
    utils::split_in_lines,
    HitBox,
};

pub struct UIEditBar {
    pub text: (Option<String>, TextFormat),
    pub apply: (String, TextFormat),
    pub cancel: (String, TextFormat),
    // pub reference: String,
}

impl UIEditBar {
    pub fn cancel(scene: &mut Scene, ui: &mut UI, reference: String) {
        let uibox = ui.get_box_mut(reference);
        for elem in &mut uibox.elems {
            elem.reset_properties(scene);
        }
    }
    pub fn apply(scene: &mut Scene, ui: &mut UI, reference: String) {
        {
            let uibox = ui.get_box_mut(reference.clone());
            let mut error = None;
            for elem in &mut uibox.elems {
                if let Err(e) = elem.validate_properties() {
                    error = Some(e);
                    break;
                }
            }
            if let Some(edit_bar) = &mut uibox.edit_bar {
                if let Some(error) = error {
                    edit_bar.text.0 = Some(error);
                    return;
                } else {
                    edit_bar.text.0 = None
                }
            }
        }
        {
            let mut vector = ui.get_box_mut(reference.clone()).elems.split_off(0);
            for elem in &mut vector {
                elem.submit_properties(scene, ui);
            }
            ui.get_box_mut((reference).clone())
                .elems
                .append(&mut vector);
        }
    }
    pub fn draw(&self, pos: (u32, u32)) -> Vec<HitBox> {
        // let apply_hitbox = HitBox {
        //     pos: (pos.x)
        // }
        vec![]
    }
    pub fn height(&self, settings: &UISettings) -> u32 {
        let mut height = 0;
        if let Some(str) = &self.text.0 {
            let format = &self.text.1;
            let available_width = format.width - format.padding_left - format.padding_right;
            let lines = split_in_lines(str.clone(), available_width, format);
            height += (format.font_size() as u32 + settings.margin) * lines.len() as u32;
        }
        height += self.apply.1.height;
        height
    }
    pub fn new(reference: String, settings: &UISettings) -> Self {
        // Position::Relative(settings.gui_width as i32 / 2 - 120, -50),
        // Position::Relative(settings.gui_width as i32 / 2 + 20, -50),
        let mut textformat = TextFormat::field_format(settings);
        textformat.bg_color = None;
        Self {
            apply: (
                "Apply".to_string(),
                TextFormat::new_btn_apply_format(settings),
            ),
            cancel: (
                "Cancel".to_string(),
                TextFormat::new_btn_cancel_format(settings),
            ),
            text: (None, textformat),
        }
    }
}
