use std::{fs::read_dir, io::Result, path::{self, Path}};

use image::Rgba;
use crate::ui::{
        uibox::{BoxPosition, UIBox},
        uielement::{Category, UIElement},
        uisettings::UISettings,
        utils::{
            misc::{ElemType, FnSubmitValue, Property, Value}, style::Style, ui_utils::get_parent_ref
        }
    };

#[derive(Debug)]
pub struct Folder {
    files: Vec<String>,
    folders: Vec<String>
}

pub fn get_files_in_folder(path: &str) -> Result<Folder> {
    let mut folder: Folder;
    let path = Path::new(path);
    folder = Folder {
        files: Vec::new(),
        folders: Vec::new()
    };

    for entry in read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(filename) = path.file_name() {
                if let Some(filename_str) = filename.to_str() {
                    folder.files.push(filename_str.to_string());
                }
            }
        } else if path.is_dir() {
            if let Some(foldername) = path.file_name() {
                if let Some(foldername_str) = foldername.to_str() {
                    folder.folders.push(foldername_str.to_string());
                }
            }
        }
    }

    folder.files.sort();
    folder.folders.sort();
    Ok(folder)
}

fn is_same_path(path1: &String, path2: &String) -> bool {
    let path1 = path::Path::new(path1);
    let path2 = path::Path::new(path2);
    let normalized_path1 = path1.canonicalize();
    if let Err(_) = normalized_path1 {
        return false;
    }
    let normalized_path2 = path2.canonicalize();
    if let Err(_) = normalized_path2 {
        return false;
    }
    return normalized_path1.unwrap() == normalized_path2.unwrap();
}

fn create_folder_ui(folder: String, value_element: &mut UIElement, category: &mut UIElement, path: &String, settings: &UISettings, initial_value: &String, elem_ref: String, ref_parent: &String) -> Result<bool> {
    let mut found = false;
    let mut folder_cat = UIElement::new(&folder, &elem_ref, ElemType::Category(Category::collapsed()), settings);
    let folder_path = path.clone() + "/" + &folder;
    if create_files_ui(value_element, &mut folder_cat, &folder_path, settings, initial_value.clone(), format!("{}.{}", ref_parent, elem_ref))? == true {
        found = true;
        let cat = folder_cat.get_elem_type_mut();
        if let ElemType::Category(cat) = cat {
            cat.collapsed = false;
        }
    };
    category.add_element(folder_cat);
    Ok(found)
}

fn create_file_ui(file: String, value_element: &mut UIElement, category: &mut UIElement, path: &String, settings: &UISettings, initial_value: &String, elem_ref: String, ref_parent: &String) -> Result<bool> {
    let mut found = false;
    let mut selected = Style::text(settings);
    let not_selected = selected.clone();
    selected.bg_color = Some(not_selected.font_color);
    selected.font_color = Rgba([0, 0, 0, 255]);

    let mut element = UIElement::new(&file, &elem_ref, ElemType::Button(Some(Box::new(
        move |button, _, ui| {
            if let Some(button) = button {
                let mut parent_ref = get_parent_ref(button.reference.clone());
                while !parent_ref.ends_with(".cat_file") {
                    parent_ref = get_parent_ref(parent_ref);
                    if parent_ref.is_empty() {
                        return;
                    }
                }
                let value_property_ref = parent_ref.clone() + ".value";
                let value_property = ui.get_property_mut(&value_property_ref);
                if let Some(value_property) = value_property {
                    let ref_of_current_value = value_property.value.clone();
                    value_property.value = Value::Text(button.reference.clone());
                    if let Value::Text(ref_of_current_value) = ref_of_current_value {
                        let element = ui.get_element_mut(ref_of_current_value);
                        if let Some(element) = element {
                            element.style = not_selected.clone();
                        }
                    }
                    button.style = selected.clone();
                }
            }
        }
    ))), settings);

    let value = path.clone() + "/" + file.clone().as_str();
    if is_same_path(&value, initial_value) {
        element.style = selected.clone();
        if let ElemType::Property(property) = value_element.get_elem_type_mut() {
            property.value = Value::Text(format!("{}.{}", ref_parent, elem_ref));
        }
        found = true;
    } else {
        element.style = not_selected.clone();
    }
    element.value = Some(value);
    category.add_element(element);
    Ok(found)
}

fn create_files_ui(value_element: &mut UIElement, category: &mut UIElement, path: &String, settings: &UISettings, initial_value: String, ref_parent: String) -> Result<bool> {
    let mut found = false;
    let folder = get_files_in_folder(path)?;
    let mut i = 0;
    for folder in folder.folders {
        let elem_ref = format!("folder{}", i);
        i += 1;
        if create_folder_ui(folder, value_element, category, path, settings, &initial_value, elem_ref, &ref_parent)? {

        }
    }
    i = 0;
    for file in folder.files {
        let elem_ref = format!("file{}", i);
        i += 1;
        if create_file_ui(file, value_element, category, path, settings, &initial_value, elem_ref, &ref_parent)? {
            found = true;
        }
    }
    return Ok(found);
}

fn create_value_element(settings: &UISettings, id: &str) -> UIElement {
    let mut value_element = UIElement::new("", id, ElemType::Property(
        Property::new(
            Value::Text("".to_string()),
            Box::new(|_, _, _, _| ()),
            Box::new(|_, _, _| Ok(())),
            settings)
    ), settings);
    value_element.style.visible = false;
    return value_element;
}

pub fn get_file_box(default_folder: String, box_name: String, submit: FnSubmitValue, settings: &UISettings, initial_value: String) -> UIBox {
    let mut file_box =  UIBox::new("file_box", BoxPosition::Center, settings.gui_width, settings);
    let mut cat = UIElement::new(&box_name, "cat_file", ElemType::Category(Category::default()), settings);
    cat.on_click = Some(Box::new(move |_element,_scene, ui| {
        ui.destroy_box("file_box");
    }));
    let mut value_element = create_value_element(settings, "value");
    
    let file_ui = create_files_ui(&mut value_element, &mut cat, &default_folder, settings, initial_value, "file_box.cat_file".to_string());
    
    if let Ok(_) = file_ui {
        cat.add_element(value_element);
        file_box.add_elements(vec![cat]);
        file_box.set_edit_bar(settings, Some(Box::new(move |_, scene, ui| {
            let value_element_ref = "file_box.cat_file.value".to_string();
            let element = ui.get_property_mut(&value_element_ref);
            if let Some(element) = element {
                if let Value::Text(value) = element.value.clone() {
                    let selected_element = ui.get_element_mut(value.clone());
                    if let Some(selected_element) = selected_element {
                        if let Some(value) = selected_element.value.clone() {
                            submit(None, Value::Text(value), scene, ui);
                        }
                    }
                }
            }
            ui.destroy_box("file_box");
        })));
        return file_box;
    }
    panic!("Problem opening files");
}