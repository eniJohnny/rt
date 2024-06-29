use std::{
    cell::RefCell,
    sync::{Arc, RwLock},
    thread::current,
};

use chrono::offset;
use image::{Rgba, RgbaImage};

use crate::{
    display::utils::{draw_text2, draw_text_background2},
    gui::{
        draw::draw_background,
        textformat::{Formattable, Style, StyleBuilder},
        uisettings::UISettings,
    },
    model::{
        materials::{color::Color, texture::Texture},
        scene::Scene,
    },
    SCREEN_WIDTH_U32,
};

use super::{
    ui::{Editing, UI},
    uibox::UIBox,
    utils::{draw_element_text, get_pos, get_size, split_in_lines},
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
    fn base_style(&self, settings: &UISettings) -> Style {
        Style::editing(settings)
    }
}

pub enum ElemType {
    Text,
    Stat(Box<dyn Fn(&Scene) -> String>),
    Property(Property),
    Category(Category),
    Button(Box<dyn Fn(&mut Scene, &mut UI)>),
    Row(Vec<UIElement>),
}

impl Formattable for ElemType {
    fn base_style(&self, settings: &UISettings) -> Style {
        match self {
            ElemType::Row(..) => Style::row(settings),
            ElemType::Button(..) => Style::button(settings),
            ElemType::Category(..) => Style::category(settings),
            ElemType::Property(..) => Style::field_format(settings),
            ElemType::Stat(..) => StyleBuilder::default(settings).bg_color(None).build(),
            ElemType::Text => {
                let mut format = Style::field_format(settings);
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
    pub format: Style,
    pub size: (u32, u32),
    pub id: String,
    pub reference: String,
}

impl UIElement {
    pub fn new(name: &str, id: &str, elem: ElemType, settings: &UISettings) -> Self {
        UIElement {
            visible: true,
            format: elem.base_style(settings),
            elem_type: elem,
            text: String::from(name),
            size: (0, 0),
            reference: id.to_string(),
            id: id.to_string(),
        }
    }
    pub fn height(&self, max_size: (u32, u32), settings: &UISettings) -> u32 {
        if !self.visible {
            return 0;
        }
        let mut height = get_size(&self.text, &self.format, max_size).1;
        if let ElemType::Category(cat) = &self.elem_type {
            if !cat.collapsed {
                for elem in &cat.elems {
                    height += elem.height(max_size, settings) + settings.margin;
                }
            }
        }
        height
    }

    pub fn set_format(&mut self, format: Style) {
        self.format = format;
    }

    pub fn refresh_format(&mut self, settings: &UISettings) {
        self.format = self.elem_type.base_style(settings);
        if let ElemType::Category(cat) = &mut self.elem_type {
            for elem in &mut cat.elems {
                elem.refresh_format(settings);
            }
        } else if let ElemType::Row(elems) = &mut self.elem_type {
            for elem in elems {
                elem.refresh_format(settings);
            }
        }
    }

    pub fn set_reference(&mut self, parent_ref: String) {
        self.reference = parent_ref + "." + &self.id;

        if let ElemType::Category(cat) = &mut self.elem_type {
            for elem in &mut cat.elems {
                elem.set_reference(self.reference.clone());
            }
        } else if let ElemType::Row(elems) = &mut self.elem_type {
            for elem in elems {
                elem.set_reference(self.reference.clone());
            }
        }
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
            ElemType::Row(elems) => {
                for elem in elems {
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
        }
        match &mut self.elem_type {
            ElemType::Category(cat) => {
                for elem in &mut cat.elems {
                    let result = elem.get_element_by_reference_mut(reference);
                    if result.is_some() {
                        return result;
                    }
                }
            }
            ElemType::Row(elems) => {
                for elem in elems {
                    let result = elem.get_element_by_reference_mut(reference);
                    if result.is_some() {
                        return result;
                    }
                }
            }
            _ => {}
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
        } else if let ElemType::Row(elems) = &mut self.elem_type {
            for elem in elems {
                elem.reset_properties(scene);
            }
        }
    }

    pub fn validate_properties(&self) -> Result<(), String> {
        if let ElemType::Category(cat) = &self.elem_type {
            for elem in &cat.elems {
                elem.validate_properties()?;
            }
        } else if let ElemType::Property(prop) = &self.elem_type {
            (prop.fn_validate)(&prop.value)?;
        } else if let ElemType::Row(elems) = &self.elem_type {
            for elem in elems {
                elem.validate_properties()?;
            }
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
        } else if let ElemType::Row(elems) = &self.elem_type {
            for elem in elems {
                elem.submit_properties(scene, ui);
            }
        }
    }

    pub fn draw(
        &self,
        img: &mut RgbaImage,
        ui: &UI,
        scene: &Arc<RwLock<Scene>>,
        hitbox: &HitBox,
        max_height: u32,
    ) -> Vec<HitBox> {
        let mut vec = vec![];
        if max_height == 0 {
            return vec;
        }
        match &self.elem_type {
            ElemType::Row(elems) => {
                let available_width = hitbox.size.0 / elems.len() as u32 - ui.uisettings().margin;
                let mut offset_x = 0;
                for elem in elems {
                    let size = get_size(&elem.text, &elem.format, hitbox.size);
                    let center = (available_width / 2, hitbox.size.1);
                    let pos = (
                        hitbox.pos.0 + offset_x + center.0 - size.0 / 2,
                        hitbox.pos.1 + center.1 - size.1 / 2,
                    );
                    let hitbox = HitBox {
                        pos,
                        size,
                        reference: elem.reference.clone(),
                        disabled: matches!(elem.elem_type, ElemType::Row(_)),
                    };
                    let hitbox_list = elem.draw(img, ui, scene, &hitbox, max_height);
                    offset_x += available_width + ui.uisettings().margin;
                    vec.push(hitbox);
                    for hitbox in hitbox_list {
                        vec.push(hitbox)
                    }
                }
            }
            ElemType::Button(..) => {
                draw_element_text(
                    img,
                    self.text.clone(),
                    hitbox.pos,
                    hitbox.size,
                    &self.format,
                );
            }
            ElemType::Category(cat) => {
                draw_element_text(
                    img,
                    self.text.clone(),
                    hitbox.pos,
                    hitbox.size,
                    &self.format,
                );

                if !cat.collapsed {
                    let mut offset_y = hitbox.size.1 + ui.uisettings().margin;
                    for elem in &cat.elems {
                        if elem.visible {
                            let hitbox = HitBox {
                                pos: get_pos((hitbox.pos.0, hitbox.pos.1 + offset_y), (0, 0), 0),
                                size: get_size(&elem.text, &elem.format, hitbox.size),
                                reference: elem.reference.clone(),
                                disabled: matches!(elem.elem_type, ElemType::Row(_)),
                            };
                            let hitbox_list =
                                elem.draw(img, ui, scene, &hitbox, max_height - offset_y);
                            offset_y += hitbox.size.1 + ui.uisettings().margin;
                            vec.push(hitbox);
                            for hitbox in hitbox_list {
                                offset_y += hitbox.size.1 + ui.uisettings().margin;
                                vec.push(hitbox)
                            }
                            if offset_y > max_height {
                                return vec;
                            }
                        }
                    }
                }
            }
            ElemType::Property(property) => {
                draw_element_text(
                    img,
                    self.text.clone(),
                    hitbox.pos,
                    hitbox.size,
                    &self.format,
                );
                let value;
                if let Value::Bool(value) = property.value {
                    let checkbox_size = (18, 18);
                    let height = hitbox.size.1;
                    let checkbox_pos = (
                        hitbox.pos.0 + hitbox.size.0 - self.format.padding_right - checkbox_size.0,
                        hitbox.pos.1 + (height - checkbox_size.1) / 2,
                    );

                    draw_background(
                        img,
                        checkbox_pos,
                        checkbox_size,
                        Rgba([40, 10, 90, 255]),
                        self.format.border_radius,
                    );
                    draw_background(
                        img,
                        (checkbox_pos.0 + 2, checkbox_pos.1 + 2),
                        (checkbox_size.0 - 4, checkbox_size.1 - 4),
                        Rgba([255, 255, 255, 255]),
                        self.format.border_radius,
                    );
                    if value {
                        draw_background(
                            img,
                            (checkbox_pos.0 + 4, checkbox_pos.1 + 4),
                            (checkbox_size.0 - 8, checkbox_size.1 - 8),
                            Rgba([40, 10, 90, 255]),
                            self.format.border_radius,
                        );
                    }
                } else {
                    let format;
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
                    let value_width = value.len() as u32 * format.font_size as u32 / 2
                        + format.padding_left
                        + format.padding_right;
                    let offset = hitbox.size.0 - value_width;
                    draw_element_text(
                        img,
                        value,
                        (hitbox.pos.0 + offset, hitbox.pos.1),
                        (value_width, hitbox.size.1),
                        format,
                    );
                }
            }
            ElemType::Stat(function) => {
                draw_element_text(
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
                draw_element_text(
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
                    let size = get_size(&line, &self.format, (available_width, max_height));
                    draw_element_text(
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
                if let Value::Bool(value) = property.value {
                    property.value = Value::Bool(!value);
                } else if let Some(edit) = ui.editing() {
                    if &edit.reference != &self.reference {
                        ui.set_editing(Some(Editing {
                            reference: self.reference.clone(),
                            value: property.value.to_string(),
                        }));
                    }
                } else {
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

impl Category {
    pub fn default() -> Self {
        Self {
            elems: vec![],
            collapsed: false,
        }
    }
}

pub type FnSubmit = Box<dyn Fn(Value, &mut Scene, &mut UI)>;
pub type FnApply = Box<dyn Fn(&mut Scene, &mut UI)>;
pub type FnValidate = Box<dyn Fn(&Value) -> Result<(), &'static str>>;

pub struct Property {
    pub value: Value,
    pub initial_value: Value,
    pub editing_format: Style,
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
            editing_format: value.base_style(settings),
            initial_value: value.clone(),
            value,
            fn_submit,
            fn_validate,
        }
    }

    pub fn get_value_from_string(&self, val: String) -> Result<Value, String> {
        match self.value {
            Value::Bool(_) => return Err("Bool value edited ?".to_string()),
            Value::Float(_) => {
                let val = val.parse::<f64>();
                if val.is_err() {
                    return Err("The value must be a proper float".to_string());
                } else {
                    return Ok(Value::Float(val.unwrap()));
                }
            }
            Value::Text(_) => {
                return Ok(Value::Text(val));
            }
            Value::Unsigned(_) => {
                let val = val.parse::<u32>();
                if val.is_err() {
                    return Err("The value must be a proper unsigned integer".to_string());
                } else {
                    return Ok(Value::Unsigned(val.unwrap()));
                }
            }
        }
    }
}
