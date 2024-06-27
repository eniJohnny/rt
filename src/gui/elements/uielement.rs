use std::{
    cell::RefCell,
    sync::{Arc, RwLock}, thread::current,
};

use image::{Rgba, RgbaImage};

use crate::{
    display::utils::{draw_text2, draw_text_background2},
    gui::{
        draw::draw_background, textformat::{FormatBuilder, Formattable, TextFormat}, uisettings::UISettings
    },
    model::{materials::texture::Texture, scene::Scene}, SCREEN_WIDTH_U32,
};

use super::{ui::{Editing, UI}, uibox::UIBox, Displayable, Position};

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
            Value::Unsigned(unsigned) => unsigned.to_string(),
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
    Button(Box<dyn Fn(&mut Scene, &mut UI)>),
}

impl Formattable for ElemType {
    fn base_format(&self, settings: &UISettings) -> TextFormat {
        match self {
            ElemType::Button(..) => TextFormat::new_btn_format(settings),
            ElemType::Category(..) => TextFormat::new_category_format(settings),
            ElemType::Property(..) => TextFormat::field_format(settings),
            ElemType::Stat(..) => FormatBuilder::default(settings).bg_color(None).build(),
            ElemType::Text => {
                let mut format = TextFormat::field_format(settings);
                format.bg_color = None;
                format
            },
        }
    }
}

pub struct UIElement {
    pub visible: bool,
    pub elem_type: ElemType,
    pub text: String,
    pub format: TextFormat,
    pub pos: Position,
    pub size: (u32, u32),
    pub reference: String,
}

impl UIElement {
    pub fn new(
        name: &str,
        reference: String,
        elem: ElemType,
        pos: Position,
        settings: &UISettings,
    ) -> Self {
        UIElement {
            visible: true,
            format: elem.base_format(settings),
            elem_type: elem,
            text: String::from(name),
            pos,
            size: (0, 0),
            reference: reference + "." + name,
        }
    }
    pub fn height(&self, settings: &UISettings) -> u32 {
        if !self.visible {
            return 0;
        }
        let mut height = self.get_size(&self.text, &self.format).1;
        if let ElemType::Category(cat) = &self.elem_type {
            if !cat.collapsed {
                for elem in &cat.elems {
                    height += elem.height(settings);
                }
            }
        }
        height
    }

