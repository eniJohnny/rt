use std::{
    collections::HashMap,
    sync::{
        mpsc::{Receiver, Sender},
        Arc, RwLock,
    },
};

use image::{ImageBuffer, Rgba, RgbaImage};
use winit::keyboard::Key;

use crate::{
    gui::{draw::draw_background, uisettings::UISettings},
    model::scene::Scene,
    SCREEN_HEIGHT_U32, SCREEN_WIDTH_U32,
};

use super::{
    uibox::UIBox,
    uieditbar::UIEditBar,
    uielement::{ElemType, Property, UIElement},
    utils::{get_parent_ref, get_pos, get_size, give_back_element, take_element},
    HitBox,
};

#[derive(Clone)]
pub struct Editing {
    pub reference: String,
    pub value: String,
}

pub struct Statistics {
    pub fps: u32,
}

pub struct UIContext {
    pub ui_img: RgbaImage,
    pub scene_img: RgbaImage,
    pub receiver: Receiver<(ImageBuffer<Rgba<u8>, Vec<u8>>, bool)>,
    pub transmitter: Sender<bool>,
    pub draw_time_avg: f64,
    pub draw_time_samples: u32,
    pub last_input_time: u32,
    pub final_img: bool,
    pub image_asked: bool
}

impl UIContext {
    pub fn new(
        receiver: Receiver<(ImageBuffer<Rgba<u8>, Vec<u8>>, bool)>,
        transmitter: Sender<bool>,
    ) -> Self {
        Self {
            ui_img: RgbaImage::new(SCREEN_WIDTH_U32, SCREEN_HEIGHT_U32),
            scene_img: RgbaImage::new(SCREEN_WIDTH_U32, SCREEN_HEIGHT_U32),
            receiver,
            transmitter,
            draw_time_avg: 0.,
            draw_time_samples: 0,
            last_input_time: 0,
            final_img: false,
            image_asked: false
        }
    }
}

pub struct UI {
    boxes: HashMap<String, UIBox>,
    uisettings: UISettings,
    box_index: usize,
    active_box_reference: String,
    editing: Option<Editing>,
    mouse_position: (u32, u32),
    inputs: Vec<Key>,
    hitbox_vec: Vec<HitBox>,
    dirty: bool,
    context: Option<UIContext>,
}

impl UI {
    pub fn default(
        receiver: Receiver<(ImageBuffer<Rgba<u8>, Vec<u8>>, bool)>,
        transmitter: Sender<bool>,
    ) -> Self {
        UI {
            box_index: 0,
            boxes: HashMap::new(),
            uisettings: UISettings::default(),
            active_box_reference: "".to_string(),
            editing: None,
            mouse_position: (0, 0),
            inputs: vec![],
            dirty: true,
            hitbox_vec: vec![],
            context: Some(UIContext::new(receiver, transmitter)),
        }
    }

    pub fn uisettings(&self) -> &UISettings {
        &self.uisettings
    }

    pub fn uisettings_mut(&mut self) -> &mut UISettings {
        &mut self.uisettings
    }

    pub fn context(&self) -> Option<&UIContext> {
        self.context.as_ref()
    }

    pub fn context_mut(&mut self) -> Option<&mut UIContext> {
        self.context.as_mut()
    }

    pub fn take_context(&mut self) -> UIContext {
        self.context.take().unwrap()
    }

