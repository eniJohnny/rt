use crate::{
    ui::uisettings::UISettings, SCREEN_HEIGHT_U32, SCREEN_WIDTH_U32
};
use super::{
    ui::UI,
    uieditbar::UIEditBar,
    uielement::UIElement,
    utils::{
        draw_utils::{draw_background, get_needed_height, get_size}, misc::{ElemType, FnAny, Property}, style::Style, ui_utils::{translate_hitboxes, UIContext}, HitBox
    }
};

pub struct UIBox {
    pub relative_pos: BoxPosition,
    pub absolute_pos: (u32, u32),
    pub size: (u32, u32),
    pub offset: u32,
    pub max_height: u32,
    pub scrollable: bool,
    // pub borders: Option<(Color, usize)>,
    pub elems: Vec<UIElement>,
    pub reference: String,
    pub style: Style,
    pub edit_bar: Option<UIEditBar>,
}

impl UIBox {
    pub fn new(reference: &str, pos: BoxPosition, width: u32, settings: &UISettings) -> UIBox {
        UIBox {
            relative_pos: pos,
            absolute_pos: (0, 0),
            size: (width, 0),
            max_height: SCREEN_HEIGHT_U32,
            style: Style::uibox(settings),
            elems: vec![],
            reference: reference.to_string(),
            edit_bar: None,
            offset: 0,
            scrollable: false
        }
    }

    pub fn add_elements(&mut self, mut elems: Vec<UIElement>) {
        for elem in &mut elems {
            elem.update_reference(self.reference.clone());
        }
        self.elems.append(&mut elems);
    }

    pub fn set_edit_bar(&mut self, settings: &UISettings, on_apply: Option<FnAny>) {
        self.edit_bar = Some(UIEditBar::new(self.reference.clone(), settings, on_apply))
    }

