use std::sync::{Arc, RwLock};

use image::RgbaImage;

use crate::{
    model::{maths::hit::Hit, scene::Scene}, ui::{
        draw_utils::draw_element_text, settings, style::{self, Style}, ui::UI, uisettings::UISettings, utils::{get_size, give_back_element, split_in_lines, take_element}
    }, SCREEN_HEIGHT_U32
};

use super::{utils::FnApply, HitBox};

pub const TXT_MESSAGE: &str = "txtMessage";
pub const BTN_APPLY: &str = "btnApply";
pub const BTN_CANCEL: &str = "btnCancel";

pub struct UIEditBar {
    pub text: (Option<String>, Style, Option<HitBox>),
    pub apply: (String, Style, Option<HitBox>),
    pub cancel: (String, Style, Option<HitBox>),
    pub on_apply: Option<FnApply>,
    pub reference: String,
}

impl UIEditBar {
    pub fn cancel(scene: &Arc<RwLock<Scene>>, ui: &mut UI, reference: String) {
        let uibox = ui.get_box_mut(&reference);
        for elem in &mut uibox.elems {
            elem.reset_properties(scene);
        }
    }
    pub fn apply(scene: &Arc<RwLock<Scene>>, ui: &mut UI, reference: String) {
        let mut properties_vec = vec![];
        let uibox = ui.get_box(&reference);
        for elem in &uibox.elems {
            elem.get_properties_reference(&mut properties_vec);
        }
        for reference in properties_vec {
            if let Some((mut elem, parent_ref, index)) =
                take_element(ui, reference.clone())
            {
                elem.submit_properties(scene, ui);
                give_back_element(ui, elem, parent_ref, index);
            } else {
                println!("ERROR: UIElement {} not found", reference)
            }
        }
        let uibox = ui.get_box_mut(&reference);
        if let Some(edit_bar) = uibox.edit_bar.take() {
            if let Some(on_apply) = &edit_bar.on_apply {
                on_apply(None, scene, ui);
            }
            let uibox = ui.get_box_mut(&reference);
            uibox.edit_bar = Some(edit_bar);
        }
    }

    pub fn generate_hitboxes(
        &mut self,
        pos: (u32, u32),
        settings: &UISettings,
        size: (u32, u32),
    ) -> Vec<HitBox> {
        let mut hitbox_vec = vec![];
        let mut offset_y = 0;
        if let Some(str) = &self.text.0 {
            let hitbox = HitBox {
                pos,
                size: get_size(str, &self.text.1, size),
                reference: self.reference.clone() + "." + TXT_MESSAGE,
                disabled: false,
            };
            offset_y += hitbox.size.1;
            self.text.2 = Some(hitbox.clone());
            hitbox_vec.push(hitbox);
        }
        offset_y += settings.margin;
        let apply_size = get_size(&self.apply.0, &self.apply.1, (size.0 / 2, size.1));
        let cancel_size = get_size(&self.cancel.0, &self.cancel.1, (size.0 / 2, size.1));
        let mid_point = pos.0 + size.0 / 2;

        let apply_hitbox = HitBox {
            pos: (mid_point - 20 - apply_size.0, pos.1 + offset_y),
            size: apply_size,
            reference: self.reference.clone() + "." + BTN_APPLY,
            disabled: false,
        };
        let cancel_hitbox = HitBox {
            pos: (mid_point + 20, pos.1 + offset_y),
            size: cancel_size,
            reference: self.reference.clone() + "." + BTN_CANCEL,
            disabled: false,
        };
        self.apply.2 = Some(apply_hitbox.clone());
        self.cancel.2 = Some(cancel_hitbox.clone());
        hitbox_vec.push(apply_hitbox);
        hitbox_vec.push(cancel_hitbox);

        hitbox_vec
    }

    pub fn translate_hitboxes (&mut self, absolute_pos: (u32, u32)) {
        if let Some(hitbox) = &mut self.apply.2 {
            hitbox.pos.0 += absolute_pos.0;
            hitbox.pos.1 += absolute_pos.1;
        }
        if let Some(hitbox) = &mut self.cancel.2 {
            hitbox.pos.0 += absolute_pos.0;
            hitbox.pos.1 += absolute_pos.1;
        }
        if let Some(hitbox) = &mut self.text.2 {
            hitbox.pos.0 += absolute_pos.0;
            hitbox.pos.1 += absolute_pos.1;
        }
    }

    pub fn draw(&self, img: &mut RgbaImage) {
        if let Some(str) = &self.text.0 {
            if let Some(hitbox) = &self.text.2 {
                let format = &self.text.1;
                let available_width = hitbox.size.0 - format.padding_left - format.padding_right;
                let lines = split_in_lines(str.clone(), available_width, format);

                let mut offset_y = 0;
                for line in lines {
                    let size = get_size(&line, format, hitbox.size);
                    draw_element_text(
                        img,
                        line,
                        (hitbox.pos.0, hitbox.pos.1 + offset_y),
                        size,
                        format,
                    );
                    offset_y += size.1;
                }
            }
        }
        if let Some(hitbox) = &self.apply.2 {
            draw_element_text(
                img,
                self.apply.0.clone(),
                hitbox.pos,
                hitbox.size,
                &self.apply.1,
            );
        }
        if let Some(hitbox) = &self.cancel.2 {
            draw_element_text(
                img,
                self.cancel.0.clone(),
                hitbox.pos,
                hitbox.size,
                &self.cancel.1,
            );
        }
    }

    pub fn refresh_formats(&mut self, settings: &UISettings) {
        self.apply.1 = Style::btn_apply(settings);
        self.cancel.1 = Style::btn_cancel(settings);
        let mut textformat = Style::property(settings);
        textformat.bg_color = None;
        self.text.1 = textformat;
    }

    pub fn new(reference: String, settings: &UISettings, on_apply: Option<FnApply>) -> Self {
        let mut textformat = Style::property(settings);
        textformat.bg_color = None;
        Self {
            apply: ("Apply".to_string(), Style::btn_apply(settings), None),
            cancel: ("Cancel".to_string(), Style::btn_cancel(settings), None),
            text: (None, textformat, None),
            reference,
            on_apply,
        }
    }
}
