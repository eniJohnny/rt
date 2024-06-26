use std::sync::{Arc, RwLock};

use image::{Rgba, RgbaImage};


use crate::{display::utils::draw_text2, gui::{textformat::{Formattable, TextFormat}, uisettings::UISettings}, model::{materials::texture::Texture, scene::Scene}};

use super::{ui::UI, Displayable, Position};

#[derive(Debug, Clone)]
pub enum Value {
    Text(String),
    Float(f64),
    Unsigned(u32),
    Bool(bool),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Text(str) => str.clone(),
            Value::Bool(bool) => bool.to_string(),
            Value::Float(float) => float.to_string(),
            Value::Unsigned(unsigned) => unsigned.to_string()
        }
    }
}

impl Formattable for Value {
    fn base_format(&self, settings: &UISettings) -> TextFormat {
        TextFormat::new_editing_format(settings)
    }
}

pub enum ElemType {
    Text,
    Stat(Box<dyn Fn(&Scene) -> String>),
    Property(Property),
    Category(Category),
    Button(Box<dyn Fn(&mut Scene, &mut UI)>, usize, usize),
}

impl Formattable for ElemType {
    fn base_format(&self, settings: &UISettings) -> TextFormat {
        match self {
            ElemType::Button(..) => {
                let mut format = TextFormat::default();
                format.set_font_size(36.);
                format
            },
            ElemType::Category(..) => {
                TextFormat::default()
            },
            ElemType::Property(..) => {
                let mut format = TextFormat::default();
                format.set_background_color(Rgba([89, 89, 89, 255]));
                format
            },
            ElemType::Stat(..) => {
                TextFormat::default()
            },
            ElemType::Text => {
                TextFormat::default()
            }
        }
    }
}

pub struct UIElement {
    visible: bool,
    pub elem_type: ElemType,
    name: String,
    pos: Position,
    format: TextFormat,
    pub position: (u32, u32),
    pub size: (u32, u32),
    pub reference: String,
}

impl UIElement {
    pub fn new(name: &str, reference: String, elem: ElemType, pos: Position, settings: &UISettings) -> Self {
        UIElement {
            visible: true,
            format: elem.base_format(settings),
            elem_type: elem,
            name: String::from(name),
            pos,
            position: (0, 0),
            size: (0, 0),
            reference: reference + "." + name
        }
    }
    pub fn height(&self, settings: &UISettings) -> u32 {
        if !self.visible {
            return 0;
        }
        let mut height = settings.field_height + settings.margin;
        if let ElemType::Category(cat) = &self.elem_type {
            if !cat.collapsed {
                for elem in &cat.elems {
                    height += elem.height(settings);
                }
            }
        }
        height
    }

    pub fn get_property_by_reference(&mut self, reference: &String) -> Option<&mut Property> {
        match &mut self.elem_type {
            ElemType::Property(property) => {
                if &self.reference == reference {
                    return Some(property);
                }
            }
            ElemType::Category(cat) => {
                for elem in &mut cat.elems {
                    if let Some(property) = elem.get_property_by_reference(reference) {
                        return Some(property);
                    }
                }
            }
            _ => ()
        }
        None
    }

    pub fn draw(&self, img: &mut RgbaImage, ui: &UI, scene: &Arc<RwLock<Scene>>, pos_x: u32, pos_y: u32, indent: u32) -> u32 {
        let mut height = 0;
        match &self.elem_type {
            ElemType::Button(..) => {
                todo!()
            },
            ElemType::Category(cat) => {
                draw_text2(img, (pos_x, pos_y), self.name.clone(), &self.format, ui.settings(), indent);
                height += ui.settings().field_height + self.format.padding_bot + self.format.padding_top;
                if !cat.collapsed {
                    for elem in &cat.elems {
                        if elem.visible {
                            let elem_height = elem.draw(img, ui, scene, pos_x, pos_y + height, indent + 1) + ui.settings().margin;
                            height += elem_height;
                        }
                    }
                }
            },
            ElemType::Property(property) => {
                draw_text2(img, (pos_x, pos_y), self.name.clone(), &self.format, ui.settings(), indent);
                let format;
                let value;
                if let Some(edit) = ui.editing() {
                    if &self.reference == &edit.reference {
                        value = edit.value.clone() + "_";
                        format = &property.editing_format;
                        println!("{}, {}", value, edit.value.len());
                    } else {
                        value = property.value.to_string();
                        format = &self.format;
                    }
                } else {
                    value = property.value.to_string();
                    format = &self.format;
                }
                let offset = ui.settings().gui_width - value.len() as u32 * 10 - format.padding_right - format.padding_left;
                draw_text2(img, (pos_x + offset, pos_y), value.to_string(), &format, ui.settings(), indent);
                height += ui.settings().field_height + format.padding_top + format.padding_bot;
            },
            ElemType::Stat(function) => {
                draw_text2(img, (pos_x, pos_y), self.name.clone(), &self.format, ui.settings(), indent);
                let value = function(&scene.read().unwrap());
                let offset = ui.settings().gui_width - value.to_string().len() as u32 * 10 - self.format.padding_right - self.format.padding_left;
                draw_text2(img, (pos_x + offset, pos_y), self.name.clone(), &self.format, ui.settings(), indent);
                height += ui.settings().field_height + self.format.padding_top + self.format.padding_bot;
            },
            ElemType::Text => {
                draw_text2(img, (pos_x, pos_y), self.name.clone(), &self.format, ui.settings(), indent);
                height += ui.settings().field_height + self.format.padding_top + self.format.padding_bot;
            }
        }
        height
    }
}

pub struct Category {
    pub elems: Vec<UIElement>,
    pub collapsed: bool,
}

type FnSubmit = Box<dyn Fn(Value, &mut Scene, &mut UI)>;
type FnValidate = Box<dyn Fn(Value, &Scene, &UI) -> Result<(), &'static str>>;

pub struct Property {
    pub value: Value,
    pub initial_value: Value,
    pub editing_format: TextFormat,
    pub fn_submit: FnSubmit,
    pub fn_validate: FnValidate
}

impl Property {
    pub fn new(value: Value, fn_submit: FnSubmit, fn_validate: FnValidate, settings: &UISettings) -> Self {
        Self {
            editing_format: value.base_format(settings),
            initial_value: value.clone(),
            value,
            fn_submit,
            fn_validate
        }
    }

    pub fn set_value_from_string(&mut self, val: String) {
        match self.value {
            Value::Bool(_) => {
                ()
            },
            Value::Float(_) => {
                let val = val.parse::<f64>();
                if val.is_err() {
                    println!("The value is not a proper float");
                } else {
                    self.value = Value::Float(val.unwrap());
                }
            },
            Value::Text(_) => {
                self.value = Value::Text(val);
            }
            Value::Unsigned(_) => {
                let val = val.parse::<u32>();
                if val.is_err() {
                    println!("The value is not a proper unsigned number");
                } else {
                    self.value = Value::Unsigned(val.unwrap());
                }
            },
        }
    }
}