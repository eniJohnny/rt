use std::{
    cell::RefCell,
    sync::{Arc, RwLock},
    thread::current,
};

use image::{Rgba, RgbaImage};

use crate::{
    display::utils::{draw_text2, draw_text_background2},
    gui::{
        draw::draw_background,
        textformat::{FormatBuilder, Formattable, TextFormat},
        uisettings::UISettings,
    },
    model::{materials::texture::Texture, scene::Scene},
    SCREEN_WIDTH_U32,
};

use super::{
    ui::{Editing, UI},
    uibox::UIBox,
    utils::{get_pos, get_size, split_in_lines},
    Displayable, HitBox,
};

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
            }
        }
    }
}

pub struct UIElement {
    pub visible: bool,
    pub elem_type: ElemType,
    pub text: String,
    pub format: TextFormat,
    pub size: (u32, u32),
    pub reference: String,
}

impl UIElement {
    pub fn new(name: &str, reference: String, elem: ElemType, settings: &UISettings) -> Self {
        UIElement {
            visible: true,
            format: elem.base_format(settings),
            elem_type: elem,
            text: String::from(name),
            size: (0, 0),
            reference: reference + "." + name,
        }
    }
    pub fn height(&self, settings: &UISettings) -> u32 {
        if !self.visible {
            return 0;
        }
        let mut height = get_size(&self.text, &self.format).1;
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

    pub fn get_element_by_reference_mut(&mut self, reference: &String) -> Option<&mut UIElement> {
        if &self.reference == reference {
            return Some(self);
        } else if let ElemType::Category(cat) = &mut self.elem_type {
            for elem in &mut cat.elems {
                let result = elem.get_element_by_reference_mut(reference);
                if result.is_some() {
                    return result;
                }
            }
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

    fn draw_text(
        &self,
        img: &mut RgbaImage,
        text: String,
        pos: (u32, u32),
        size: (u32, u32),
        format: &TextFormat,
    ) {
        if let Some(color) = format.bg_color {
            draw_background(img, pos, size, color, format.border_radius);
        }
        draw_text2(
            img,
            (pos.0 + format.padding_left, pos.1 + format.padding_top),
            text,
            format,
        );
    }

    pub fn draw(
        &self,
        img: &mut RgbaImage,
        ui: &UI,
        scene: &Arc<RwLock<Scene>>,
        hitbox: &HitBox,
    ) -> Vec<HitBox> {
        let mut vec = vec![];
        match &self.elem_type {
            ElemType::Button(..) => {
                self.draw_text(
                    img,
                    self.text.clone(),
                    hitbox.pos,
                    hitbox.size,
                    &self.format,
                );
            }
            ElemType::Category(cat) => {
                self.draw_text(
                    img,
                    self.text.clone(),
                    hitbox.pos,
                    hitbox.size,
                    &self.format,
                );

                if !cat.collapsed {
                    for elem in &cat.elems {
                        let mut height = hitbox.size.1 + ui.settings().margin;
                        if elem.visible {
                            let hitbox = HitBox {
                                pos: get_pos((hitbox.pos.0, hitbox.pos.1 + height), (0, 0), 0),
                                size: get_size(&elem.text, &elem.format),
                                reference: elem.reference.clone(),
                            };
                            let hitbox_list = elem.draw(img, ui, scene, &hitbox);
                            height += hitbox.size.1 + ui.settings().margin;
                            vec.push(hitbox);
                            for hitbox in hitbox_list {
                                height += hitbox.size.1 + ui.settings().margin;
                                vec.push(hitbox)
                            }
                        }
                    }
                }
            }
            ElemType::Property(property) => {
                self.draw_text(
                    img,
                    self.text.clone(),
                    hitbox.pos,
                    hitbox.size,
                    &self.format,
                );
                let format;
                let value;
                if let Some(edit) = ui.editing() {
                    println!("editing");
                    if &self.reference == &edit.reference {
                        value = edit.value.clone() + "_";
                        println!("drawing editing format");
                        format = &property.editing_format;
                    } else {
                        value = property.value.to_string();
                        format = &self.format;
                    }
                } else {
                    value = property.value.to_string();
                    format = &self.format;
                }
                let value_width = value.len() as u32 * format.font_size as u32 / 2
                    + format.padding_left
                    + format.padding_right;
                let offset = hitbox.size.0 - value_width;
                self.draw_text(
                    img,
                    value,
                    (hitbox.pos.0 + offset, hitbox.pos.1),
                    (value_width, hitbox.size.1),
                    format,
                );
            }
            ElemType::Stat(function) => {
                self.draw_text(
                    img,
                    self.text.clone(),
                    hitbox.pos,
                    hitbox.size,
                    &self.format,
                );
                let value = function(&scene.read().unwrap());
                let value_width = value.len() as u32 * self.format.font_size as u32 / 2
                    + self.format.padding_left
                    + self.format.padding_right;
                let offset = hitbox.size.0 - value_width;
                self.draw_text(
                    img,
                    value,
                    (hitbox.pos.0 + offset, hitbox.pos.1),
                    (value_width, hitbox.size.1),
                    &self.format,
                );
            }
            ElemType::Text => {
                let available_width =
                    self.format.width - self.format.padding_left - self.format.padding_right;
                let mut lines = split_in_lines(self.text.clone(), available_width, &self.format);
                let mut height = 0;
                for line in lines {
                    let size = get_size(&line, &self.format);
                    self.draw_text(
                        img,
                        line,
                        (hitbox.pos.0, hitbox.pos.1 + height),
                        size,
                        &self.format,
                    );
                    height += size.1;
                }
            }
        }
        vec
    }

    pub fn clicked(&mut self, scene: &Arc<RwLock<Scene>>, ui: &mut UI) {
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
            }
            ElemType::Button(fn_click) => {
                fn_click(&mut scene.write().unwrap(), ui);
            }
            ElemType::Category(cat) => {
                cat.collapsed = !cat.collapsed;
            }
            _ => (),
        }
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
