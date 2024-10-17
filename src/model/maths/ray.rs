use std::fmt::{Display, Formatter, Result};

use super::vec3::Vec3;

#[derive(Clone, Debug)]
pub struct Ray {
    pos: Vec3,
    dir: Vec3,
    depth: u8,
    sampling: bool,
    pub debug: bool,
}

impl Ray {
    pub fn new(pos: Vec3, dir: Vec3, depth: u8) -> Self {
        Self {
            pos,
            dir,
            depth,
            sampling: false,
            debug: false,
        }
    }

    pub fn get_pos(&self) -> &Vec3 {
        return &self.pos;
    }
    pub fn get_dir(&self) -> &Vec3 {
        return &self.dir;
    }
    pub fn get_depth(&self) -> u8 {
        return self.depth;
    }
    pub fn set_sampling(&mut self, sampling: bool) {
        self.sampling = sampling;
    }
    pub fn is_sampling(&self) -> bool {
        self.sampling
    }
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
    }
    pub fn set_depth(&mut self, depth: u8) {
        self.depth = depth;
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "pos: {}\ndir: {}\ndepth: {}",
            self.pos, self.dir, self.depth
        )
    }
}