    pub fn set_format(&mut self, format: TextFormat) {
        self.format = format;
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
            _ => (),
        }
        None
    }

    pub fn reset_properties(&mut self, scene: &mut Scene) {
        if let ElemType::Category(cat) = &mut self.elem_type {
            for elem in &mut cat.elems {
                elem.reset_properties(scene);
            }
        } else if let ElemType::Property(prop) = &mut self.elem_type {
            prop.value = prop.initial_value.clone();
        }
    }

    pub fn validate_properties(&self) -> Result<(), String> {
        let mut ok = true;
        if let ElemType::Category(cat) = &self.elem_type {
            for elem in &cat.elems {
                elem.validate_properties()?;
            }
        } else if let ElemType::Property(prop) = &self.elem_type {
            (prop.fn_validate)(&prop.value)?;
        }
        Ok(())
    }

    pub fn submit_properties(&self, scene: &mut Scene, ui: &mut UI) {
        if let ElemType::Category(cat) = &self.elem_type {
            for elem in &cat.elems {
                elem.submit_properties(scene, ui);
            }
        } else if let ElemType::Property(prop) = &self.elem_type {
            (prop.fn_submit)(prop.value.clone(), scene, ui);
        }
    }

    pub fn get_pos(&self, pos: &Position, box_pos: (u32, u32), box_size: (u32, u32), inline_pos: (u32, u32), indent: u32) -> (u32, u32) {
        match pos {
            Position::Inline => {
                (box_pos.0 + inline_pos.0 + indent, box_pos.1 + inline_pos.1)
            }
            Position::Relative(x, y) => {
                let pos_x = match x {
                    _ if *x < 0 => box_pos.0 + (box_size.0 as i32 + x) as u32,
                    0 => box_pos.0 + inline_pos.0 + indent,
                    _ => box_pos.0 + *x as u32,
                };
                let pos_y: u32 = match y {
                    _ if *y < 0 => box_pos.1 + (box_size.1 as i32 + y) as u32,
                    0 => box_pos.1 + inline_pos.1,
                    _ => box_pos.1 + *y as u32,
                };
                (pos_x, pos_y)
            }
        }
    }

    pub fn get_size(&self, text: &String, format: &TextFormat) -> (u32, u32) {
        let mut height = format.font_size() as u32 + format.padding_bot + format.padding_top;
        let mut width = format.font_size() as u32 / 2 * text.len() as u32 + format.padding_left + format.padding_top;
        if let ElemType::Text = self.elem_type {
            let available_width = format.width - format.padding_left - format.padding_right;
            let mut current_width = 0;
            let mut lines = 1;

            let mut txt_split = self.text.split(" ");
            while let Some(str) = txt_split.next() {
                let word_width = format.font_size() as u32 * (str.len() + 1) as u32;
                if current_width + word_width > available_width {
                    current_width = word_width;
                    lines += 1;
                } else {
                    current_width += word_width;
                }
            }
            height = lines * format.font_size() as u32 + format.padding_bot + format.padding_top;
        }
        if format.height > height {
            height = format.height;
        }
        if format.width > width {
            width = format.width;
        } 
        (width, height)
    }

    fn draw_text(&self, img: &mut RgbaImage, text: String, box_pos: (u32, u32), box_size: (u32, u32), inline_pos: (u32, u32), format: &TextFormat, indent: u32, position: Position) -> u32 {
        let pos = self.get_pos(&position, box_pos, box_size, inline_pos, indent);
        let size = self.get_size(&text, format);
        if let Some(color) = format.bg_color {
            draw_background(
                img,
                pos,
                size,
                color,
                format.border_radius);
        }
        draw_text2(img, (pos.0 + format.padding_left, pos.1 + format.padding_top), text, format);
        match position {
            Position::Inline => {
                return size.1;
            }
            Position::Relative(x, y) => {
                if y > 0 {
                    return y as u32 + size.1;
                }
            }
            _ => ()
        }
        0
    }

    pub fn draw(
        &self,
        img: &mut RgbaImage,
        ui: &UI,
        scene: &Arc<RwLock<Scene>>,
        box_pos: (u32, u32),
        box_size: (u32, u32),
        inline_pos: (u32, u32),
        indent: u32,
    ) -> u32 {
        let mut height = 0;
        match &self.elem_type {
            ElemType::Button(..) => {
                height += self.draw_text(img, self.text.clone(), box_pos, box_size, inline_pos, &self.format, indent, self.pos.clone());
            }
            ElemType::Category(cat) => {
                let cat_height = self.draw_text(img, self.text.clone(), box_pos, box_size, inline_pos, &self.format, indent, self.pos.clone());
                height += cat_height;
                    
                if !cat.collapsed {
                    for elem in &cat.elems {
                        if elem.visible {
                            let elem_height =
                                elem.draw(img, ui, scene, box_pos, box_size, (inline_pos.0, inline_pos.1 + height), indent + ui.settings().indent_padding)
                                    + ui.settings().margin;
                            height += elem_height;
                        }
                    }
                }
            }
            ElemType::Property(property) => {
                height += self.draw_text(img, self.text.clone(), box_pos, box_size, inline_pos, &self.format, indent, self.pos.clone());
                let format;
                let value;
                if let Some(edit) = ui.editing() {
                    if &self.reference == &edit.reference {
                        value = edit.value.clone() + "_";
                        format = &property.editing_format;
                    } else {
                        value = property.value.to_string();
                        format = &self.format;
                    }
                } else {
                    value = property.value.to_string();
                    format = &self.format;
                }
                let offset = value.len()as u32 * format.font_size() as u32 / 2 + format.padding_right + format.padding_left;
                self.draw_text(img, value, box_pos, box_size, inline_pos, format, indent, Position::Relative(-(offset as i32), 0));
            }
            ElemType::Stat(function) => {
                height += self.draw_text(img, self.text.clone(), box_pos, box_size, inline_pos, &self.format, indent, self.pos.clone());
                let value = function(&scene.read().unwrap());
                let offset = value.to_string().len() as i32 * 10 - self.format.padding_right as i32;
                self.draw_text(img, value, box_pos, box_size, inline_pos, &self.format, indent, Position::Relative(-(offset as i32), 0));
            }
            ElemType::Text => {
                let available_width = self.format.width - self.format.padding_left - self.format.padding_right;
                let mut current_width = 0;
                let mut lines = vec![];
    
                let mut txt_split = self.text.split(" ");
                let mut line = String::from("");
                while let Some(str) = txt_split.next() {
                    let word_width = self.format.font_size() as u32 / 2 * (str.len() + 1) as u32;
                    if current_width + word_width > available_width {
                        current_width = word_width;
                        lines.push(line.clone());
                        line = str.to_string() + " ";
                    } else {
                        line += str;
                        line += " ";
                        current_width += word_width;
                    }
                }
                lines.push(line);
                let mut pos = self.pos.clone();
                for line in lines {
                    height += self.draw_text(img, line, box_pos, box_size, inline_pos, &self.format, indent, pos.clone());
                    if let Position::Relative(x, y) = pos {
                        pos = Position::Relative(x, y + self.format.font_size() as i32 / 2 + ui.settings().margin as i32)
                    }
                }
            }
        }
        height
    }

    pub fn clicked(&mut self, click: (u32, u32), box_pos: (u32, u32), box_size: (u32, u32), indent: u32, inline_pos: (u32, u32), scene: &Arc<RwLock<Scene>>, ui: &mut UI) -> u32 {
        let pos = self.get_pos(&self.pos, box_pos, box_size, inline_pos, indent);
        let size = self.get_size(&self.text, &self.format);
        let mut height = size.1;
        println!("ref: {}, click: {}:{}, pos: {}:{}, size: {}:{}", self.reference, click.0, click.1, pos.0, pos.1, size.0, size.1);
        if click.0 > pos.0 && click.0 < pos.0 + size.0 && click.1 > pos.1 && click.1 < pos.1 + size.1 {
            println!("{} has been clicked", self.reference);
            match &mut self.elem_type {
                ElemType::Property(property) => {
                    println!("Property !");
                    if let Some(edit) = ui.editing() {
                        if &edit.reference != &self.reference {
                            ui.set_editing(Some(Editing {
                                reference: self.reference.clone(),
                                value: property.value.to_string(),
                            }));
                        }
                    } else {
                        println!("Set editing");
                        ui.set_editing(Some(Editing {
                            reference: self.reference.clone(),
                            value: property.value.to_string(),
                        }));
                    }
                },
                ElemType::Button(fn_click) => {
                    fn_click(&mut scene.write().unwrap(), ui);
                },
                ElemType::Category(cat) => {
                    cat.collapsed = !cat.collapsed;
                },
                _ => ()
            }
        } else if let ElemType::Category(cat) = &mut self.elem_type {
            for elem in &mut cat.elems {
                height += elem.clicked(click, box_pos, box_size, indent, (inline_pos.0, inline_pos.1 + height), scene, ui);
            }
        }
        height
        
    }
}

