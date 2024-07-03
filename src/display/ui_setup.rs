use std::{path::Path, sync::{Arc, RwLock}};

use winit::dpi::Position;

use crate::{model::{materials::texture::{Texture, TextureType}, maths::vec3::Vec3, scene::Scene}, picker::get_files_in_folder, render::render_threads::start_render_threads, ui::{elements::{uielement::{Category, UIElement}, utils::{ElemType, FnApply, FnSubmit, Property, Value}, Displayable}, settings, ui::UI, uibox::{BoxPosition, UIBox}, uisettings::{self, UISettings}, utils::get_parent_ref}};

pub fn setup_uisettings(ui: &mut UI, scene: &Arc<RwLock<Scene>>) {
    let mut settings_box = UIBox::new("uisettings", BoxPosition::TopRight(50, 50), ui.uisettings().gui_width);
    settings_box.add_elements(ui.uisettings().get_fields("UI settings", ui.uisettings()));
    settings_box.add_elements(scene.read().unwrap().settings().get_fields("Render settings", ui.uisettings()));
    settings_box.add_elements(get_texture_ui("Color", scene.read().unwrap().elements()[0].material().color(), Box::new(
        |value: Texture, scene: &Arc<RwLock<Scene>>| {
            scene.write().unwrap().elements_as_mut()[0].material_mut().set_color(value);
    }), ui.uisettings()));
    settings_box.set_edit_bar(ui.uisettings(), None);

    let index = ui.add_box(settings_box);
    ui.set_active_box(index);
}

pub fn setup_ui(scene: &Arc<RwLock<Scene>>) -> UI {
    let (ra, tb) = start_render_threads(Arc::clone(&scene));
    tb.send(true).unwrap();
    let mut ui = UI::default(ra, tb);
    setup_uisettings(&mut ui, scene);
    ui
}

pub fn open_file_ui(path: String, box_name: String, submit: FnSubmit, settings: &UISettings) {
    let files = get_files_in_folder(&path);
    if let Ok(files) = files {
        let mut file_box =  UIBox::new("file_box", BoxPosition::Center, settings.gui_width);
        let mut cat = UIElement::new(&box_name, "cat_file", ElemType::Category(Category::default()), settings);
        let mut i = 0;
        for file in files {
            let reference = "cat_file".to_string() + &i.to_string();
            cat.add_element(UIElement::new(file.as_str(), &reference, ElemType::Button(Box::new(
                |scene, ui| {
                    // submit(Value::Text(()))
                }
            )), settings))
        }
    }
    
}

pub fn get_vector_ui(initial_value: Vec3, name: &str, reference: &str, settings: &UISettings, submit_x: FnSubmit, submit_y: FnSubmit, submit_z: FnSubmit, color: bool) -> UIElement{
    let mut category = UIElement::new(name, reference, ElemType::Category(Category::default()), settings);
    let x_label;
    let y_label;
    let z_label;
    if color {
        x_label = "R";
        y_label = "G";
        z_label = "B";
    } else {
        x_label = "X";
        y_label = "Y";
        z_label = "Z";
    }
    category.add_element(UIElement::new(x_label, "x", ElemType::Property(Property::new(Value::Float(*initial_value.x()), submit_x, Box::new(|_| Ok(())), settings)), settings));
    category.add_element(UIElement::new(y_label, "y", ElemType::Property(Property::new(Value::Float(*initial_value.y()), submit_y, Box::new(|_| Ok(())), settings)), settings));
    category.add_element(UIElement::new(z_label, "z", ElemType::Property(Property::new(Value::Float(*initial_value.z()), submit_z, Box::new(|_| Ok(())), settings)), settings));
    category
}

pub fn get_vector_from_vector_ui(vector_elem: &UIElement, normalized: bool) -> Vec3 {
    let mut vec3 = Vec3::new(0., 0., 0.);
    if let ElemType::Category(cat) = &vector_elem.elem_type {
        let mut i = 0;
        for elem in &cat.elems {
            if let ElemType::Property(property) = &elem.elem_type {
                if let Value::Float(float_value) = &property.value {
                    match i {
                        0 => {
                            vec3.set_x(*float_value);
                        }
                        1 => {
                            vec3.set_y(*float_value);
                        }
                        2 => {
                            vec3.set_z(*float_value);
                        }
                        _ => ()
                    }
                }
            }
            i += 1;
        }
    }
    if normalized {
        vec3 = vec3.normalize();
    }
    vec3
}

