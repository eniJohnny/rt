use crate::{gui::{textformat::TextFormat, uisettings::UISettings}, model::scene::Scene};

use super::{ui::UI, uielement::{ElemType, UIElement}, Position};


pub struct UIEditBar {
    pub txt_message: UIElement,
    pub btn_apply: UIElement,
    pub btn_cancel: UIElement,
    pub reference: String,
}

impl UIEditBar {
    pub fn new(reference: String, settings: &UISettings) -> Self {
        let reference = reference + ".editbar";
        let mut txt_message = UIElement::new(
            "",
            reference.clone() + ".message",
            ElemType::Text,
            Position::Relative(0, -120),
            settings,
        );
        txt_message.visible = false;
        let ref2 = reference.clone();
        let fn_validate_and_apply: Box<dyn Fn(&mut Scene, &mut UI)> = Box::new(move |scene, ui| {
            {
                let uibox = ui.get_box_mut((ref2).clone());
                let mut error = None;
                for elem in &mut uibox.elems {
                    if let Err(e) = elem.validate_properties() {
                        error = Some(e);
                        break;
                    }
                }
                if let Some(edit_bar) = &mut uibox.edit_bar {
                    if let Some(error) = error {
                        edit_bar.txt_message.text = error;
                        edit_bar.txt_message.visible = true;
                        return;
                    } else {
                        edit_bar.txt_message.visible = false;
                    }
                }
            }
            {
                let mut vector = ui.get_box_mut((ref2).clone()).elems.split_off(0);
                for elem in &mut vector {
                    elem.submit_properties(scene, ui);
                }
                ui.get_box_mut((ref2).clone()).elems.append(&mut vector);
            }
        });
        let mut btn_apply = UIElement::new(
            "Apply",
            reference.clone() + ".btnApply",
            ElemType::Button(fn_validate_and_apply),
            Position::Relative(settings.gui_width as i32 / 2 - 120, -50),
            settings,
        );
        let ref2 = reference.clone();
        let mut btn_cancel = UIElement::new(
            "Cancel",
            reference.clone() + ".btnCancel",
            ElemType::Button(Box::new(move |scene, ui| {
                let uibox = ui.get_box_mut(ref2.clone());
                for elem in &mut uibox.elems {
                    elem.reset_properties(scene);
                }
            })),
            Position::Relative(settings.gui_width as i32 / 2 + 20, -50),
            settings,
        );
        btn_cancel.set_format(TextFormat::new_btn_cancel_format(settings));
        btn_apply.set_format(TextFormat::new_btn_apply_format(settings));
        Self {
            reference,
            txt_message,
            btn_apply,
            btn_cancel,
        }
    }
}