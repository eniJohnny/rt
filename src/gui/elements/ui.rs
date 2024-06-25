use std::collections::HashMap;

use crate::{gui::settings::UISettings, model::maths::vec2::Vec2};

use super::window::UIBox;

pub struct UI {
    boxes: HashMap<usize, UIBox>,
    inlined: Vec<usize>,
    settings: UISettings,
}

impl UI {
    pub fn default() -> Self {
        UI {
            boxes: HashMap::new(),
            inlined: vec![],
            settings: UISettings::default(),
        }
    }

    pub fn settings(&self) -> &UISettings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut UISettings {
        &mut self.settings
    }

    fn get_inlined_box_pos(&self, box_index: usize, settings: &UISettings) -> (usize, usize) {
        let mut pos = (settings.padding, settings.padding);
        for index in &self.inlined {
            if *index == box_index {
                return pos;
            }
            let uibox = self
                .boxes
                .get(&index)
                .expect("Inlined boxes' indexes are out of date");
        }
        pos
    }

    pub fn draw(&self) {
        // let inline_pos = (self.padding, self.padding);
        // let mut other_boxes = self.other_boxes.clone().split_off(at);
        // boxes.sort_by(|a, b| a.zindex.cmp(&b.zindex));
        // for box in boxes
    }
}
