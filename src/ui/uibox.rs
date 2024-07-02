use std::{
    cell::{Ref, RefCell},
    sync::{Arc, RwLock},
};

use image::RgbaImage;

use crate::{
    model::{materials::color::Color, scene::Scene},
    ui::{style::Style, uisettings::UISettings},
    SCREEN_HEIGHT, SCREEN_HEIGHT_U32, SCREEN_WIDTH,
};

use super::{
    draw_utils::draw_background,
    elements::{
        uieditbar::UIEditBar,
        uielement::UIElement,
        utils::{ElemType, FnApply},
        HitBox,
    },
    ui::UI,
    utils::{get_pos, get_size},
};

pub struct UIBox {
    pub pos: (u32, u32),
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
    pub fn new(reference: String, pos: (u32, u32), width: u32) -> UIBox {
        UIBox {
            pos: pos,
            size: (width, 0),
            max_height: SCREEN_HEIGHT_U32,
            background_color: Some(Color::new(0.1, 0.1, 0.1)),
            // borders: None,
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

    pub fn process(
        &mut self,
        ui: &mut UI,
        scene: &Arc<RwLock<Scene>>,
        settings: &UISettings,
    ) -> Vec<HitBox> {
        let mut hitbox_vec = vec![];
        let mut offset_y = 0;
        self.size.1 = self.max_height;
        for i in 0..self.elems.len() {
            let mut elem = self.elems.remove(i);
            if elem.visible {
                let hitbox = HitBox {
                    pos: get_pos(self.pos, (0, self.pos.1 + offset_y), 0),
                    size: get_size(
                        &elem.text,
                        &elem.style,
                        (self.size.0, self.size.1 - offset_y),
                    ),
                    reference: elem.reference.clone(),
                    disabled: matches!(elem.elem_type, ElemType::Row(_)),
                };
                elem.hitbox = Some(hitbox.clone());
                let vec = elem.process(ui, scene, self.max_height - offset_y);
                let needed_height = hitbox.pos.1 + hitbox.size.1 + settings.margin - self.pos.1;
                if needed_height >= self.size.1 {
                    break;
                }
                if needed_height > offset_y {
                    offset_y = needed_height;
                }
                hitbox_vec.push(hitbox);

                for hitbox in vec {
                    let needed_height = hitbox.pos.1 + hitbox.size.1 + settings.margin - self.pos.1;
                    if needed_height > offset_y {
                        offset_y = needed_height;
                    }
                    hitbox_vec.push(hitbox)
                }
            }
            self.elems.insert(i, elem);
        }
        if let Some(mut edit_bar) = self.edit_bar.take() {
            let mut vec =
                edit_bar.process((self.pos.0, self.pos.1 + offset_y), settings, self.size);
            offset_y = vec[1].pos.1 + vec[1].size.1 + settings.margin * 2;
            hitbox_vec.append(&mut vec);
            self.edit_bar = Some(edit_bar);
        }
        self.size.1 = offset_y;
        hitbox_vec
    }

    pub fn draw(&self, img: &mut RgbaImage, ui: &UI, scene: &Arc<RwLock<Scene>>) {
        if let Some(color) = &self.background_color {
            draw_background(img, self.pos, self.size, color.to_rgba(), 0);
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
