use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use image::{Rgba, RgbaImage};
use winit::event::VirtualKeyCode;

use crate::{
    display::utils::draw_text2,
    gui::{
        draw::{draw_button_background, draw_background},
        uisettings::UISettings,
    },
    model::{maths::vec2::Vec2, scene::Scene},
    GUI_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use super::{
    uibox::UIBox,
    uielement::{ElemType, Property, UIElement},
    Position,
};

#[derive(Clone)]
pub struct Editing {
    pub reference: String,
    pub value: String,
}

pub struct UI {
    boxes: HashMap<String, UIBox>,
    inlined: Vec<String>,
    settings: UISettings,
    box_index: usize,
    active_box_reference: String,
    editing: Option<Editing>,
    mouse_position: (u32, u32),
    inputs: Vec<VirtualKeyCode>,
    dirty: bool,
}

impl UI {
    pub fn default() -> Self {
        UI {
            box_index: 0,
            boxes: HashMap::new(),
            inlined: vec![],
            settings: UISettings::default(),
            active_box_reference: "".to_string(),
            editing: None,
            mouse_position: (0, 0),
            inputs: vec![],
            dirty: false,
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

    // pub fn get_element_by_reference(&self, reference: String) -> Option<&UIElement> {
    //     for uibox in self.boxes.values_mut() {
    //         for elem in &mut uibox.elems {
    //             if let Some(elem) = elem.get_element_by_reference(reference) {
    //                 return Some(elem);
    //             }
    //         }
    //     }
    //     None
    // }

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

    // fn get_inlined_box_pos(&self, box_index: usize) -> (usize, usize) {
    //     let mut pos = (self.settings.padding, self.settings.padding);
    //     for index in &self.inlined {
    //         if *index == box_index {
    //             return pos;
    //         }
    //         let uibox = self
    //             .boxes
    //             .get(&index)
    //             .expect("Inlined boxes' indexes are out of date").height(&self.settings);
    //     }
    //     pos
    // }

    pub fn draw(&mut self, scene: &Arc<RwLock<Scene>>, img: &mut RgbaImage) {
        img.fill_with(|| 0);
        let width = self.settings.gui_width;
        let pos_x = SCREEN_WIDTH as u32 - self.settings.gui_width;
        let mut pos_y = 0;
        if self.active_box_reference != "" {
            let uibox = self
                .boxes
                .get(&self.active_box_reference)
                .expect("Destroyed UIBox still referenced as active");
            let uibox_height = uibox.height(self.settings().margin);
            if let Some(color) = &uibox.background_color {
                for x in pos_x..(pos_x + width) {
                    for y in pos_y..(pos_y + uibox_height) {
                        img.put_pixel(x as u32, y as u32, color.to_rgba());
                    }
                }
            }

            for elem in &uibox.elems {
                if elem.visible {
                    pos_y += elem.draw(img, &self, scene, uibox.pos, (self.settings().gui_width, uibox_height), (0, pos_y), 0);
                }
            }
            if let Some(edit_bar) = &uibox.edit_bar {
                for elem in vec![&edit_bar.txt_message, &edit_bar.btn_apply, &edit_bar.btn_cancel] {
                    if elem.visible {
                        elem.draw(img, &self, scene, uibox.pos, (self.settings().gui_width, uibox_height), (0, pos_y), 0);
                    }
                }
            }
        }
        self.dirty = false;
    }
}

pub fn ui_clicked(click: (u32, u32), scene: &Arc<RwLock<Scene>>, ui: &mut UI) -> bool {
    let mut new_elems = vec![];
    let gui_width = ui.settings.gui_width;
    let margin = ui.settings().margin;
    let mut clicked = false;
    let mut edit_bar_opt = None;
    let mut box_pos = (0, 0);
    let mut box_size = (0, 0);
    let mut inline_pos = (0, 0);
    if let Some(uibox) = ui.active_box_mut() {
        new_elems = uibox.elems.split_off(0);
        box_pos = uibox.pos;
        box_size = (gui_width, uibox.height(margin));
        inline_pos = (0, 0);
        edit_bar_opt = uibox.edit_bar.take();
        if click.0 > box_pos.0 && click.0 < box_pos.0 + box_size.0 && click.1 > box_pos.1 && click.1 < box_pos.1 + box_size.1 {
            let mut height = 0;
            for elem in &mut new_elems {
                height += elem.clicked(click, box_pos, box_size, 0, (inline_pos.0, inline_pos.1 + height), scene, ui);
            }
            clicked = true;
        }
    }
    if let Some(mut edit_bar) =  edit_bar_opt {   
        for elem in vec![&mut edit_bar.btn_apply, &mut edit_bar.btn_cancel, &mut edit_bar.txt_message] {
            elem.clicked(click, box_pos, box_size, 0, inline_pos, scene, ui);
        }
        edit_bar_opt = Some(edit_bar);
    }
    if let Some(uibox) = ui.active_box_mut() {
        uibox.elems.append(&mut new_elems);
        uibox.edit_bar = edit_bar_opt;
    }
    clicked
}