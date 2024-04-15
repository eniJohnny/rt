use std::fmt::{Display, Formatter, Result};

use super::vec3::Vec3;

pub struct Ray {
    pos: Vec3,
    dir: Vec3,
    depth: u8
}

impl Ray {
    pub fn new(pos: Vec3, dir: Vec3, depth: u8) -> Self {
        Self {
            pos,
            dir,
            depth
        }
    }

    pub fn get_pos(&self) -> &Vec3 { return &self.pos; }
    pub fn get_dir(&self) -> &Vec3 { return &self.dir; }
    pub fn get_depth(&self) -> &u8 { return &self.depth; }
}


impl Display for Ray {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "pos: {}\ndir: {}\ndepth: {}", self.pos, self.dir, self.depth)
	}
}