pub struct Category {
    pub elems: Vec<UIElement>,
    pub collapsed: bool,
}

pub type FnSubmit = Box<dyn Fn(Value, &mut Scene, &mut UI)>;
pub type FnValidate = Box<dyn Fn(&Value) -> Result<(), &'static str>>;

pub struct Property {
    pub value: Value,
    pub initial_value: Value,
    pub editing_format: TextFormat,
    pub fn_submit: FnSubmit,
    pub fn_validate: FnValidate,
}

impl Property {
    pub fn new(
        value: Value,
        fn_submit: FnSubmit,
        fn_validate: FnValidate,
        settings: &UISettings,
    ) -> Self {
        Self {
            editing_format: value.base_format(settings),
            initial_value: value.clone(),
            value,
            fn_submit,
            fn_validate,
        }
    }

    pub fn set_value_from_string(&mut self, val: String) -> Option<String> {
        match self.value {
            Value::Bool(_) => (),
            Value::Float(_) => {
                let val = val.parse::<f64>();
                if val.is_err() {
                    return Some("The value must be a proper float".to_string());
                } else {
                    self.value = Value::Float(val.unwrap());
                }
            }
            Value::Text(_) => {
                self.value = Value::Text(val);
            }
            Value::Unsigned(_) => {
                let val = val.parse::<u32>();
                if val.is_err() {
                    return Some("The value must be a proper unsigned integer".to_string());
                } else {
                    self.value = Value::Unsigned(val.unwrap());
                }
            }
        }
        None
    }
}
