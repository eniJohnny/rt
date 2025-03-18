use crate::{ui::uisettings::UISettings, OBJECTS, SCENE_TOOLBAR, SCROLL_PIXEL_AMOUNT, SETTINGS, TOOLBAR};
use image::Rgba;
use winit::keyboard::Key;
use std::
    collections::HashMap
;
use super::{
    uibox::UIBox,
    uieditbar::UIEditBar,
    uielement::UIElement,
     utils::{
        draw_utils::is_inside_box, misc::Property, style::StyleBuilder, ui_utils::{get_parent_ref, give_back_element, take_element, Editing, UIContext}, HitBox
    }
};

pub struct UI {
    boxes: HashMap<String, UIBox>,
    uisettings: UISettings,
    box_index: usize,
    active_box_queue: Vec<String>,
    editing: Option<Editing>,
    mouse_position: (u32, u32),
    inputs: Vec<Key>,
    hitbox_vec: Vec<HitBox>,
    dirty: bool
}

impl UI {
    pub fn default() -> Self {
        UI {
            box_index: 0,
            boxes: HashMap::new(),
            uisettings: UISettings::default(),
            active_box_queue: vec![],
            editing: None,
            mouse_position: (0, 0),
            inputs: vec![],
            dirty: true,
            hitbox_vec: vec![],
        }
    }

    pub fn uisettings(&self) -> &UISettings {
        &self.uisettings
    }

