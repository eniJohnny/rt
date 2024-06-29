use std::cell::{Ref, RefCell};

use image::RgbaImage;

use crate::{
    gui::{gui::Property, textformat::Style, uisettings::UISettings, Gui},
    model::{materials::color::Color, scene::Scene},
    SCREEN_HEIGHT, SCREEN_HEIGHT_U32, SCREEN_WIDTH,
};

use super::{
    ui::UI,
    uieditbar::UIEditBar,
    uielement::{ElemType, FnApply, UIElement},
    HitBox,
};

pub struct UIBox {
    pub pos: (u32, u32),
    pub size: (u32, u32),
    pub max_height: u32,
    pub visible: bool,
    pub borders: Option<(Color, usize)>,
    pub background_color: Option<Color>,
    pub elems: Vec<UIElement>,
    pub reference: String,
    pub edit_bar: Option<UIEditBar>,
}

impl UIBox {
    pub fn default(gui: &UI, reference: String) -> Self {
        UIBox {
            pos: (SCREEN_WIDTH as u32 - gui.uisettings().gui_width, 0),
            size: (0, 0),
            max_height: SCREEN_HEIGHT_U32,
            background_color: Some(Color::new(0.1, 0.1, 0.1)),
            borders: None,
            visible: true,
            elems: vec![],
            reference: reference.clone(),
            edit_bar: None,
        }
    }

    pub fn add_elements(&mut self, mut elems: Vec<UIElement>) {
        for elem in &mut elems {
            elem.set_reference(self.reference.clone());
        }
        self.elems.append(&mut elems);
    }

    pub fn set_edit_bar(&mut self, settings: &UISettings, on_apply: Option<FnApply>) {
        self.edit_bar = Some(UIEditBar::new(self.reference.clone(), settings, on_apply))
    }

    pub fn validate_properties(&self, scene: &mut Scene, ui: &mut UI) -> Result<(), String> {
        for elem in &self.elems {
            elem.validate_properties()?;
        }
        Ok(())
    }

    // pub fn refresh_formats(&mut self) {
    //     for_each_element(&mut self.elems, &None, &None, |elem, scene, ui| {
    //         // if let Some(ui) = ui {
    //         //     elem.format = elem.elem_type.base_format(ui.settings());
    //         // }
    //     });
    // }

    pub fn refresh_formats(&mut self, settings: &UISettings) {
        for elem in &mut self.elems {
            elem.refresh_format(settings);
        }
        if let Some(edit_bar) = &mut self.edit_bar {
            edit_bar.refresh_formats(settings);
        }
    }

    pub fn height(&self, settings: &UISettings) -> u32 {
        let mut edit_bar_height = 0;
        if let Some(edit_bar) = &self.edit_bar {
            edit_bar_height += edit_bar.height(self.size.0, settings);
        }
        let mut height = 0;
        for elem in &self.elems {
            if !elem.visible {
                continue;
            }
            if edit_bar_height >= self.max_height {
                break;
            }
            height += elem.height(
                (self.size.0, self.max_height - edit_bar_height - height),
                settings,
            ) + settings.margin;
        }
        height + edit_bar_height
    }

    pub fn show(&self, ui: &mut UI) {
        ui.set_active_box(self.reference.clone());
    }
}