    pub fn validate_properties(
        &self,
        ui: &mut UI,
    ) -> Result<(), String> {
        for elem in &self.elems {
            elem.validate_properties(ui)?;
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
        self.style.visible = true;
    }

    pub fn get_property(&self, reference: &str) -> Option<&Property> {
        for elem in &self.elems {
            if let Some(property) = elem.get_property(reference) {
                return Some(property);
            }
        }
        None
    }

    pub fn get_property_mut(&mut self, reference: &str) -> Option<&mut Property> {
        for elem in &mut self.elems {
            if let Some(property) = elem.get_property_mut(reference) {
                return Some(property);
            }
        }
        None
    }

    pub fn get_element(&self, reference: &str) -> Option<&UIElement>{
        for elem in &self.elems {
            if let Some(elem) = elem.get_element(reference) {
                return Some(elem);
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

    pub fn style(&self) -> &Style {
        &self.style
    }

    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn generate_hitboxes(
        &mut self,
        ui: &mut UI,
        context: &UIContext,
        settings: &UISettings,
    ) -> Vec<HitBox> {
        self.scrollable = false;
        let mut edit_bar_hitbox_list = vec![];
        let mut hitbox_list = vec![];
        let mut edit_bar_height = 0;
        let pos = (self.style.border_left as i32, self.style.border_top as i32);
        let mut size = (self.size.0 - self.style.border_left - self.style.border_right, self.max_height - self.style.border_top - self.style.border_bot);
        // We process the edit bar first
        if let Some(mut edit_bar) = self.edit_bar.take() {
            edit_bar_hitbox_list = 
                edit_bar.generate_hitboxes((pos.0 as u32, pos.1 as u32), settings, size);
            edit_bar_height = get_needed_height(&edit_bar_hitbox_list);
            self.edit_bar = Some(edit_bar);
            if edit_bar_height > self.max_height {
                // If we don't even have enough place for the edit bar, we dont show the UIBox at all
                return vec![];
            }
        }
        // We always show the edit_bar if present, it's the fields we truncate if needed
        size.1 -= edit_bar_height;
        let mut available_height;
        let mut current_offset = 0;
        
        // We first calculate the positions from position 0,0, we will translate all of the hitboxes later. We need the height to accurately position the box in most cases.
        for i in 0..self.elems.len() {
            // To avoid mutable borrows problems and be able to provide a mutable UI to the needed functions, we take the element out of the UI.
            let mut elem = self.elems.remove(i);
            if elem.style.visible {
                let hitbox_size = get_size(
                    &elem.text,
                    &elem.style,
                    (size.0, size.1),
                );
                let mut hitbox = HitBox {
                    pos: (pos.0, pos.1 + current_offset as i32 - self.offset as i32 + settings.margin as i32),
                    size: hitbox_size,
                    reference: elem.reference.clone(),
                    disabled: matches!(elem.elem_type, ElemType::Row(_)),
                    visible: true
                };
                available_height = size.1 + self.offset - current_offset;
                let needed_height = (hitbox.pos.1 + hitbox.size.1 as i32 + self.offset as i32) as u32;
                elem.hitbox = Some(hitbox.clone());
                let vec = elem.generate_hitbox(ui, context, available_height as i32);
                if hitbox.pos.1 < self.style.border_top as i32 || hitbox.pos.1 as u32 + hitbox.size.1 > size.1 {
                    hitbox.disabled = true;
                    hitbox.visible = false;
                    elem.hitbox = Some(hitbox.clone());
                }
                if needed_height > current_offset {
                    current_offset = needed_height;
                }
                hitbox_list.push(hitbox);
                
                for mut hitbox in vec {
                    let needed_height = (hitbox.pos.1 + hitbox.size.1 as i32 + self.offset as i32) as u32;
                    if hitbox.pos.1 < 0 {
                        hitbox.disabled = true;
                        hitbox.visible = false;
                        if needed_height > current_offset {
                            current_offset = needed_height;
                        }
                    } else if hitbox.pos.1 as u32 + hitbox.size.1 > size.1 {
                        current_offset = size.1 + self.offset;
                        hitbox.disabled = true;
                        hitbox.visible = false;
                        self.scrollable = true;
                    } else {
                        hitbox_list.push(hitbox);
                        if needed_height > current_offset {
                            current_offset = needed_height;
                        }
                    }
                }
            }
            // We insert the element back into the ui
            self.elems.insert(i, elem);
        }
        let total_height = size.1.min(current_offset - self.offset);
        //We now have the true size of the box
        translate_hitboxes(&mut hitbox_list, self.absolute_pos.0, self.absolute_pos.1);
        translate_hitboxes(&mut edit_bar_hitbox_list, self.absolute_pos.0, self.absolute_pos.1 + total_height);
        hitbox_list.append(&mut edit_bar_hitbox_list);
        self.translate_hitboxes_to_relative_position(current_offset - self.offset, edit_bar_height, settings);
        hitbox_list
    }

    pub fn translate_hitboxes_to_relative_position(&mut self, fields_height: u32, edit_bar_height: u32, settings: &UISettings) {
        self.size.1 = fields_height + edit_bar_height + settings.margin * 2 + self.style.border_bot + self.style.border_top;
        self.absolute_pos = self.relative_pos.get_pos(self.size);
        for elem in &mut self.elems {
            elem.translate_hitboxes(self.absolute_pos);
        }
        if let Some(edit_bar) = &mut self.edit_bar {
            edit_bar.translate_hitboxes((self.absolute_pos.0, self.absolute_pos.1 + fields_height));
        }
    }

    pub fn draw(&self, ui: &UI, context: &mut UIContext) {
        draw_background(&mut context.ui_img, self.absolute_pos, self.size, &self.style);
        for elem in &self.elems {
            if elem.style.visible {
                elem.draw(ui, context);
            }
        }
        if let Some(edit_bar) = &self.edit_bar {
            edit_bar.draw(&mut context.ui_img);
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
            BoxPosition::TopLeft(offset_y, offset_x) => {
                (offset_x, offset_y)
            }
            BoxPosition::TopRight(offset_y, offset_x) => {
                (SCREEN_WIDTH_U32 - offset_x - size.0, offset_y)
            }
            BoxPosition::BotLeft(offset_y, offset_x) => {
                (offset_x, SCREEN_HEIGHT_U32 - offset_y - size.1)
            }
            BoxPosition::BotRight(offset_y, offset_x) => {
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