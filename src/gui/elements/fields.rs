use crate::{
    gui::settings::UISettings,
    model::{
        materials::{color::Color, texture::Texture},
        maths::vec3::Vec3,
        scene::Scene,
    },
};

use super::{ui::UI, Displayable, Position};

#[derive(Debug)]
pub enum Value {
    Text(String),
    Texture(Texture),
    Vector(Vec3),
    Color(Color),
    Float(f64),
    Usize(usize),
    Bool(bool),
}

pub enum ElemType {
    TopBar(Option<String>, bool, bool),
    Text,
    Stat(Box<dyn Fn(&Scene)>),
    Property(Property),
    Category(Category),
    Button(Box<dyn Fn(&mut Scene, &mut UI)>, usize, usize),
}

pub struct UIElement {
    visible: bool,
    elem: ElemType,
    name: String,
    pos: Position,
}

impl UIElement {
    pub fn new(name: &str, elem: ElemType, pos: Position) -> Self {
        UIElement {
            visible: true,
            elem,
            name: String::from(name),
            pos,
        }
    }
    pub fn height(&self, settings: &UISettings) -> usize {
        if !self.visible {
            return 0;
        }
        let mut height = settings.field_height;
        if let ElemType::Category(cat) = &self.elem {
            if !cat.collapsed {
                for elem in &cat.elems {
                    height += elem.height(settings);
                }
            }
        }
        height
    }
}

pub struct Category {
    pub elems: Vec<UIElement>,
    pub collapsed: bool,
}

pub struct Property {
    pub value: Value,
    pub on_change: Box<dyn Fn(Value, &mut Scene, &mut UI)>,
}
