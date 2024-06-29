use image::RgbaImage;

use crate::{
    gui::{
        elements::utils::get_size,
        settings,
        textformat::{self, Style},
        uisettings::UISettings,
    },
    model::{maths::hit::Hit, scene::Scene},
    SCREEN_HEIGHT_U32,
};

use super::{
    ui::UI,
    uielement::{ElemType, FnApply, FnSubmit, UIElement},
    utils::{draw_element_text, split_in_lines},
    HitBox,
};

pub const TXT_MESSAGE: &str = "txtMessage";
pub const BTN_APPLY: &str = "btnApply";
pub const BTN_CANCEL: &str = "btnCancel";

pub struct UIEditBar {
    pub text: (Option<String>, Style),
    pub apply: (String, Style),
    pub cancel: (String, Style),
    pub on_apply: Option<FnApply>,
    pub reference: String,
}

impl UIEditBar {
    pub fn cancel(scene: &mut Scene, ui: &mut UI, reference: String) {
        let uibox = ui.get_box_mut(reference);
        for elem in &mut uibox.elems {
            elem.reset_properties(scene);
        }
    }
    pub fn apply(scene: &mut Scene, ui: &mut UI, reference: String) {
        let mut vector = ui.get_box_mut(reference.clone()).elems.split_off(0);
        for elem in &mut vector {
            elem.submit_properties(scene, ui);
        }
        let uibox = ui.get_box_mut((reference).clone());
        uibox.elems.append(&mut vector);

        if let Some(edit_bar) = uibox.edit_bar.take() {
            if let Some(on_apply) = &edit_bar.on_apply {
                on_apply(scene, ui);
            }
            let uibox = ui.get_box_mut((reference).clone());
            uibox.edit_bar = Some(edit_bar);
        }
    }
    pub fn draw(
        &self,
        img: &mut RgbaImage,
        pos: (u32, u32),
        settings: &UISettings,
        width: u32,
    ) -> Vec<HitBox> {
        let mut hitbox_vec = vec![];
        let mut offset_y = 0;
        if let Some(str) = &self.text.0 {
            let format = &self.text.1;
            let available_width = format.width - format.padding_left - format.padding_right;
            let lines = split_in_lines(str.clone(), available_width, format);
            let hitbox = HitBox {
                pos,
                size: (
                    format.width,
                    (format.font_size() as u32 + format.padding_bot + format.padding_top)
                        * lines.len() as u32,
                ),
                reference: self.reference.clone() + "." + TXT_MESSAGE,
                disabled: false,
            };
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
            hitbox_vec.push(hitbox);
        }
        offset_y += settings.margin;
        let apply_size = get_size(&self.apply.0, &self.apply.1, (width / 2, SCREEN_HEIGHT_U32));
        let cancel_size = get_size(
            &self.cancel.0,
            &self.cancel.1,
            (width / 2, SCREEN_HEIGHT_U32),
        );
        let mid_point = pos.0 + width / 2;

        let apply_hitbox = HitBox {
            pos: (mid_point - 20 - apply_size.0, pos.1 + offset_y),
            size: apply_size,
            reference: self.reference.clone() + "." + BTN_APPLY,
            disabled: false,
        };
        draw_element_text(
            img,
            self.apply.0.clone(),
            apply_hitbox.pos,
            apply_hitbox.size,
            &self.apply.1,
        );

        let cancel_hitbox = HitBox {
            pos: (mid_point + 20, pos.1 + offset_y),
            size: cancel_size,
            reference: self.reference.clone() + "." + BTN_CANCEL,
            disabled: false,
        };
        draw_element_text(
            img,
            self.cancel.0.clone(),
            cancel_hitbox.pos,
            cancel_hitbox.size,
            &self.cancel.1,
        );
        hitbox_vec.push(apply_hitbox);
        hitbox_vec.push(cancel_hitbox);

        hitbox_vec
    }
    pub fn height(&self, width: u32, settings: &UISettings) -> u32 {
        let mut height = 0;
        if let Some(str) = &self.text.0 {
            let format = &self.text.1;
            let available_width = width - format.padding_left - format.padding_right;
            let lines = split_in_lines(str.clone(), available_width, format);
            height += (format.font_size() as u32 + format.padding_bot) * lines.len() as u32;
        }
        height += settings.margin;
        height += get_size(&self.apply.0, &self.apply.1, (width / 2, SCREEN_HEIGHT_U32))
            .1
            .max(
                get_size(
                    &self.cancel.0,
                    &self.cancel.1,
                    (width / 2, SCREEN_HEIGHT_U32),
                )
                .1,
            )
            + settings.margin * 2;
        height
    }

    pub fn refresh_formats(&mut self, settings: &UISettings) {
        self.apply.1 = Style::btn_apply(settings);
        self.cancel.1 = Style::btn_cancel(settings);
        let mut textformat = Style::field_format(settings);
        textformat.bg_color = None;
        self.text.1 = textformat;
    }

    pub fn new(reference: String, settings: &UISettings, on_apply: Option<FnApply>) -> Self {
        let mut textformat = Style::field_format(settings);
        textformat.bg_color = None;
        Self {
            apply: ("Apply".to_string(), Style::btn_apply(settings)),
            cancel: ("Cancel".to_string(), Style::btn_cancel(settings)),
            text: (None, textformat),
            reference,
            on_apply,
        }
    }
}