    pub fn uisettings_mut(&mut self) -> &mut UISettings {
        &mut self.uisettings
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

    pub fn get_box(&self, reference: &str) -> Option<&UIBox> {
        if let Some(uibox) = self.boxes.get(reference) {
            return Some(uibox);
        }
        None
    }

    pub fn get_box_mut(&mut self, reference: &str) -> Option<&mut UIBox> {
        if let Some(uibox) = self.boxes.get_mut(reference) {
            return Some(uibox);
        }
        None
    }

    pub fn take_box(&mut self, reference: &str) -> UIBox {
        self.boxes.remove(reference).expect("UIBox not found")
    }

    pub fn give_back_box(&mut self, uibox: UIBox) {
        self.boxes.insert(uibox.reference.clone(), uibox);
    }

    pub fn destroy_box(&mut self, reference: &str) {
        self.boxes.remove(reference);
        if let Some(last_reference) = self.active_box_queue.last() {
            if reference == *last_reference {
                self.active_box_queue.pop();
            }
        }
    }

    pub fn destroy_last_box(&mut self) {
        let mandatory_boxes = [TOOLBAR, SCENE_TOOLBAR];
        let mut last_reference = None;
        for (value, _) in &self.boxes {
            if !mandatory_boxes.contains(&value.as_str()) {
                last_reference = Some(value.clone());
            }
        }
        let settings = self.uisettings.clone();
        if let Some(reference) = last_reference {
            if reference == SETTINGS || reference == OBJECTS{
                if let Some(elem) = self.get_element_mut(format!("{}.row.{}", SCENE_TOOLBAR, reference)) {
                    elem.set_style(StyleBuilder::from_existing(&elem.style, &settings)
                        .bg_color(Some(Rgba([200, 200, 200, 255])))
                        .build()
                    );
                }
            }
            self.destroy_box(&reference);
        }
    }

    pub fn get_element(&self, reference: String) -> Option<&UIElement> {
        for uibox in self.boxes.values() {
            if let Some(element) =  uibox.get_element(&reference) {
                return Some(element);
            }
        }
        None
    }

    pub fn get_element_mut(&mut self, reference: String) -> Option<&mut UIElement> {
        for uibox in self.boxes.values_mut() {
            if let Some(element) =  uibox.get_element_mut(&reference) {
                return Some(element);
            }
        }
        None
    }

    pub fn remove_element_by_reference(&mut self, reference: String) -> Option<UIElement> {
        for (_, uibox) in &mut self.boxes {
            let mut index = 0;
            for element in &mut uibox.elems {
                if element.reference == reference {
                    break;
                }
                if let Some(element) = element.remove_element_by_reference(&reference) {
                    return Some(element);
                }
                index += 1;
            }
            if index != uibox.elems.len() {
                uibox.elems.remove(index);
                break;
            }
        }
        None
    }

    pub fn get_property(&self, reference: &String) -> Option<&Property> {
        for uibox in self.boxes.values() {
            if let Some(property) = uibox.get_property(reference) {
                return Some(property);
            }
        }
        println!("Property {} not found", reference);
        None
    }

    pub fn get_property_mut(&mut self, reference: &String) -> Option<&mut Property> {
        for uibox in self.boxes.values_mut() {
            if let Some(property) = uibox.get_property_mut(reference) {
                return Some(property);
            }
        }
        println!("Property {} not found", reference);
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
        if self.active_box_queue.len() == 0 {
            None
        } else {
            let last_reference = self.active_box_queue.last().expect("ERROR : No last element in active_box_queue despite len > 0");
            Some(
                self.boxes
                    .get(last_reference)
                    .expect("ERROR : Couldn't find the added UIBox"),
            )
        }
    }

    pub fn active_box_mut(&mut self) -> Option<&mut UIBox> {
        if self.active_box_queue.len() == 0 {
            None
        } else {
            let last_reference = self.active_box_queue.last().expect("ERROR : No last element in active_box_queue despite len > 0");
            Some(
                self.boxes
                    .get_mut(last_reference)
                    .expect("ERROR : Couldn't find the added UIBox"),
            )
        }
    }

    pub fn active_box_reference(&self) -> String {
        if let Some(last_reference) = self.active_box_queue.last() {
            last_reference.clone()
        } else {
            "".to_string()
        }
    }

    pub fn set_active_box(&mut self, id: String) {
        self.active_box_queue.push(id);
        self.dirty = true;
    }

    pub fn input_pressed(&mut self, pressed: Key) {
        if !self.inputs.contains(&pressed) {
            self.inputs.push(pressed)
        }
    }

    pub fn input_released(&mut self, released: Key) {
        if self.inputs.contains(&released) {}
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

    pub fn validate_properties(&mut self, reference: String) -> bool {
        let uibox = self.get_box(&reference);
        let mut error = None;
        if let Some(uibox) = uibox {
            for elem in &uibox.elems {
                if let Err(e) = elem.validate_properties(&self) {
                    error = Some(e);
                    break;
                }
            }
        }
        let uibox = self.get_box_mut(&reference);
        if let Some(uibox) = uibox {
            if let Some(edit_bar) = &mut uibox.edit_bar {
                if let Some(error) = error {
                    edit_bar.text.0 = Some(error);
                    return false;
                } else {
                    edit_bar.text.0 = None;
                }
            }
        }
        return true;
    }

    pub fn generate_hitboxes(&mut self, context: &UIContext) {
        let settings_snapshot = self.uisettings.clone();
        let mut reference_vec = vec![];
        let mut hitbox_vec = vec![];

        for key in self.boxes.keys() {
            reference_vec.push(key.to_string());
        }

        for reference in reference_vec {
            let mut uibox = self.boxes.remove(&reference).unwrap();
            if !uibox.style.visible {
                continue;
            }
            hitbox_vec.append(&mut uibox.generate_hitboxes(self, context, &settings_snapshot));
            self.boxes.insert(reference, uibox);
        }
        self.hitbox_vec = hitbox_vec;
    }

    pub fn draw(&mut self, context: &mut UIContext) {
        context.ui_img.fill_with(|| 1);
        for (_, uibox) in &self.boxes {
            if !uibox.style.visible || uibox.reference == *self.active_box_reference() {
                continue;
            }
            uibox.draw(self, context);
        }
        if let Some(active_box) = self.active_box() {
            active_box.draw(self, context);
        }
        self.dirty = false;
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }
}

pub fn ui_scrolled(pos: (u32, u32), scroll:f32, _context: &mut UIContext, ui: &mut UI) {
    if let Some(active_box) = ui.active_box_mut() {
        if !is_inside_box(pos, active_box.absolute_pos, active_box.size) {
            return;
        }
        if (scroll < 0. && active_box.offset > 0) || active_box.scrollable {
            active_box.offset = (active_box.offset as f32 + scroll * SCROLL_PIXEL_AMOUNT as f32) as u32;
            ui.set_dirty();
        }
    }
    for (_, uibox) in &mut ui.boxes {
        if is_inside_box(pos, uibox.absolute_pos, uibox.size) {
            if (scroll < 0. && uibox.offset > 0) || uibox.scrollable {
                uibox.offset = (uibox.offset as f32 + scroll * SCROLL_PIXEL_AMOUNT as f32) as u32;
                ui.set_dirty();
            }
            return;
        }
    }
}

pub fn ui_clicked(click: (u32, u32), context: &mut UIContext, ui: &mut UI) -> bool {
    let hitbox_list = ui.hitbox_vec.split_off(0);
    let mut active_box_ref: String = "".to_string();
    if let Some(active_box) = ui.active_box() {
        if !is_inside_box(click, active_box.absolute_pos, active_box.size) {
            return false;
        }
        active_box_ref = active_box.reference.clone();
    }
    for hitbox in hitbox_list {
        if !hitbox.disabled
            && hitbox.reference.starts_with(&active_box_ref)
            && is_inside_box(click, (hitbox.pos.0 as u32, hitbox.pos.1 as u32), hitbox.size)
        {
            if hitbox.reference.ends_with("btnApply") || hitbox.reference.ends_with("btnCancel") {
                let box_ref = get_parent_ref(hitbox.reference.clone());
                let uibox = ui.get_box_mut(&box_ref);
                if let Some(uibox) = uibox {
                    if let Some(_) = uibox.edit_bar {
                        if hitbox.reference.ends_with("btnApply") {
                            UIEditBar::apply(context, ui, box_ref);
                        } else if hitbox.reference.ends_with("btnCancel") {
                            UIEditBar::cancel(ui, box_ref);
                        }
                    }
                }
                return true;
            } else {
                if let Some((mut elem, parent_ref, index)) =
                    take_element(ui, hitbox.reference.clone())
                {
                    elem.clicked(context, ui);
                    give_back_element(ui, elem, parent_ref, index);
                    return true;
                } else {
                    println!("ERROR: UIElement {} not found", &hitbox.reference)
                }
            }
        }
    }
    for (_, uibox) in &ui.boxes {
        if uibox.style().visible && is_inside_box(click, uibox.absolute_pos, uibox.size) {
            return true;
        }
    }
    false
}
