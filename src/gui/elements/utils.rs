use std::borrow::Borrow;

use image::RgbaImage;

use crate::{
    display::utils::draw_text2,
    gui::{
        draw::draw_background,
        textformat::{Formattable, Style},
    },
    model::scene::Scene,
};

use super::{
    ui::UI,
    uielement::{ElemType, UIElement},
    HitBox,
};

pub fn get_pos(parent_pos: (u32, u32), offset_pos: (u32, u32), indent: u32) -> (u32, u32) {
    (
        parent_pos.0 + offset_pos.0 + indent,
        parent_pos.1 + offset_pos.1,
    )
}

pub fn get_size(text: &String, style: &Style, max_size: (u32, u32)) -> (u32, u32) {
    let mut height = style.font_size() as u32 + style.padding_bot + style.padding_top;
    let mut width =
        style.font_size() as u32 / 2 * text.len() as u32 + style.padding_left + style.padding_top;

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
    (width.min(max_size.0), height.min(max_size.0))
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
        } else if let ElemType::Row(elems) = &mut elem.elem_type {
            let mut index = 0;
            for i in 0..elems.len() {
                if elems[i].reference == reference {
                    index = i;
                    break;
                }
            }
            let elem = elems.swap_remove(index);
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

pub fn draw_element_text(
    img: &mut RgbaImage,
    text: String,
    pos: (u32, u32),
    size: (u32, u32),
    format: &Style,
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

// pub fn refresh_formats(mut elems: &mut Vec<UIElement>, ui: &UI) {
//     for_each_element(&mut elems, &None, &Some(ui), |elem, scene, ui| {
//         if let Some(ui) = ui {
//             elem.format = elem.elem_type.base_style(ui.uisettings());
//         }
//     });
// }

// pub fn for_each_element(
//     elems: &mut Vec<UIElement>,
//     scene: &Option<&Scene>,
//     ui: &Option<&UI>,
//     fonction: impl Fn(&mut UIElement, &Option<&Scene>, &Option<&UI>),
// ) {
//     for i in 0..elems.len() {
//         let mut elem = elems.swap_remove(i);
//         &fonction(&mut elem, scene, ui);
//         elems.insert(i, elem);
//         let elem = elems.get_mut(i).unwrap();
//         if let ElemType::Category(cat) = &mut elem.elem_type {
//             for_each_element(&mut cat.elems, scene, ui, &fonction);
//         } else if let ElemType::Row(elems) = &mut elem.elem_type {
//             for_each_element(elems, scene, ui, &fonction);
//         }
//     }
// }

// pub fn for_each_element_mut(
//     elems: &mut Vec<UIElement>,
//     scene: &Option<&mut Scene>,
//     ui: &Option<&mut UI>,
//     fonction: impl Fn(&mut UIElement, &Option<&mut Scene>, &Option<&mut UI>),
// ) {
//     for i in 0..elems.len() {
//         let mut elem = elems.swap_remove(i);
//         &fonction(&mut elem, scene, ui);
//         elems.insert(i, elem);
//         let elem = elems.get_mut(i).unwrap();
//         if let ElemType::Category(cat) = &mut elem.elem_type {
//             for_each_element_mut(&mut cat.elems, scene, ui, &fonction);
//         } else if let ElemType::Row(elems) = &mut elem.elem_type {
//             for_each_element_mut(elems, scene, ui, &fonction);
//         }
//     }
// }
