use std::{fs::read_dir, io::Result, path::Path};

use image::Rgba;
use crate::ui::{
        uibox::{BoxPosition, UIBox},
        uielement::{Category, UIElement},
        uisettings::UISettings,
        utils::{
            misc::{ElemType, FnSubmitValue, Property, Value},
            ui_utils::get_parent_ref,
            style::Style
        }
    };

#[derive(Debug)]
pub struct Folder {
    name: String,
    files: Vec<String>,
    folders: Vec<Folder>
}

pub fn get_files_in_folder(path: &str) -> Result<Folder> {
    let mut folder: Folder;
    let path = Path::new(path);
    if let Some(filename) = path.file_name() {
        if let Some(filename_str) = filename.to_str() {
            folder = Folder {
                name: filename_str.to_string(),
                files: Vec::new(),
                folders: Vec::new()
            };
        } else {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid filename"));
        }
    } else {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid filename"));
    }

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
            if let Some(folderpath) = path.to_str() {
                if let Ok(sub_folder) = get_files_in_folder(folderpath) {
                    folder.folders.push(sub_folder);
                }
            }
        }
    }

    folder.files.sort();
    folder.folders.sort_by_key(|folder| folder.name.clone());
    dbg!(&folder);
    Ok(folder)
}

fn get_folder_ui(folder: Folder, settings: &UISettings) -> Vec<UIElement> {
    let mut style_selected = Style::text(settings);
    let style_not_selected = style_selected.clone();
    style_selected.bg_color = Some(style_not_selected.font_color);
    style_selected.font_color = Rgba([0, 0, 0, 255]);
    
    let mut files: Vec<UIElement> = Vec::new();

    let mut i = 0;
    for subfolder in folder.folders {
        let mut folder_cat = UIElement::new(&subfolder.name, format!("folder{}", i).as_str(), ElemType::Category(Category::collapsed()), settings);
        for file in get_folder_ui(subfolder, settings) {
            folder_cat.add_element(file);
        }
        files.push(folder_cat);
        i += 1;
    }
    i = 0;
    for file in folder.files {
        let mut element = UIElement::new(&file, format!("file{}", i).as_str(), ElemType::Button(Some(Box::new(
            move |button, _, ui| {
                if let Some(button) = button {
                    let file_box_ref = "file_box.cat_file";
                    let value_property_ref = file_box_ref.to_string() + ".value";
                    let value_property = ui.get_property_mut(&value_property_ref);
                    if let Some(value_property) = value_property {
                        let ref_of_current_value = value_property.value.clone();
                        value_property.value = Value::Text(button.reference.clone());
                        if let Value::Text(ref_of_current_value) = ref_of_current_value {
                            let element = ui.get_element_mut(ref_of_current_value);
                            if let Some(element) = element {
                                element.style = style_not_selected.clone();
                            }
                        }
                        button.style = style_selected.clone();
                    }
                }
            }
        ))), settings);
        element.style = style_not_selected.clone();
        files.push(element);
        i += 1;
    }
    files
}

pub fn get_file_box(path: String, box_name: String, submit: FnSubmitValue, settings: &UISettings, initial_value: String) -> UIBox {
    let mut style_selected = Style::text(settings);
    let style_not_selected = style_selected.clone();
    style_selected.bg_color = Some(style_not_selected.font_color);
    style_selected.font_color = Rgba([0, 0, 0, 255]);

    let folder = get_files_in_folder(&path);
    if let Ok(folder) = folder {

        let mut file_box =  UIBox::new("file_box", BoxPosition::Center, settings.gui_width, settings);
        let mut cat = UIElement::new(&box_name, "cat_file", ElemType::Category(Category::default()), settings);
        
        
        
        
        
        
        // let files = folder.files;
        
        // let mut cat = UIElement::new(&box_name, "cat_file", ElemType::Category(Category::default()), settings);
        // let mut i = 0;
        // let mut nb_page = 1;
        // let mut nb_elements_per_page = 0;
        // let mut id_page = String::from("page") + &nb_page.to_string();
        // let mut name_page = String::from("Page ") + &nb_page.to_string();
        // let mut page = UIElement::new(&name_page, &id_page, ElemType::Category(Category::collapsed()), settings);

        let mut ref_of_initial_value = "".to_string();
        // for file in files {
        //     // if nb_elements_per_page == 10 {
        //     // }
        //     let reference = "cat_file".to_string() + &i.to_string();
        //     let not_selected = style_not_selected.clone();
        //     let selected= style_selected.clone();
            // let mut element = UIElement::new(&file, &reference, ElemType::Button(Some(Box::new(
            //     move |button, _, ui| {
            //         if let Some(button) = button {
            //             let parent_ref = get_parent_ref(get_parent_ref(button.reference.clone()));
            //             let value_property_ref = parent_ref.clone() + ".value";
            //             let value_property = ui.get_property_mut(&value_property_ref);
            //             if let Some(value_property) = value_property {
            //                 let ref_of_current_value = value_property.value.clone();
            //                 value_property.value = Value::Text(button.reference.clone());
            //                 if let Value::Text(ref_of_current_value) = ref_of_current_value {
            //                     let element = ui.get_element_mut(ref_of_current_value);
            //                     if let Some(element) = element {
            //                         element.style = not_selected.clone();
            //                     }
            //                 }
            //                 button.style = selected.clone();
            //             }
            //         }
            //     }
            // ))), settings);
            if file == initial_value {
                element.style = style_selected.clone();  
                if let ElemType::Category(page_category) = page.get_elem_type_mut() {
                    page_category.collapsed = false;
                }
                ref_of_initial_value = "file_box.cat_file.".to_string() + &id_page + "." + &reference;
            }  else {
                element.style = style_not_selected.clone();
            }
        //     page.add_element(element);
        //     nb_elements_per_page = nb_elements_per_page + 1;
        //     if nb_elements_per_page == 10 {
        //         cat.add_element(page);
        //         nb_page = nb_page + 1;
        //         nb_elements_per_page = 0;
        //         id_page = String::from("page") + &nb_page.to_string();
        //         name_page = String::from("Page ") + &nb_page.to_string();
        //         page = UIElement::new(&name_page, &id_page, ElemType::Category(Category::collapsed()), settings);
        //     }

        //     i += 1;
        // }
        // if nb_elements_per_page > 0 {
        //     cat.add_element(page);
        // }

        let mut value_element = UIElement::new("", "value", ElemType::Property(
            Property::new(
                Value::Text(ref_of_initial_value),
                Box::new(|_, _, _, _| ()),
                Box::new(|_, _, _| Ok(())),
                settings)
        ), settings);
        value_element.style.visible = false;
        cat.add_element(value_element);

        for file in get_folder_ui(folder, settings) {
            cat.add_element(file);
        }

        file_box.add_elements(vec![cat]);
        file_box.set_edit_bar(settings, Some(Box::new(move |_, scene, ui| {
            let value_element_ref = "file_box.cat_file.value".to_string();
            let element = ui.get_property_mut(&value_element_ref);
            if let Some(element) = element {
                if let Value::Text(value) = element.value.clone() {
                    let selected_element = ui.get_element_mut(value.clone());
                    if let Some(selected_element) = selected_element {
                        submit(None, Value::Text(selected_element.text.clone()), scene, ui);
                    }
                }
            }
            ui.destroy_box("file_box");
        })));
        return file_box;
    }
    panic!("Problem opening files");
}