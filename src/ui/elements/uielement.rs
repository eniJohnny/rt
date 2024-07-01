use std::{
    cell::RefCell,
    sync::{Arc, RwLock},
    thread::current,
};

use chrono::offset;
use image::{Rgba, RgbaImage};

use crate::{
    ui::{
        draw_utils::{draw_checkbox, draw_element_text}, style::{Formattable, Style, StyleBuilder}, ui::UI, uisettings::UISettings, utils::{get_pos, get_size, split_in_lines, Editing}
    },
    model::{
        materials::{color::Color, texture::Texture},
        scene::Scene,
    },
    SCREEN_WIDTH_U32,
};

use super::{utils::{ElemType, Property, Value}, HitBox};
pub struct UIElement {
    pub visible: bool,
    pub elem_type: ElemType,
    pub text: String,
    pub style: Style,
    pub size: (u32, u32),
    pub id: String,
    pub reference: String,
    pub value: Option<String>,
    pub hitbox: Option<HitBox>,
}

impl UIElement {
    pub fn new(name: &str, id: &str, elem: ElemType, settings: &UISettings) -> Self {
        UIElement {
            visible: true,
            style: elem.base_style(settings),
            elem_type: elem,
            text: String::from(name),
            size: (0, 0),
            reference: id.to_string(),
            id: id.to_string(),
            value: None,
            hitbox: None,
        }
    }

    pub fn add_element(&mut self, elem: UIElement) {
        if let ElemType::Category(cat) = &mut self.elem_type {
            cat.elems.push(elem);
        } else if let ElemType::Row(elems) = &mut self.elem_type {
            elems.push(elem);
        }
    }

    pub fn set_style(&mut self, format: Style) {
        self.style = format;
    }

