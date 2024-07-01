use std::{borrow::Borrow, sync::mpsc::{Receiver, Sender}};

use image::{ImageBuffer, Rgba, RgbaImage};

use crate::{
    ui::style::{Formattable, Style}, model::scene::Scene, SCREEN_HEIGHT_U32, SCREEN_WIDTH_U32
};

use super::{
    elements::{uielement::UIElement, utils::ElemType}, ui::UI
};


#[derive(Clone)]
pub struct Editing {
    pub reference: String,
    pub value: String,
}

pub struct Statistics {
    pub fps: u32,
}

pub struct UIContext {
    pub ui_img: RgbaImage,
    pub scene_img: RgbaImage,
    pub receiver: Receiver<(ImageBuffer<Rgba<u8>, Vec<u8>>, bool)>,
    pub transmitter: Sender<bool>,
    pub draw_time_avg: f64,
    pub draw_time_samples: u32,
    pub last_input_time: u32,
    pub final_img: bool,
    pub image_asked: bool
}

impl UIContext {
    pub fn new(
        receiver: Receiver<(ImageBuffer<Rgba<u8>, Vec<u8>>, bool)>,
        transmitter: Sender<bool>,
    ) -> Self {
        Self {
            ui_img: RgbaImage::new(SCREEN_WIDTH_U32, SCREEN_HEIGHT_U32),
            scene_img: RgbaImage::new(SCREEN_WIDTH_U32, SCREEN_HEIGHT_U32),
            receiver,
            transmitter,
            draw_time_avg: 0.,
            draw_time_samples: 0,
            last_input_time: 0,
            final_img: false,
            image_asked: false
        }
    }
}

pub fn get_pos(parent_pos: (u32, u32), offset_pos: (u32, u32), indent: u32) -> (u32, u32) {
    (
        parent_pos.0 + offset_pos.0 + indent,
        parent_pos.1 + offset_pos.1,
    )
}

pub fn get_size(text: &String, style: &Style, max_size: (u32, u32)) -> (u32, u32) {
    let mut height = style.font_size() as u32 + style.padding_bot + style.padding_top;
    let mut width = (style.font_size() / 2. * text.len() as f32) as u32
        + style.padding_left
        + style.padding_top;

    let mut wanted_width = style.width.max(max_size.0 * style.fill_width as u32);
    let mut wanted_height = style.height;

    if wanted_height > height {
        height = wanted_height;
    }
    if wanted_width > width {
        width = wanted_width;
    }
    if width > max_size.0 {
        let lines = split_in_lines(text.clone(), style.width, style);
        height += (style.font_size() as u32 + style.padding_bot) * lines.len() as u32;
    }
    (width.min(max_size.0), height.min(max_size.1))
}

pub fn split_in_lines(str: String, available_width: u32, format: &Style) -> Vec<String> {
    let mut current_width = 0;
    let mut lines = vec![];
    let mut txt_split = str.split(" ");
    let mut line = String::from("");
    while let Some(str) = txt_split.next() {
        let word_width = format.font_size() as u32 / 2 * (str.len() + 1) as u32;
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
    lines
}

pub fn get_parent_ref(reference: String) -> String {
    let count = reference.clone().split(".").count();
    let copy = reference.clone();
    let mut split = copy.split(".");
    let mut parent_ref = "".to_string();
    for i in 0..count - 1 {
        if i != 0 {
            parent_ref += ".";
        }
        parent_ref += split.next().unwrap();
    }
    return parent_ref;
}

pub fn take_element(ui: &mut UI, reference: String) -> Option<(UIElement, String, usize)> {
    let parent_ref = get_parent_ref(reference.clone());
    if !parent_ref.contains(".") {
        let uibox = ui.get_box_mut(parent_ref.clone());
        let mut index = None;
        for i in 0..uibox.elems.len() {
            if uibox.elems[i].reference == reference {
                index = Some(i);
                break;
            }
        }
        if let Some(index) = index {
            let elem = uibox.elems.remove(index);
            return Some((elem, parent_ref, index));
        }
    } else {
        let elem = ui.get_element_by_reference_mut(parent_ref.clone()).unwrap();
        if let ElemType::Category(cat) = &mut elem.elem_type {
            let mut index = 0;
            for i in 0..cat.elems.len() {
                if cat.elems[i].reference == reference {
                    index = i;
                    break;
                }
            }
            let elem = cat.elems.remove(index);
            return Some((elem, parent_ref, index));
        } else if let ElemType::Row(elems) = &mut elem.elem_type {
            let mut index = 0;
            for i in 0..elems.len() {
                if elems[i].reference == reference {
                    index = i;
                    break;
                }
            }
            let elem = elems.remove(index);
            return Some((elem, parent_ref, index));
        }
    }
    None
}

pub fn give_back_element(ui: &mut UI, elem: UIElement, parent_ref: String, index: usize) {
    if !parent_ref.contains(".") {
        let uibox = ui.get_box_mut(parent_ref.clone());
        uibox.elems.insert(index, elem);
    } else {
        let parent = ui.get_element_by_reference_mut(parent_ref.clone()).unwrap();
        if let ElemType::Category(cat) = &mut parent.elem_type {
            cat.elems.insert(index, elem);
        } else if let ElemType::Row(elems) = &mut parent.elem_type {
            elems.insert(index, elem);
        }
    }
}