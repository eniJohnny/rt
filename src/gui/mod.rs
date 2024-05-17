pub mod draw;
pub mod utils;
pub mod textformat;
use crate::{
    model::maths::vec2::Vec2,
    SCREEN_WIDTH,
};

#[derive(Debug, PartialEq)]
pub struct Gui {
    keys: Vec<String>,
    values: Vec<String>,
    hitboxes: Vec<(Vec2, Vec2)>,
    apply_hitbox: (Vec2, Vec2),
    cancel_hitbox: (Vec2, Vec2),
    element_index: usize,
    updating: bool,
    updating_index: usize,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
            hitboxes: Vec::new(),
            apply_hitbox: (Vec2::new((SCREEN_WIDTH - 350) as f64, 540.), Vec2::new((SCREEN_WIDTH - 250) as f64, 580.)),
            cancel_hitbox: (Vec2::new((SCREEN_WIDTH - 160) as f64, 540.), Vec2::new((SCREEN_WIDTH - 50) as f64, 580.)),
            element_index: 0,
            updating: false,
            updating_index: 0,
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
}
