use std::{
    cell::{Ref, RefCell},
    sync::{Arc, RwLock},
};

use image::RgbaImage;

use crate::{
    model::{materials::color::Color, scene::Scene},
    ui::{uisettings::UISettings},
    SCREEN_HEIGHT, SCREEN_HEIGHT_U32, SCREEN_WIDTH, SCREEN_WIDTH_U32,
};

use super::{ui::UI, uieditbar::UIEditBar, uielement::UIElement, utils::{draw_utils::{draw_background, get_needed_height, get_size}, misc::{ElemType, FnAny, Property}, ui_utils::translate_hitboxes, HitBox}};

pub struct UIBox {
    pub relative_pos: BoxPosition,
    pub absolute_pos: (u32, u32),
    pub size: (u32, u32),
    pub max_height: u32,
    pub visible: bool,
    // pub borders: Option<(Color, usize)>,
    pub background_color: Option<Color>,
    pub elems: Vec<UIElement>,
    pub reference: String,
    pub edit_bar: Option<UIEditBar>,
}

impl UIBox {
    pub fn new(reference: &str, pos: BoxPosition, width: u32) -> UIBox {
        UIBox {
            relative_pos: pos,
            absolute_pos: (0, 0),
            size: (width, 0),
            max_height: SCREEN_HEIGHT_U32,
            background_color: Some(Color::new(0.1, 0.1, 0.1)),
            // borders: None,
            visible: true,
            elems: vec![],
            reference: reference.to_string(),
            edit_bar: None,
        }
    }

    pub fn add_elements(&mut self, mut elems: Vec<UIElement>) {
        for elem in &mut elems {
            elem.set_reference(self.reference.clone());
        }
        self.elems.append(&mut elems);
    }

    pub fn set_edit_bar(&mut self, settings: &UISettings, on_apply: Option<FnAny>) {
        self.edit_bar = Some(UIEditBar::new(self.reference.clone(), settings, on_apply))
    }

    pub fn validate_properties(
        &self,
        scene: &Arc<RwLock<Scene>>,
        ui: &mut UI,
    ) -> Result<(), String> {
        for elem in &self.elems {
            elem.validate_properties()?;
        }
        Ok(())
    }

