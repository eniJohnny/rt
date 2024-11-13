use image::Rgba;

use crate::{model::maths::vec3::Vec3, ui::{uielement::{Category, UIElement}, uisettings::UISettings, utils::misc::{ElemType, FnSubmitValue, Property, Value}}};

pub fn get_vector_ui(initial_value: Vec3, name: &str, reference: &str, settings: &UISettings, submit_x: FnSubmitValue, submit_y: FnSubmitValue, submit_z: FnSubmitValue, color: bool, min: Option<f64>, max: Option<f64>) -> UIElement{
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
    let mut property_x = Property::new(Value::Float(*initial_value.x()), submit_x, Box::new(|_, _, _| Ok(())), settings);
    let mut property_y = Property::new(Value::Float(*initial_value.y()), submit_y, Box::new(|_, _, _| Ok(())), settings);
    let mut property_z = Property::new(Value::Float(*initial_value.z()), submit_z, Box::new(|_, _, _| Ok(())), settings);
    if min.is_some() || max.is_some() {
        property_x.fn_validate = Box::new(move |value, _, _| {
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
        });
        property_y.fn_validate = Box::new(move |value, _, _| {
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
        });
        property_z.fn_validate = Box::new(move |value, _, _| {
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
        });
    }

    let mut x = UIElement::new(x_label, "x", ElemType::Property(property_x), settings);
    let mut y = UIElement::new(y_label, "y", ElemType::Property(property_y), settings);
    let mut z = UIElement::new(z_label, "z", ElemType::Property(property_z), settings);
    x.style_mut().font_color = Rgba([255, 150, 150, 255]);
    y.style_mut().font_color = Rgba([150, 255, 150, 255]);
    z.style_mut().font_color = Rgba([150, 150, 255, 255]);
    category.add_element(x);
    category.add_element(y);
    category.add_element(z);
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