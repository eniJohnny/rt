use image::RgbaImage;
use winit::dpi::Position;

use crate::{
    gui::{uisettings::UISettings, Gui},
    model::{materials::color::Color, scene::Scene}, SCREEN_WIDTH,
};

use super::{fields::UIElement, ui::UI};

pub struct UIBox {
    pub id: usize,
    pub pos: (u32, u32),
    pub collapsed: bool,
    pub visible: bool,
    pub borders: Option<(Color, usize)>,
    pub background_color: Option<Color>,
    pub zindex: usize,
    pub margin: u32,
    pub elems: Vec<UIElement>,
    pub reference: String
}

impl UIBox {
    pub fn default(gui: &UI, reference: String) -> Self {
        UIBox {
            id: 0,
            pos: (SCREEN_WIDTH as u32 - gui.settings().gui_width, 0),
            background_color: Some(Color::new(0., 0., 0.)),
            borders: None,
            visible: true,
            collapsed: false,
            zindex: 1,
            margin: gui.settings().margin,
            elems: vec![],
            reference
        }
    }

    pub fn add_elements(&mut self, mut elems: Vec<UIElement>) {
        self.elems.append(&mut elems);
    }

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
        ui.set_active_box(self.id);
    }
}