pub fn get_texture_ui(name: &str, texture: &Texture, submit: Box<impl Fn(Texture, &Arc<RwLock<Scene>>) + 'static>, settings: &UISettings) -> Vec<UIElement> {
    let mut elements_vec = vec![];
    let mut category = UIElement::new(name, name, ElemType::Category(Category::default()), settings);

    let as_file;
    let mut as_vec = Vec3::new(0., 0., 0.);
    let mut as_text = "".to_string();
    let texture_type = match texture {
        Texture::Value(value , ttype) => {
            as_file = false;
            as_vec = value.clone();
            ttype.clone()
        },
        Texture::Texture(value, ttype) => {
            as_file = true;
            as_text = value.clone();
            ttype.clone()
        }
    };
    {
        let texture_type = texture_type.clone();
        let as_file = as_file;
        let mut chk_file = UIElement::new("As file", "chk_file", ElemType::Property(
            Property::new(Value::Bool(as_file), 
                Box::new(move |elem, value, scene, ui| {
                    if let Some(elem) = elem {
                        if let Value::Bool(as_file) = value {
                            let parent_ref = get_parent_ref(elem.reference.clone());
                            if as_file {
                                let file_element_reference = parent_ref + ".as_file";
                                let file_element = ui.get_property_mut(&file_element_reference);
                                if let Some(property) = file_element {
                                    if let Value::Text(file) = &property.value {
                                        submit(Texture::Texture(file.clone(), texture_type.clone()), scene);
                                    }
                                }
                            } else {
                                let value_element_reference = parent_ref + ".as_value";
                                let value_element = ui.get_element_mut(value_element_reference);
                                if let Some(elem) = value_element {
                                    if let ElemType::Property(property) = &elem.elem_type {
                                        submit(Texture::from_value(&property.value), scene);
                                    } else {
                                        submit(Texture::from_vector(&"".to_string(), get_vector_from_vector_ui(elem, false)), scene);
                                    }
                                }
                            }
                        }
                    }
                })
                , Box::new(|_| Ok(())), settings)
        ), settings);
        chk_file.on_click = Some(Box::new(move |elem, _, ui| {
            let elem = elem.unwrap();
            if let ElemType::Property(property) = &elem.elem_type {
                if let Value::Bool(is_file) = property.value {
                    let parent_ref = get_parent_ref(elem.reference.clone());
                        let file_element_reference = parent_ref.clone() + ".as_file";
                        let file_element = ui.get_element_mut(file_element_reference).unwrap();
                        file_element.visible = is_file;
                        let value_element_reference = parent_ref + ".as_value";
                        let value_element = ui.get_element_mut(value_element_reference).unwrap();
                        value_element.visible = !is_file;
                }
            }
        }));
        category.add_element(chk_file);
    }
    let mut elem = match texture_type {
        TextureType::Float => {
            UIElement::new("Value", "as_value", ElemType::Property(Property::new(Value::Float(as_vec.to_value()), Box::new(|_, _, _, _| ()), Box::new(|_| Ok(())), settings)), settings)
        }
        TextureType::Vector => {
            get_vector_ui(as_vec, "Value", "as_value", settings, Box::new(|_, _, _, _| ()), Box::new(|_, _, _, _| ()), Box::new(|_, _, _, _| ()), false)
        }
        TextureType::Color => {
            get_vector_ui(as_vec, "Value", "as_value", settings, Box::new(|_, _, _, _| ()), Box::new(|_, _, _, _| ()), Box::new(|_, _, _, _| ()), true)
        }
        _ => panic!("There should not be a non float/vector/color texture")
    };
    elem.visible = !as_file;
    category.add_element(elem);

    let mut elem = UIElement::new("File", "as_file", ElemType::Property(Property::new(Value::Text(as_text), Box::new(|_, _, _, _| ()), Box::new(|_| Ok(())), settings)), settings);
    elem.visible = as_file;
    //TODO: Open file picker
    category.add_element(elem);


    elements_vec.push(category);
    elements_vec
}