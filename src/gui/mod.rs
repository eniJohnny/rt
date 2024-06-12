pub mod draw;
pub mod utils;
pub mod textformat;

use crate::
    model::{maths::vec2::Vec2, Element}
;

#[derive(Debug, PartialEq, Clone)]
pub struct Gui {
    keys: Vec<String>,
    values: Vec<String>,
    hitboxes: Vec<(Vec2, Vec2)>,
    apply_hitbox: (Vec2, Vec2),
    cancel_hitbox: (Vec2, Vec2),
    element_index: usize,
    updating: bool,
    updating_index: usize,
    is_open: bool,
    light_index: i32,
    displaying: String,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            hitboxes: Vec::new(),
            apply_hitbox: (Vec2::new(1250., 540.), Vec2::new(1350., 580.)),
            cancel_hitbox: (Vec2::new(1440., 540.), Vec2::new(1550., 580.)),
            element_index: 0,
            updating: false,
            updating_index: 0,
            is_open: false,
            light_index: -1,
            displaying: "".to_string(),
        }
    }

    pub fn keys(&self) -> &Vec<String> {
        &self.keys
    }
    pub fn values(&self) -> &Vec<String> {
        &self.values
    }
    pub fn hitboxes(&self) -> &Vec<(Vec2, Vec2)> {
        &self.hitboxes
    }
    pub fn apply_hitbox(&self) -> &(Vec2, Vec2) {
        &self.apply_hitbox
    }
    pub fn cancel_hitbox(&self) -> &(Vec2, Vec2) {
        &self.cancel_hitbox
    }
    pub fn element_index(&self) -> usize {
        self.element_index
    }
    pub fn updating(&self) -> bool {
        self.updating
    }
    pub fn updating_index(&self) -> usize {
        self.updating_index
    }
    pub fn is_open(&self) -> bool {
        self.is_open
    }
    pub fn light_index(&self) -> i32 {
        self.light_index
    }
    pub fn displaying(&self) -> &String {
        &self.displaying
    }

    pub fn set_element_index(&mut self, index: usize) {
        self.element_index = index;
    }
    pub fn set_updating(&mut self, updating: bool) {
        self.updating = updating;
    }
    pub fn set_updating_index(&mut self, index: usize) {
        self.updating_index = index;
    }
    pub fn set_updates(&mut self, index: usize, value: &String, hitbox: &(Vec2, Vec2)) {
        self.values[index] = value.to_string();
        self.hitboxes[index] = hitbox.clone();
    }
    pub fn set_is_open(&mut self, is_open: bool) {
        self.is_open = is_open;
    }
    pub fn set_light_index(&mut self, light_index: i32) {
        self.light_index = light_index;
    }
    pub fn set_displaying(&mut self, displaying: &String) {
        self.displaying = displaying.to_string();
    }

}