    pub fn give_back_context(&mut self, context: UIContext) {
        self.context = Some(context);
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

    pub fn refresh_formats(&mut self) {
        let new_settings = self.uisettings().clone();
        for (_, uibox) in &mut self.boxes {
            uibox.refresh_formats(&new_settings);
        }
        self.set_dirty()
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
        self.boxes.get_mut(&reference).expect(&format!(
            "ERROR : Couldn't find the added UIBox {}",
            reference
        ))
    }

    pub fn get_element_by_reference_mut(&mut self, reference: String) -> Option<&mut UIElement> {
        for uibox in self.boxes.values_mut() {
            for elem in &mut uibox.elems {
                if let Some(elem) = elem.get_element_by_reference_mut(&reference) {
                    return Some(elem);
                }
            }
        }
        println!("Element {} not found", reference);
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

    pub fn input_pressed(&mut self, pressed: Key) {
        if !self.inputs.contains(&pressed) {
            self.inputs.push(pressed)
        }
    }

    pub fn input_released(&mut self, released: Key) {
        if self.inputs.contains(&released) {
        }
        for i in 0..self.inputs.len() {
            if let Some(input) = self.inputs().get(i) {
                if input == &released {
                    self.inputs.swap_remove(i);
                }
            }
        }
    }

    pub fn inputs(&self) -> &Vec<Key> {
        &self.inputs
    }

    pub fn validate_properties(&mut self, reference: String) {
        let uibox = self.get_box_mut(reference.clone());
        let mut error = None;
        for elem in &mut uibox.elems {
            if let Err(e) = elem.validate_properties() {
                error = Some(e);
                break;
            }
        }
        if let Some(edit_bar) = &mut uibox.edit_bar {
            if let Some(error) = error {
                edit_bar.text.0 = Some(error);
                return;
            } else {
                edit_bar.text.0 = None
            }
        }
    }

    pub fn process(&mut self, scene: &Arc<RwLock<Scene>>) {
        let settings_snapshot = self.uisettings.clone();
        let mut reference_vec = vec![];
        let mut hitbox_vec = vec![];

        for key in self.boxes.keys() {
            reference_vec.push(key.to_string());
        }

        for reference in reference_vec {
            let mut uibox = self.boxes.remove(&reference).unwrap();
            if !uibox.visible {
                continue;
            }
            let mut offset_y = 0;
            uibox.size.1 = uibox.max_height;
            for i in 0..uibox.elems.len() {
                let mut elem = uibox.elems.remove(i);
                if elem.visible {
                    let hitbox = HitBox {
                        pos: get_pos(uibox.pos, (0, uibox.pos.1 + offset_y), 0),
                        size: get_size(
                            &elem.text,
                            &elem.style,
                            (uibox.size.0, uibox.size.1 - offset_y),
                        ),
                        reference: elem.reference.clone(),
                        disabled: matches!(elem.elem_type, ElemType::Row(_)),
                    };
                    elem.hitbox = Some(hitbox.clone());
                    let vec = elem.process(self, scene, uibox.max_height - offset_y);
                    let needed_height =
                        hitbox.pos.1 + hitbox.size.1 + settings_snapshot.margin - uibox.pos.1;
                    if needed_height >= uibox.size.1 {
                        break;
                    }
                    if needed_height > offset_y {
                        offset_y = needed_height;
                    }
                    hitbox_vec.push(hitbox);

                    for hitbox in vec {
                        let needed_height =
                            hitbox.pos.1 + hitbox.size.1 + settings_snapshot.margin - uibox.pos.1;
                        if needed_height > offset_y {
                            offset_y = needed_height;
                        }
                        hitbox_vec.push(hitbox)
                    }
                }
                uibox.elems.insert(i, elem);
            }
            if let Some(mut edit_bar) = uibox.edit_bar.take() {
                let mut vec = edit_bar.process(
                    (uibox.pos.0, uibox.pos.1 + offset_y),
                    &self.uisettings,
                    uibox.size,
                );
                offset_y = vec[1].pos.1 + vec[1].size.1 + self.uisettings().margin * 2;
                hitbox_vec.append(&mut vec);
                uibox.edit_bar = Some(edit_bar);
            }
            uibox.size.1 = offset_y;
            self.boxes.insert(reference, uibox);
        }
        self.hitbox_vec = hitbox_vec;
    }

    pub fn draw(&mut self, scene: &Arc<RwLock<Scene>>, img: &mut RgbaImage) {
        img.fill_with(|| 1);
        for (_, uibox) in &self.boxes {
            if !uibox.visible {
                continue;
            }
            if let Some(color) = &uibox.background_color {
                draw_background(img, uibox.pos, uibox.size, color.to_rgba(), 0);
            }
            for elem in &uibox.elems {
                if elem.visible {
                    elem.draw(img, self, scene);
                }
            }
            if let Some(edit_bar) = &uibox.edit_bar {
                edit_bar.draw(img);
            }
        }
        self.dirty = false;
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }
}

pub fn ui_clicked(click: (u32, u32), scene: &Arc<RwLock<Scene>>, ui: &mut UI) -> bool {
    let hitbox_list = ui.hitbox_vec.split_off(0);
    for hitbox in hitbox_list {
        if !hitbox.disabled
            && click.0 > hitbox.pos.0
            && click.0 < hitbox.pos.0 + hitbox.size.0
            && click.1 > hitbox.pos.1
            && click.1 < hitbox.pos.1 + hitbox.size.1
        {
            if hitbox.reference.ends_with("btnApply") || hitbox.reference.ends_with("btnCancel") {
                let box_ref = get_parent_ref(hitbox.reference.clone());
                let uibox = ui.get_box_mut(box_ref.clone());
                if let Some(_) = uibox.edit_bar {
                    if hitbox.reference.ends_with("btnApply") {
                        UIEditBar::apply(&mut scene.write().unwrap(), ui, box_ref);
                    } else if hitbox.reference.ends_with("btnCancel") {
                        UIEditBar::cancel(&mut scene.write().unwrap(), ui, box_ref);
                    }
                }
                return true;
            } else {
                if let Some((mut elem, parent_ref, index)) =
                    take_element(ui, hitbox.reference.clone())
                {
                    elem.clicked(scene, ui);
                    give_back_element(ui, elem, parent_ref, index);
                    return true;
                } else {
                    println!("ERROR: UIElement {} not found", &hitbox.reference)
                }
            }
        }
    }
    false
}