    pub fn refresh_formats(&mut self, settings: &UISettings) {
        for elem in &mut self.elems {
            elem.refresh_format(settings);
        }
        if let Some(edit_bar) = &mut self.edit_bar {
            edit_bar.refresh_formats(settings);
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn get_property_mut(&mut self, reference: &str) -> Option<&mut Property> {
        for elem in &mut self.elems {
            if let Some(property) = elem.get_property_mut(reference) {
                return Some(property);
            }
        }
        None
    }

    pub fn get_element_mut(&mut self, reference: &str) -> Option<&mut UIElement>{
        for elem in &mut self.elems {
            if let Some(elem) = elem.get_element_mut(reference) {
                return Some(elem);
            }
        }
        None
    }

    pub fn generate_hitboxes(
        &mut self,
        ui: &mut UI,
        scene: &Arc<RwLock<Scene>>,
        settings: &UISettings,
    ) -> Vec<HitBox> {
        let mut edit_bar_hitbox_list = vec![];
        let mut hitbox_list = vec![];
        let mut edit_bar_height = 0;
        // We process the edit bar first
        if let Some(mut edit_bar) = self.edit_bar.take() {
            edit_bar_hitbox_list = 
                edit_bar.generate_hitboxes((0, 0), settings, (self.size.0, self.max_height));
            edit_bar_height = get_needed_height(&edit_bar_hitbox_list);
            self.edit_bar = Some(edit_bar);
            if edit_bar_height > self.max_height {
                // If we don't even have enough place for the edit bar, we dont show the UIBox at all
                return vec![];
            }
        }
        let mut fields_height = 0;
        // We always show the edit_bar if present, it's the fields we truncate if needed
        let max_height = self.max_height - edit_bar_height;
        // We first calculate the positions from position 0,0, we will translate all of the hitboxes later. We need the height to accurately position the box in most cases.
        let pos = (0, 0);
        for i in 0..self.elems.len() {
            // To avoid mutable borrows problems and be able to provide a mutable UI to the needed functions, we take the element out of the UI.
            let mut elem = self.elems.remove(i);
            if elem.visible {
                let hitbox = HitBox {
                    pos: (pos.0, pos.1 + fields_height + settings.margin),
                    size: get_size(
                        &elem.text,
                        &elem.style,
                        (self.size.0, max_height - fields_height),
                    ),
                    reference: elem.reference.clone(),
                    disabled: matches!(elem.elem_type, ElemType::Row(_)),
                };
                elem.hitbox = Some(hitbox.clone());
                let vec = elem.generate_hitbox(ui, scene, max_height - fields_height);
                let needed_height = (hitbox.pos.1 + hitbox.size.1 + settings.margin).max(get_needed_height(&vec));
                if needed_height >= max_height {
                    break;
                }
                if needed_height > fields_height {
                    fields_height = needed_height;
                }
                hitbox_list.push(hitbox);

                for hitbox in vec {
                    let needed_height = hitbox.pos.1 + hitbox.size.1 + settings.margin - pos.1;
                    if needed_height > fields_height {
                        fields_height = needed_height;
                    }
                    hitbox_list.push(hitbox)
                }
            }
            self.elems.insert(i, elem);
        }
        //We now have the true size of the box
        translate_hitboxes(&mut hitbox_list, self.absolute_pos.0, self.absolute_pos.1);
        translate_hitboxes(&mut edit_bar_hitbox_list, self.absolute_pos.0, self.absolute_pos.1 + fields_height);
        hitbox_list.append(&mut edit_bar_hitbox_list);
        self.translate_hitboxes_to_relative_position(fields_height, edit_bar_height, settings);
        hitbox_list
    }

    pub fn translate_hitboxes_to_relative_position(&mut self, fields_height: u32, edit_bar_height: u32, settings: &UISettings) {
        self.size.1 = fields_height + edit_bar_height + settings.margin * 2;
        self.absolute_pos = self.relative_pos.get_pos(self.size);
        for elem in &mut self.elems {
            elem.translate_hitboxes(self.absolute_pos);
        }
        if let Some(edit_bar) = &mut self.edit_bar {
            edit_bar.translate_hitboxes((self.absolute_pos.0, self.absolute_pos.1 + fields_height));
        }
    }

    pub fn draw(&self, img: &mut RgbaImage, ui: &UI, scene: &Arc<RwLock<Scene>>) {
        if let Some(color) = &self.background_color {
            draw_background(img, self.absolute_pos, self.size, color.to_rgba(), 0);
        }
        for elem in &self.elems {
            if elem.visible {
                elem.draw(img, ui, scene);
            }
        }
        if let Some(edit_bar) = &self.edit_bar {
            edit_bar.draw(img);
        }
    }
}

pub enum BoxPosition {
    TopLeft(u32, u32),
    TopRight(u32, u32),
    BotLeft(u32, u32),
    BotRight(u32, u32),
    Center,
    CenterLeft(u32),
    CenterTop(u32),
    CenterRight(u32),
    CenterBot(u32),
}

impl BoxPosition {
    pub fn get_pos(&self, size: (u32, u32)) -> (u32, u32) {
        match *self {
            BoxPosition::TopLeft(offset_x, offset_y) => {
                (offset_x, offset_y)
            }
            BoxPosition::TopRight(offset_x, offset_y) => {
                (SCREEN_WIDTH_U32 - offset_x - size.0, offset_y)
            }
            BoxPosition::BotLeft(offset_x, offset_y) => {
                (offset_x, SCREEN_HEIGHT_U32 - offset_y - size.1)
            }
            BoxPosition::BotRight(offset_x, offset_y) => {
                (SCREEN_WIDTH_U32 - offset_x - size.0, SCREEN_HEIGHT_U32 - offset_y - size.1)
            }
            BoxPosition::Center => {
                let center = (SCREEN_WIDTH_U32 / 2, SCREEN_HEIGHT_U32 / 2);
                (center.0 - size.0 / 2, center.1 - size.1 / 2)
            }
            BoxPosition::CenterLeft(offset_x) => {
                let center_y = SCREEN_HEIGHT_U32 / 2;
                (offset_x, center_y - size.1 / 2)
            }
            BoxPosition::CenterRight(offset_x) => {
                let center_y = SCREEN_HEIGHT_U32 / 2;
                (SCREEN_WIDTH_U32 - offset_x - size.0, center_y - size.1 / 2)
            }
            BoxPosition::CenterTop(offset_y) => {
                let center_x = SCREEN_WIDTH_U32 / 2;
                (center_x - size.0 / 2, offset_y)
            }
            BoxPosition::CenterBot(offset_y) => {
                let center_x = SCREEN_WIDTH_U32 / 2;
                (center_x - size.0 / 2, SCREEN_HEIGHT_U32 - offset_y - size.1)
            }
        }
    }
}