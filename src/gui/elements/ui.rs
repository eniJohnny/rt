use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use image::{Rgba, RgbaImage};
use winit::event::VirtualKeyCode;

use crate::{
    display::utils::draw_text2,
    gui::{
        draw::{draw_background, draw_button_background},
        uisettings::UISettings,
    },
    model::{maths::vec2::Vec2, scene::Scene},
    GUI_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use super::{
    uibox::UIBox,
    uielement::{ElemType, Property, UIElement},
    utils::{get_pos, get_size, give_back_element, take_element},
    HitBox,
};

#[derive(Clone)]
pub struct Editing {
    pub reference: String,
    pub value: String,
}

pub struct UI {
    boxes: HashMap<String, UIBox>,
    settings: UISettings,
    box_index: usize,
    active_box_reference: String,
    editing: Option<Editing>,
    mouse_position: (u32, u32),
    inputs: Vec<VirtualKeyCode>,
    hitbox_vec: Vec<HitBox>,
    dirty: bool,
}

impl UI {
    pub fn default() -> Self {
        UI {
            box_index: 0,
            boxes: HashMap::new(),
            settings: UISettings::default(),
            active_box_reference: "".to_string(),
            editing: None,
            mouse_position: (0, 0),
            inputs: vec![],
            dirty: false,
            hitbox_vec: vec![],
        }
    }

    pub fn settings(&self) -> &UISettings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut UISettings {
        &mut self.settings
    }

    pub fn mouse_position(&self) -> (u32, u32) {
        self.mouse_position
    }

    pub fn set_mouse_position(&mut self, pos: (u32, u32)) {
        self.mouse_position = pos;
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn editing(&self) -> &Option<Editing> {
        &self.editing
    }

    pub fn set_editing(&mut self, editing: Option<Editing>) {
        self.editing = editing;
        self.dirty = true;
    }

    pub fn get_box(&self, reference: String) -> &UIBox {
        self.boxes
            .get(&reference)
            .expect("ERROR : Couldn't find the added UIBox")
    }

    pub fn get_box_mut(&mut self, reference: String) -> &mut UIBox {
        self.boxes
            .get_mut(&reference)
            .expect("ERROR : Couldn't find the added UIBox")
    }

    pub fn get_element_by_reference_mut(&mut self, reference: String) -> Option<&mut UIElement> {
        for uibox in self.boxes.values_mut() {
            for elem in &mut uibox.elems {
                if let Some(elem) = elem.get_element_by_reference_mut(&reference) {
                    return Some(elem);
                }
            }
        }
        None
    }

    pub fn get_property_by_reference(&mut self, reference: &String) -> Option<&mut Property> {
        for uibox in self.boxes.values_mut() {
            for elem in &mut uibox.elems {
                if let Some(property) = elem.get_property_by_reference(reference) {
                    return Some(property);
                }
            }
        }
        None
    }

    pub fn add_box(&mut self, mut uibox: UIBox) -> String {
        if &uibox.reference == "" {
            self.box_index += 1;
            uibox.reference = self.box_index.to_string();
        }
        let reference = uibox.reference.clone();
        self.boxes.insert(reference.clone(), uibox);
        reference
    }

    pub fn active_box(&self) -> Option<&UIBox> {
        if self.active_box_reference == "" {
            None
        } else {
            Some(
                self.boxes
                    .get(&self.active_box_reference)
                    .expect("ERROR : Couldn't find the added UIBox"),
            )
        }
    }

    pub fn active_box_mut(&mut self) -> Option<&mut UIBox> {
        if self.active_box_reference == "" {
            None
        } else {
            Some(
                self.boxes
                    .get_mut(&self.active_box_reference)
                    .expect("ERROR : Couldn't find the added UIBox"),
            )
        }
    }

    pub fn set_active_box(&mut self, id: String) {
        self.active_box_reference = id;
        self.dirty = true;
    }

    pub fn input_pressed(&mut self, pressed: VirtualKeyCode) {
        self.inputs.push(pressed)
    }

    pub fn input_released(&mut self, released: VirtualKeyCode) {
        for i in 0..self.inputs.len() {
            if self.inputs.get(i).unwrap() == &released {
                self.inputs.remove(i);
            }
        }
    }

    pub fn inputs(&self) -> &Vec<VirtualKeyCode> {
        &self.inputs
    }

    pub fn draw(&mut self, scene: &Arc<RwLock<Scene>>, img: &mut RgbaImage) {
        img.fill_with(|| 0);
        let width = self.settings.gui_width;
        let mut hitbox_vec = vec![];
        if self.active_box_reference != "" {
            let uibox = self
                .boxes
                .get(&self.active_box_reference)
                .expect("Destroyed UIBox still referenced as active");
            let pos = uibox.pos;
            let uibox_height = uibox.height(self.settings());
            if let Some(color) = &uibox.background_color {
                for x in pos.0..(pos.0 + width) {
                    for y in pos.1..(pos.1 + uibox_height) {
                        img.put_pixel(x as u32, y as u32, color.to_rgba());
                    }
                }
            }

            let mut height = 0;
            for elem in &uibox.elems {
                if elem.visible {
                    let hitbox = HitBox {
                        pos: get_pos(uibox.pos, (0, pos.1 + height), 0),
                        size: get_size(&elem.text, &elem.format),
                        reference: elem.reference.clone(),
                    };
                    let vec = elem.draw(img, self, scene, &hitbox);
                    height += hitbox.size.1 + self.settings().margin;
                    hitbox_vec.push(hitbox);
                    for hitbox in vec {
                        height += hitbox.size.1 + self.settings().margin;
                        hitbox_vec.push(hitbox)
                    }
                }
            }
            if let Some(edit_bar) = &uibox.edit_bar {
                todo!("Handle edit bar")
            }
        }
        self.dirty = false;
        self.hitbox_vec = hitbox_vec;
    }
}

pub fn ui_clicked(click: (u32, u32), scene: &Arc<RwLock<Scene>>, ui: &mut UI) -> bool {
    let hitbox_list = ui.hitbox_vec.split_off(0);
    for hitbox in hitbox_list {
        if click.0 > hitbox.pos.0
            && click.0 < hitbox.pos.0 + hitbox.size.0
            && click.1 > hitbox.pos.1
            && click.1 < hitbox.pos.1 + hitbox.size.1
        {
            if let Some((mut elem, parent_ref, index)) = take_element(ui, hitbox.reference) {
                elem.clicked(scene, ui);
                give_back_element(ui, elem, parent_ref, index);
                return true;
            }
        }
    }
    false
}
