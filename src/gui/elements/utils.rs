use image::RgbaImage;

use crate::{display::utils::draw_text2, gui::{draw::draw_background, textformat::TextFormat}};

use super::{
    ui::UI,
    uielement::{ElemType, UIElement},
};

pub fn get_pos(box_pos: (u32, u32), inline_pos: (u32, u32), indent: u32) -> (u32, u32) {
    (box_pos.0 + inline_pos.0 + indent, box_pos.1 + inline_pos.1)
}

pub fn get_size(text: &String, format: &TextFormat) -> (u32, u32) {
    let mut height = format.font_size() as u32 + format.padding_bot + format.padding_top;
    let mut width = format.font_size() as u32 / 2 * text.len() as u32
        + format.padding_left
        + format.padding_top;
    if format.height > height {
        height = format.height;
    }
    if format.width == 0 {
        return (width, height);
    }
    if width > format.width {
        let lines = split_in_lines(text.clone(), format.width, format);
        height += (format.font_size() as u32 + format.padding_bot) * lines.len() as u32;
    }
    if format.width > width {
        width = format.width;
    }
    (width, height)
}

pub fn split_in_lines(str: String, available_width: u32, format: &TextFormat) -> Vec<String> {
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
        let mut index = 0;
        for i in 0..uibox.elems.len() {
            if uibox.elems[i].reference == reference {
                index = i;
                break;
            }
        }
        let elem = uibox.elems.swap_remove(index);
        return Some((elem, parent_ref, index));
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
            let elem = cat.elems.swap_remove(index);
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
        }
    }
}

pub fn draw_element_text(
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