use std::sync::{Arc, RwLock};
use super::{
    file_ui::get_file_box,
    vector_ui::{get_vector_from_vector_ui, get_vector_ui}
};
use crate::{
    model::{
        materials::texture::{Texture, TextureType},
        maths::vec3::Vec3, scene::Scene
    },
    ui::{
        uielement::{Category, UIElement},
        uisettings::UISettings,
        utils::{
            misc::{ElemType, Property, Value},
            ui_utils::get_parent_ref
        }
    }
};


pub fn get_texture_ui(name: &str, texture: &Texture, submit: Box<dyn Fn(Texture, &Arc<RwLock<Scene>>)>, settings: &UISettings, file: bool, only_file: bool, min: Option<f64>, max: Option<f64>, only_file_default: Option<Vec3>) -> UIElement {
    let mut category = UIElement::new(name, name, ElemType::Category(Category::collapsed()), settings);
    

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
            if value != "" {
                as_file = true;
            } else {
                as_file = false;
            }
            as_text = value.clone();
            ttype.clone()
        }
    };
    {
        let texture_type = texture_type.clone();
        let as_file = as_file;
        if file {
            let chk_name ;
            if only_file {
                chk_name = "Textured";
            } else {
                chk_name = "As file"
            }
            let mut chk_file = UIElement::new(chk_name, "chk_file", ElemType::Property(
                Property::new(Value::Bool(as_file), 
                    Box::new(move |elem, value, context, ui| {
                        if let Some(elem) = elem {
                            if let Value::Bool(as_file) = value {
                                let parent_ref = get_parent_ref(elem.reference.clone());
                                if as_file {
                                    let file_element_reference = parent_ref + ".as_file";
                                    let file_element = ui.get_property_mut(&file_element_reference);
                                    if let Some(property) = file_element {
                                        if let Value::Text(file) = &property.value {
                                            let scene = match context.active_scene {
                                                Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                                                None => return,
                                            };
                                            submit(Texture::Texture(file.clone(), texture_type.clone()), scene);
                                        }
                                    }
                                } else if only_file {
                                    if let Some(default) = only_file_default {
                                        let scene = match context.active_scene {
                                            Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                                            None => return,
                                        };
                                        submit(Texture::from_vector("", default), scene);
                                    }
                                } else {
                                    let scene = match context.active_scene {
                                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                                        None => return,
                                    };
                                    let value_element_reference = parent_ref + ".as_value";
                                    let value_element = ui.get_element_mut(value_element_reference);
                                    if let Some(elem) = value_element {
                                        if let ElemType::Property(property) = &elem.elem_type {
                                            submit(Texture::from_value(&property.value), scene);
                                        } else {
                                            submit(Texture::from_vector(&"".to_string(), get_vector_from_vector_ui(elem, false)), scene);
                                        }
                                    } else {
                                        submit(Texture::Texture("".to_string(), texture_type.clone()), scene);
                                    }
                                }
                            }
                        }
                    })
                    , Box::new(|_, _, _| Ok(())), settings)
            ), settings);
            chk_file.on_click = Some(Box::new(move |elem, _, ui| {
                let elem = elem.unwrap();
                if let ElemType::Property(property) = &elem.elem_type {
                    if let Value::Bool(is_file) = property.value {
                        let parent_ref = get_parent_ref(elem.reference.clone());
                            let file_element_reference = parent_ref.clone() + ".as_file";
                            let file_element = ui.get_element_mut(file_element_reference).unwrap();
                            file_element.style.visible = is_file;
                            let value_element_reference = parent_ref + ".as_value";
                            if let Some(value_element) = ui.get_element_mut(value_element_reference) {
                                value_element.style.visible = !is_file;
                            }
                    }
                }
            }));
            category.add_element(chk_file);
        }
    }
    if !only_file {
        let mut elem = match texture_type {
            TextureType::Float => {
                let float_property = Property::new(Value::Float(as_vec.to_value()), Box::new(|_, _, _, _| ()), Box::new(move |value: &Value, _, _| {
                    if let Value::Float(value) = value {
                        if let Some(min) = min.clone() {
                            if *value < min {
                                return Err(String::from("The value should not be inferior to ") + &min.to_string());
                            }
                        }
                        if let Some(max) = max.clone() {
                            if *value > max {
                                return Err(String::from("The value should not be superior to ") + &max.to_string());
                            }
                        }
                    }
                    Ok(())
                }), settings);
                UIElement::new("Value", "as_value", ElemType::Property(float_property), settings)
            }
            TextureType::Vector => {
                get_vector_ui(as_vec, "Value", "as_value", settings, Box::new(|_, _, _, _| ()), Box::new(|_, _, _, _| ()), Box::new(|_, _, _, _| ()), false, min, max)
            }
            TextureType::Color => {
                get_vector_ui(as_vec, "Value", "as_value", settings, Box::new(|_, _, _, _| ()), Box::new(|_, _, _, _| ()), Box::new(|_, _, _, _| ()), true, min, max)
            }
            _ => panic!("There should not be a non float/vector/color texture")
        };
        elem.style.visible = !as_file;
        category.add_element(elem);
    }
    let as_text = as_text;
    let name = name.to_string();
    let property_name = name.clone();
    let settings = settings.clone();
    let mut elem = UIElement::new("File", "as_file", ElemType::Property(Property::new(Value::Text(as_text.clone()), Box::new(|_, _, _, _| ()), Box::new(move |value, elem, _ui| {
        if !elem.style.visible {
            return Ok(());
        }
        if let Value::Text(file) = value {
            if !file.is_empty() {
                return Ok(());
            }
        }
        return Err(format!("No file has been selected for property {}", property_name));
    }), &settings)), &settings);
    elem.on_click = Some(Box::new(move |elem, _scene, ui| {
        if let Some(elem) = elem {
            let reference = elem.reference.clone();
            let mut initial_value = as_text.clone();
            if let ElemType::Property(property) = elem.get_elem_type() {
                initial_value = property.value.to_string();
            }
            let file_box = get_file_box("./textures/".to_string(), name.clone(), Box::new(move |_, value, _scene, ui| {
                let elem = ui.get_property_mut(&reference.clone()).unwrap();
                elem.value = value;
            }), &settings.clone(), initial_value);
            let box_reference = file_box.reference.clone();
            ui.add_box(file_box);
            ui.set_active_box(box_reference);
        }
    }));
    elem.style.visible = as_file;
    elem.style_mut().disabled = true;
    category.add_element(elem);
    category
}