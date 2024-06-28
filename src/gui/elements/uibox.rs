use std::cell::{Ref, RefCell};

use image::RgbaImage;

use crate::{
    gui::{gui::Property, textformat::TextFormat, uisettings::UISettings, Gui},
    model::{materials::color::Color, scene::Scene},
    SCREEN_WIDTH,
};

use super::{
    ui::UI,
    uieditbar::UIEditBar,
    uielement::{ElemType, UIElement},
    HitBox,
};

pub struct UIBox {
    pub pos: (u32, u32),
    pub size: (u32, u32),
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
            pos: (SCREEN_WIDTH as u32 - gui.settings().gui_width, 0),
            size: (0, 0),
            background_color: Some(Color::new(0.1, 0.1, 0.1)),
            borders: None,
            visible: true,
            elems: vec![],
            reference,
            edit_bar: None,
        }
    }

    pub fn add_elements(&mut self, mut elems: Vec<UIElement>) {
        self.elems.append(&mut elems);
    }

    pub fn set_edit_bar(&mut self, settings: &UISettings) {
        self.edit_bar = Some(UIEditBar::new(self.reference.clone(), settings))
    }

    pub fn validate_properties(&self, scene: &mut Scene, ui: &mut UI) -> Result<(), String> {
        for elem in &self.elems {
            elem.validate_properties()?;
        }
        Ok(())
    }

    pub fn height(&self, settings: &UISettings) -> u32 {
        let mut inline_height = 0;
        for elem in &self.elems {
            if !elem.visible {
                continue;
            }
            inline_height += elem.height(settings);
        }
        if let Some(edit_bar) = &self.edit_bar {
            todo!("EditBar")
        }
        inline_height
    }

    pub fn show(&self, ui: &mut UI) {
        ui.set_active_box(self.reference.clone());
    }
}