    pub fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn refresh_format(&mut self, settings: &UISettings) {
        self.style = self.elem_type.base_style(settings);
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

    pub fn reset_properties(&mut self, scene: &Arc<RwLock<Scene>>) {
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

    pub fn submit_properties(&self, scene: &Arc<RwLock<Scene>>, ui: &mut UI) {
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

    pub fn process(&mut self, ui: &UI, scene: &Arc<RwLock<Scene>>, max_height: u32) -> Vec<HitBox> {
        let mut vec = vec![];
        if max_height == 0 {
            return vec;
        }
        if let Some(parent_hitbox) = &self.hitbox {
            match &mut self.elem_type {
                ElemType::Row(elems) => {
                    let available_width =
                        parent_hitbox.size.0 / elems.len() as u32 - ui.uisettings().margin;
                    let mut offset_x = 0;
                    for elem in elems {
                        let size = get_size(&elem.text, &elem.style, (available_width, max_height));
                        let center = (available_width / 2, parent_hitbox.size.1);
                        let pos = (
                            parent_hitbox.pos.0 + offset_x + center.0 - size.0 / 2,
                            parent_hitbox.pos.1 + center.1 - size.1 / 2,
                        );
                        let hitbox = HitBox {
                            pos,
                            size,
                            reference: elem.reference.clone(),
                            disabled: matches!(elem.elem_type, ElemType::Row(_)),
                        };
                        elem.hitbox = Some(hitbox.clone());
                        let hitbox_list = elem.process(ui, scene, max_height);
                        offset_x += available_width + ui.uisettings().margin;
                        vec.push(hitbox);
                        for hitbox in hitbox_list {
                            vec.push(hitbox);
                        }
                    }
                }
                ElemType::Category(cat) => {
                    if !cat.collapsed {
                        let mut offset_y = parent_hitbox.size.1;
                        if !parent_hitbox.disabled {
                            offset_y += ui.uisettings().margin;
                        }
                        for i in 0..cat.elems.len() {
                            let mut elem = cat.elems.remove(i);
                            if elem.visible {
                                let hitbox = HitBox {
                                    pos: get_pos(
                                        (parent_hitbox.pos.0, parent_hitbox.pos.1 + offset_y),
                                        (0, 0),
                                        0,
                                    ),
                                    size: get_size(&elem.text, &elem.style, parent_hitbox.size),
                                    reference: elem.reference.clone(),
                                    disabled: matches!(elem.elem_type, ElemType::Row(_)),
                                };
                                elem.hitbox = Some(hitbox.clone());
                                let hitbox_list = elem.process(ui, scene, max_height - offset_y);
                                let needed_height =
                                    hitbox.pos.1 + hitbox.size.1 + ui.uisettings().margin - parent_hitbox.pos.1;
                                if !hitbox.disabled && needed_height > offset_y {
                                    offset_y = needed_height;
                                }
                                vec.push(hitbox);
                                for hitbox in hitbox_list {
                                    let needed_height =
                                        hitbox.pos.1 + hitbox.size.1 + ui.uisettings().margin - parent_hitbox.pos.1;
                                    if !hitbox.disabled && needed_height > offset_y {
                                        offset_y = needed_height;
                                    }
                                    vec.push(hitbox)
                                }
                                if offset_y > max_height {
                                    return vec;
                                }
                            }
                            cat.elems.insert(i, elem);
                        }
                    }
                }
                ElemType::Property(property) => {
                    self.value = None;
                    if !matches!(property.value, Value::Bool(_)) {
                        if let Some(edit) = ui.editing() {
                            if &self.reference == &edit.reference {
                                self.value = Some(edit.value.clone() + "_");
                            }
                        }
                    }
                }
                ElemType::Stat(function) => {
                    self.value = Some(function(&scene.read().unwrap(), ui));
                }
                _ => {}
            }
        }

        vec
    }

    pub fn draw(&self, img: &mut RgbaImage, ui: &UI, scene: &Arc<RwLock<Scene>>) {
        if let Some(hitbox) = &self.hitbox {
            match &self.elem_type {
                ElemType::Row(elems) => {
                    for elem in elems {
                        elem.draw(img, ui, scene);
                    }
                }
                ElemType::Button(..) => {
                    draw_element_text(img, self.text.clone(), hitbox.pos, hitbox.size, &self.style);
                }
                ElemType::Category(cat) => {
                    draw_element_text(img, self.text.clone(), hitbox.pos, hitbox.size, &self.style);

                    if !cat.collapsed {
                        for elem in &cat.elems {
                            elem.draw(img, ui, scene);
                        }
                    }
                }
                ElemType::Property(property) => {
                    draw_element_text(img, self.text.clone(), hitbox.pos, hitbox.size, &self.style);
                    if let Value::Bool(value) = property.value {
                        draw_checkbox(img, hitbox.pos, hitbox.size, value, &self.style);
                    } else {
                        let format;
                        let value = match &self.value {
                            Some(value) => {
                                format = &property.editing_format;
                                value.clone()
                            }
                            None => {
                                format = &self.style;
                                property.value.to_string()
                            }
                        };
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
                ElemType::Stat(_) => {
                    draw_element_text(img, self.text.clone(), hitbox.pos, hitbox.size, &self.style);
                    if let Some(value) = &self.value {
                        let value_width = value.len() as u32 * self.style.font_size as u32 / 2
                            + self.style.padding_left
                            + self.style.padding_right;
                        let offset = hitbox.size.0 - value_width;
                        draw_element_text(
                            img,
                            value.clone(),
                            (hitbox.pos.0 + offset, hitbox.pos.1),
                            (value_width, hitbox.size.1),
                            &self.style,
                        );
                    }
                }
                ElemType::Text => {
                    let available_width =
                        self.style.width - self.style.padding_left - self.style.padding_right;
                    let lines = split_in_lines(self.text.clone(), available_width, &self.style);
                    let mut height = 0;
                    for line in lines {
                        let size = get_size(&line, &self.style, (available_width, hitbox.size.1));
                        draw_element_text(
                            img,
                            line,
                            (hitbox.pos.0, hitbox.pos.1 + height),
                            size,
                            &self.style,
                        );
                        height += size.1;
                    }
                }
            }
        }
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