use image::RgbaImage;
use winit::dpi::Position;

use crate::{
    gui::{settings::UISettings, Gui},
    model::{materials::color::Color, scene::Scene},
};

use super::fields::UIElement;

pub struct UIBox {
    pub pos: Position,
    pub collapsed: bool,
    pub visible: bool,
    pub borders: Option<(Color, usize)>,
    pub background_color: Option<Color>,
    pub zindex: usize,
    pub padding: usize,
    pub elems: Vec<UIElement>,
}

impl UIBox {
    pub fn default(pos: Position, gui: &Gui) -> Self {
        UIBox {
            pos,
            background_color: Some(Color::new(0., 0., 0.)),
            borders: None,
            visible: true,
            collapsed: false,
            zindex: 1,
            padding: 5,
            elems: vec![],
        }
    }

    pub fn size(&self, settings: &UISettings) -> (usize, usize) {
        let mut size = (0, 0);
        if let Some((_, width)) = &self.borders {
            size.0 += width * 2;
            size.1 += width;
        }
        for elem in &self.elems {
            size.1 += elem.height(settings);
        }

        size
    }
}
