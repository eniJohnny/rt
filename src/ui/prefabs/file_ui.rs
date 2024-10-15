use image::Rgba;

use crate::{picker::get_files_in_folder, ui::{uibox::{BoxPosition, UIBox}, uielement::{Category, UIElement}, uisettings::UISettings, utils::{misc::{ElemType, FnSubmitValue, Property, Value}, style::Style, ui_utils::get_parent_ref}}};



pub fn get_file_box(path: String, box_name: String, submit: FnSubmitValue, settings: &UISettings, initial_value: String) -> UIBox {
    let mut style_selected = Style::text(settings);
    let style_not_selected = style_selected.clone();
    style_selected.bg_color = Some(style_not_selected.font_color);
    style_selected.font_color = Rgba([0, 0, 0, 255]);

    let files = get_files_in_folder(&path);
    if let Ok(files) = files {
        let mut file_box =  UIBox::new("file_box", BoxPosition::Center, settings.gui_width, settings);
        let mut cat = UIElement::new(&box_name, "cat_file", ElemType::Category(Category::default()), settings);
        let mut i = 0;
        let mut nb_page = 1;
        let mut nb_elements_per_page = 0;
        let mut id_page = String::from("page") + &nb_page.to_string();
        let mut name_page = String::from("Page ") + &nb_page.to_string();
        let mut page = UIElement::new(&name_page, &id_page, ElemType::Category(Category::collapsed()), settings);

        let mut ref_of_initial_value = "".to_string();
        for file in files {
            if nb_elements_per_page == 10 {
            }
            let reference = "cat_file".to_string() + &i.to_string();
            let not_selected = style_not_selected.clone();
            let selected= style_selected.clone();
            let mut element = UIElement::new(&file, &reference, ElemType::Button(Some(Box::new(
                move |button, _, ui| {
                    if let Some(button) = button {
                        let parent_ref = get_parent_ref(get_parent_ref(button.reference.clone()));
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
            if file == initial_value {
                element.style = style_selected.clone();  
                if let ElemType::Category(page_category) = page.get_elem_type_mut() {
                    page_category.collapsed = false;
                }
                ref_of_initial_value = "file_box.cat_file.".to_string() + &id_page + "." + &reference;
            }  else {
                element.style = style_not_selected.clone();
            }
            page.add_element(element);
            nb_elements_per_page = nb_elements_per_page + 1;
            if nb_elements_per_page == 10 {
                cat.add_element(page);
                nb_page = nb_page + 1;
                nb_elements_per_page = 0;
                id_page = String::from("page") + &nb_page.to_string();
                name_page = String::from("Page ") + &nb_page.to_string();
                page = UIElement::new(&name_page, &id_page, ElemType::Category(Category::collapsed()), settings);
            }

            i += 1;
        }
        if nb_elements_per_page > 0 {
            cat.add_element(page);
        }

        let mut value_element = UIElement::new("", "value", ElemType::Property(
            Property::new(
                Value::Text(ref_of_initial_value),
                Box::new(|_, _, _, _| ()),
                Box::new(|_, _, _| Ok(())),
                settings)
        ), settings);
        value_element.style.visible = false;
        cat.add_element(value_element);

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