use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use image::{Rgba, RgbaImage};
use winit::event::VirtualKeyCode;

use crate::{
    display::utils::draw_text2,
    gui::{
        draw::{draw_button_background, draw_button_background2},
        uisettings::UISettings,
    },
    model::{maths::vec2::Vec2, scene::Scene},
    GUI_WIDTH, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use super::{
    uibox::UIBox,
    uielement::{ElemType, Property, UIElement},
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
        self.box_index += 1;
        uibox.reference = self.box_index.to_string();
        self.boxes.insert(self.box_index.to_string(), uibox);
        self.box_index.to_string()
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
        // let inline_pos = (self.settings.padding, self.settings.padding);
        let width = self.settings.gui_width;
        let pos_x = SCREEN_WIDTH as u32 - self.settings.gui_width;
        let mut pos_y = 0;
        if self.active_box_reference != "" {
            let uibox = self
                .boxes
                .get(&self.active_box_reference)
                .expect("Destroyed UIBox still referenced as active");
            if let Some(color) = &uibox.background_color {
                for x in pos_x..(pos_x + width) {
                    for y in pos_y..(pos_y + uibox.height(&self.settings)) {
                        img.put_pixel(x as u32, y as u32, color.to_rgba());
                    }
                }
            }

            for elem in &uibox.elems {
                pos_y += elem.draw(img, &self, scene, pos_x, pos_y, 0);
            }

            if let Some(edit_bar) = &uibox.edit_bar {
                draw_text2(
                    img,
                    edit_bar.txt_message.position,
                    edit_bar.txt_message.text.clone(),
                    &edit_bar.txt_message.format,
                    &self.settings,
                    0,
                );
                draw_button_background2(
                    img,
                    (uibox.pos.0 + 50, uibox.height(&self.settings) - 60),
                    (100, 40),
                    Rgba([70, 125, 70, 255]),
                );
            }
        }
        self.dirty = false;
    }

    pub fn clicked(&self, click: (u32, u32)) -> Option<&UIElement> {
        if let Some(uibox) = self.active_box() {
            let mut pos = uibox.pos;
            if click.0 < pos.0 + self.settings.gui_width && click.0 > pos.0 {
                for elem in &uibox.elems {
                    let elem_height = elem.height(self.settings());
                    if click.1 < pos.1 + elem_height && click.1 > pos.1 {
                        return Some(&elem);
                    }
                    pos.1 += elem_height;
                }
            }
        }
        None
    }
}
