use std::sync::{Arc, RwLock};

use crate::model::{
    materials::{color::Color, diffuse::Diffuse, texture::Texture},
    maths::{vec2::Vec2, vec3::Vec3},
    scene::Scene,
    shapes::Shape,
};

pub enum Value {
    Text(String),
    Texture(Texture),
    Vector(Vec3),
    Color(Color),
    Float(f64),
    Bool(bool),
}

pub enum FieldType {
    Text,
    Stat(Box<dyn Fn(&Scene)>),
    Property(Property),
    Category(Category),
    Button(Box<dyn Fn(&mut Scene)>),
}

pub struct Field {
    visible: bool,
    field_type: FieldType,
}

pub struct Category {
    fields: Vec<Field>,
    collapsed: bool,
}

pub struct Property {
    value: Value,
    on_change: Box<dyn Fn(Self, &mut Scene)>,
}

pub trait Displayable {
    fn get_fields() -> Vec<Field>;
}
