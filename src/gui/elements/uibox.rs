use image::RgbaImage;

use crate::{
    gui::{gui::Property, uisettings::UISettings, Gui},
    model::{materials::color::Color, scene::Scene},
    SCREEN_WIDTH,
};

use super::{
    ui::UI,
    uielement::{ElemType, UIElement},
    Position,
};

pub struct UIBox {
    pub pos: (u32, u32),
    pub collapsed: bool,
    pub visible: bool,
    pub borders: Option<(Color, usize)>,
    pub background_color: Option<Color>,
    pub zindex: usize,
    pub margin: u32,
    pub elems: Vec<UIElement>,
    pub reference: String,
    pub edit_bar: Option<UIEditBar>,
}

pub struct UIEditBar {
    pub txt_message: UIElement,
    pub btn_apply: UIElement,
    pub btn_cancel: UIElement,
    pub reference: String,
}

impl UIEditBar {
    pub fn new(
        reference: String,
        settings: &UISettings,
        fn_apply: Box<dyn Fn(&mut Scene, &mut UI)>,
    ) -> Self {
        let reference = reference + ".editbar";
        let txt_message = UIElement::new(
            "",
            reference.clone() + ".message",
            ElemType::Text,
            Position::Inline,
            settings,
        );
        let ref2 = reference.clone();
        let fn_validate_and_apply: Box<dyn Fn(&mut Scene, &mut UI)> = Box::new(move |scene, ui| {
            let uibox = ui.get_box_mut((ref2).clone());
            for elem in &mut uibox.elems {
                elem.validate_properties();
            }
            fn_apply(scene, ui);
        });
        let btn_apply = UIElement::new(
            "Apply",
            reference.clone() + ".btnApply",
            ElemType::Button(fn_validate_and_apply),
            Position::Inline,
            settings,
        );
        let ref2 = reference.clone();
        let btn_cancel = UIElement::new(
            "Cancel",
            reference.clone() + ".btnCancel",
            ElemType::Button(Box::new(move |scene, ui| {
                let uibox = ui.get_box_mut(ref2.clone());
                for elem in &mut uibox.elems {
                    elem.reset_properties(scene);
                }
            })),
            Position::Inline,
            settings,
        );
        Self {
            reference,
            txt_message,
            btn_apply,
            btn_cancel,
        }
    }
}

impl UIBox {
    pub fn default(gui: &UI, reference: String) -> Self {
        UIBox {
            pos: (SCREEN_WIDTH as u32 - gui.settings().gui_width, 0),
            background_color: Some(Color::new(0., 0., 0.)),
            borders: None,
            visible: true,
            collapsed: false,
            zindex: 1,
            margin: gui.settings().margin,
            elems: vec![],
            reference,
            edit_bar: None,
        }
    }

    pub fn add_elements(&mut self, mut elems: Vec<UIElement>) {
        self.elems.append(&mut elems);
    }

    pub fn set_edit_bar(
        &mut self,
        settings: &UISettings,
        fn_apply: Box<dyn Fn(&mut Scene, &mut UI)>,
    ) {
        self.edit_bar = Some(UIEditBar::new(self.reference.clone(), settings, fn_apply))
    }

    pub fn validate_properties(&self, scene: &mut Scene, ui: &mut UI) -> Result<(), String> {
        for elem in &self.elems {
            elem.validate_properties()?;
        }
        Ok(())
    }

    // pub fn for_each_property(&self, f: Box<dyn Fn(String, &Property)>) {
    //     for elem in self.elems {
    //         if let ElemType::Property(prop) = elem.elem_type {
    //             f(elem, )
    //         }
    //     }
    // }

    pub fn height(&self, settings: &UISettings) -> u32 {
        let mut size = 0;
        // if let Some((_, width)) = &self.borders {
        //     size += self.margin;
        // }
        for elem in &self.elems {
            size += elem.height(settings);
            size += self.margin;
        }

        size
    }

    pub fn show(&self, ui: &mut UI) {
        ui.set_active_box(self.reference.clone());
    }
}
