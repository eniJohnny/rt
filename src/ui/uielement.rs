use std::sync::{Arc, RwLock};
use image::RgbaImage;
use crate::model::scene::Scene;
use super::{ui::UI, uisettings::UISettings, utils::{draw_utils::{draw_checkbox, draw_element_text, get_size, split_in_lines}, misc::{ElemType, FnAny, Property, Value}, style::{Formattable, Style}, ui_utils::{get_pos, Editing}, HitBox}};

pub struct UIElement {
    pub elem_type: ElemType,
    pub text: String,
    pub style: Style,
    pub size: (u32, u32),
    pub id: String,
    pub reference: String,
    pub value: Option<String>,
    pub hitbox: Option<HitBox>,
    pub on_click: Option<FnAny>
}

impl UIElement {
    pub fn new(name: &str, id: &str, elem: ElemType, settings: &UISettings) -> Self {
        UIElement {
            style: elem.base_style(settings),
            elem_type: elem,
            text: String::from(name),
            size: (0, 0),
            reference: id.to_string(),
            id: id.to_string(),
            value: None,
            hitbox: None,
            on_click: None
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

    pub fn get_properties_reference(&self, vec: &mut Vec<String>)  {
        if let ElemType::Property(_) = &self.elem_type {
            vec.push(self.reference.clone());
        } else if let ElemType::Row(elems) = &self.elem_type {
            for elem in elems {
                elem.get_properties_reference(vec);
            }
        } else if let ElemType::Category(cat) = &self.elem_type {
            for elem in &cat.elems {
                elem.get_properties_reference(vec);
            }
        }
    }

    pub fn get_elem_type(&self) -> &ElemType {
        &self.elem_type
    }

    pub fn get_elem_type_mut(&mut self) -> &mut ElemType {
        &mut self.elem_type
    }

    pub fn get_property(&self, reference: &str) -> Option<&Property> {
        match &self.elem_type {
            ElemType::Property(property) => {
                if &self.reference == reference {
                    return Some(property);
                }
            }
            ElemType::Category(cat) => {
                for elem in &cat.elems {
                    if let Some(property) = elem.get_property(reference) {
                        return Some(property);
                    }
                }
            }
            ElemType::Row(elems) => {
                for elem in elems {
                    if let Some(property) = elem.get_property(reference) {
                        return Some(property);
                    }
                }
            }
            _ => (),
        }
        None
    }

    pub fn get_property_mut(&mut self, reference: &str) -> Option<&mut Property> {
        match &mut self.elem_type {
            ElemType::Property(property) => {
                if &self.reference == reference {
                    return Some(property);
                }
            }
            ElemType::Category(cat) => {
                for elem in &mut cat.elems {
                    if let Some(property) = elem.get_property_mut(reference) {
                        return Some(property);
                    }
                }
            }
            ElemType::Row(elems) => {
                for elem in elems {
                    if let Some(property) = elem.get_property_mut(reference) {
                        return Some(property);
                    }
                }
            }
            _ => (),
        }
        None
    }

    pub fn get_element(&self, reference: &str) -> Option<&UIElement> {
        if &self.reference == reference {
            return Some(self);
        }
        match &self.elem_type {
            ElemType::Category(cat) => {
                for elem in &cat.elems {
                    let result = elem.get_element(reference);
                    if result.is_some() {
                        return result;
                    }
                }
            }
            ElemType::Row(elems) => {
                for elem in elems {
                    let result = elem.get_element(reference);
                    if result.is_some() {
                        return result;
                    }
                }
            }
            _ => {}
        }
        None
    }

    pub fn get_element_mut(&mut self, reference: &str) -> Option<&mut UIElement> {
        if &self.reference == reference {
            return Some(self);
        }
        match &mut self.elem_type {
            ElemType::Category(cat) => {
                for elem in &mut cat.elems {
                    let result = elem.get_element_mut(reference);
                    if result.is_some() {
                        return result;
                    }
                }
            }
            ElemType::Row(elems) => {
                for elem in elems {
                    let result = elem.get_element_mut(reference);
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

    pub fn validate_properties(&self, ui: &UI) -> Result<(), String> {
        if let ElemType::Category(cat) = &self.elem_type {
            for elem in &cat.elems {
                elem.validate_properties(ui)?;
            }
        } else if let ElemType::Property(prop) = &self.elem_type {
            (prop.fn_validate)(&prop.value, self, ui)?;
        } else if let ElemType::Row(elems) = &self.elem_type {
            for elem in elems {
                elem.validate_properties(ui)?;
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
            (prop.fn_submit)(Some(self), prop.value.clone(), scene, ui);
        } else if let ElemType::Row(elems) = &self.elem_type {
            for elem in elems {
                elem.submit_properties(scene, ui);
            }
        }
    }

    pub fn generate_hitbox(&mut self, ui: &UI, scene: &Arc<RwLock<Scene>>, max_height: u32) -> Vec<HitBox> {
        let mut vec = vec![];
        if max_height == 0 {
            return vec;
        }
        let mut indent: u32 = 10;
        if let ElemType::Row(_vec) = &self.elem_type {
            indent = 0;
        }
        if let Some(parent_hitbox) = &self.hitbox {
            match &mut self.elem_type {
                ElemType::Row(elems) => {
                    let available_width =
                        (parent_hitbox.size.0 - ui.uisettings().margin * (elems.len() - 1) as u32 - indent) / elems.len() as u32;
                    let mut offset_x = indent;
                    for elem in elems {
                        if elem.style.visible {
                            let size = get_size(&elem.text, &elem.style, (available_width, max_height));
                            let center = (available_width / 2, parent_hitbox.size.1 / 2);
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
                            let hitbox_list = elem.generate_hitbox(ui, scene, max_height);
                            offset_x += available_width + ui.uisettings().margin;
                            vec.push(hitbox);
                            for hitbox in hitbox_list {
                                vec.push(hitbox);
                            }
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
                            if elem.style.visible {
                                let size = get_size(&elem.text, &elem.style, parent_hitbox.size);
                                let mut hitbox = HitBox {
                                    pos: get_pos(
                                        (parent_hitbox.pos.0 + indent, parent_hitbox.pos.1 + offset_y),
                                        (0, 0),
                                        0,
                                    ),
                                    size: (size.0 - indent, size.1),
                                    reference: elem.reference.clone(),
                                    disabled: matches!(elem.elem_type, ElemType::Row(_)),
                                };
                                if parent_hitbox.disabled || offset_y + size.1 > max_height {
                                    hitbox.disabled = true;
                                    offset_y = max_height;
                                } else {
                                    elem.hitbox = Some(hitbox.clone());
                                    let hitbox_list = elem.generate_hitbox(ui, scene, max_height - offset_y);
                                    let mut needed_height =
                                        hitbox.pos.1 + hitbox.size.1 - parent_hitbox.pos.1;
                                    if !hitbox.disabled {
                                        needed_height += ui.uisettings().margin;
                                        if needed_height > offset_y {
                                            offset_y = needed_height;
                                        }
                                    }
                                    if !hitbox.disabled {
                                        vec.push(hitbox);
                                    }
                                    for hitbox in hitbox_list {
                                        let needed_height =
                                            hitbox.pos.1 + hitbox.size.1 + ui.uisettings().margin - parent_hitbox.pos.1;
                                        if needed_height > offset_y {
                                            offset_y = needed_height;
                                        }
                                        if !hitbox.disabled {
                                            vec.push(hitbox);
                                        }
                                    }
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

    pub fn translate_hitboxes(&mut self, absolute_pos: (u32, u32)) {
        if let Some(hitbox) = &mut self.hitbox {
            hitbox.pos.0 += absolute_pos.0;
            hitbox.pos.1 += absolute_pos.1;
        }
        if let ElemType::Category(cat) = &mut self.elem_type {
            for elem in &mut cat.elems {
                elem.translate_hitboxes(absolute_pos);
            }
        } else if let ElemType::Row(elems) = &mut self.elem_type {
            for elem in elems {
                elem.translate_hitboxes(absolute_pos);
            }
        }
    }

    pub fn draw(&self, img: &mut RgbaImage, ui: &UI, scene: &Arc<RwLock<Scene>>) {
        if let Some(hitbox) = &self.hitbox {
            match &self.elem_type {
                ElemType::Row(elems) => {
                    for elem in elems {
                        if elem.style.visible {
                            elem.draw(img, ui, scene);
                        }
                    }
                }
                ElemType::Button(..) => {
                    draw_element_text(img, self.text.clone(), hitbox.pos, hitbox.size, &self.style);
                }
                ElemType::Category(cat) => {
                    draw_element_text(img, self.text.clone(), hitbox.pos, hitbox.size, &self.style);

                    if !cat.collapsed {
                        for elem in &cat.elems {
                            if elem.style.visible {
                                elem.draw(img, ui, scene);
                            }
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
        if !self.style.disabled {
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
                ElemType::Category(cat) => {
                    cat.collapsed = !cat.collapsed;
                }
                ElemType::Button(fn_click) => {
                    let click = fn_click.take();
                    if let Some(click) = click {
                        click(Some(self), scene, ui);
                        self.elem_type = ElemType::Button(Some(click));
                    }
                }
                _ => (),
            }
        }
        if let Some(click) = self.on_click.take() {
            click(Some(self), scene, ui);
            self.on_click = Some(click);
        }
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.style.disabled = disabled;
        if let ElemType::Category(cat) = &mut self.elem_type {
            for elem in &mut cat.elems {
                elem.set_disabled(disabled);
            }
        } else if let ElemType::Row(elems) = &mut self.elem_type {
            for elem in elems {
                elem.set_disabled(disabled);
            }
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

    pub fn collapsed() -> Self {
        Self {
            elems: vec![],
            collapsed: true,
        }
    }
}