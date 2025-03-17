use super::texture_ui::get_texture_ui;
use std::sync::{Arc, RwLock};
use crate::{
    model::{materials::texture::Texture, maths::vec3::Vec3, scene::Scene, element::Element},
    ui::{
        ui::UI,
        uielement::{Category, UIElement},
        utils::misc::{ElemType, Property, Value}
    }
};

pub fn get_material_ui(element: &Element, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
    let mut material_category = UIElement::new("Material", "material", ElemType::Category(Category::default()), ui.uisettings());

    let id_element = element.id();

    //Color
    material_category.add_element(get_texture_ui("Color", element.material().color(), Box::new(move |texture, scene| {
        let mut scene_write = scene.write().unwrap();
        if let Texture::Texture(file, _) = &texture {
            scene_write.load_texture(file, None);
        }
        if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
            element.material_mut().set_color(texture);
        }
        else if let Some(element) = scene_write.element_mut_by_id(id_element) {
            element.material_mut().set_color(texture);
        }
    }), ui.uisettings(), true, false, Some(0.), Some(1.), None));

    //Displacement
    material_category.add_element(get_texture_ui("Displacement", element.material().displacement(), Box::new(move |texture, scene| {
        let mut scene_write = scene.write().unwrap();
        if let Texture::Texture(file, _) = &texture {
            scene_write.load_texture(file, None);
        }
        if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
            element.material_mut().set_displacement(texture);
        } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
            element.material_mut().set_displacement(texture);
        }
    }), ui.uisettings(), true, true, None, None, None));

    //Norm variation
    let norm_variation = get_texture_ui("Norm", element.material().norm(), Box::new(move |texture, scene| {
        let mut scene_write = scene.write().unwrap();
        if let Texture::Texture(file, _) = &texture {
            scene_write.load_texture(file, None);
        }
        if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
            element.material_mut().set_norm(texture);
        } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
            element.material_mut().set_norm(texture);
        }
    }), ui.uisettings(), true, true, None, None, Some(Vec3::new(0., 0., 1.)));
    material_category.add_element(norm_variation);

    //Metalness
    let metalness = get_texture_ui("Metalness", element.material().metalness(), Box::new(move |texture, scene| {
        let mut scene_write = scene.write().unwrap();
        if let Texture::Texture(file, _) = &texture {
            scene_write.load_texture(file, None);
        }
        if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
            element.material_mut().set_metalness(texture);
        } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
            element.material_mut().set_metalness(texture);
        }
    }), ui.uisettings(), true, false, Some(0.), Some(1.), None);
    material_category.add_element(metalness);

    //Refraction
    let refraction = UIElement::new("Refraction", "refraction", ElemType::Property(Property::new(Value::Float(element.material().refraction()),
        Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
            let mut scene_write = scene.write().unwrap();
            if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
                if let Value::Float(float_value) = value {
                    element.material_mut().set_refraction(float_value);
                }
            } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
                if let Value::Float(float_value) = value {
                    element.material_mut().set_refraction(float_value);
                }
            }
        }), Box::new(|value, _, _| {
            if let Value::Float(float_value) = value {
                if float_value >= &1. {
                    Ok(())
                } else {
                    Err("Refraction index cannot be inferior to 1.".to_string())    
                }
            }else {
                Err("Refraction must be a valid float.".to_string())
            }
        }), ui.uisettings())), ui.uisettings());
    material_category.add_element(refraction);

    //Transparency
    let transparency = get_texture_ui("Transparency", element.material().transparency(), Box::new(move |texture, scene| {
        let mut scene_write = scene.write().unwrap();
        if let Texture::Texture(file, _) = &texture {
            scene_write.load_texture(file, None);
        }
        if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
            element.material_mut().set_transparency(texture);
        } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
            element.material_mut().set_transparency(texture);
        }
    }), ui.uisettings(), true, false, Some(0.), Some(1.), None);
    material_category.add_element(transparency);

    //Roughness
    let roughness = get_texture_ui("Roughness", element.material().roughness(), Box::new(move |texture, scene| {
        let mut scene_write = scene.write().unwrap();
        if let Texture::Texture(file, _) = &texture {
            scene_write.load_texture(file, None);
        }
        if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
            element.material_mut().set_roughness(texture);
        } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
            element.material_mut().set_roughness(texture);
        }
    }), ui.uisettings(), true, false, Some(0.), Some(1.), None);
    material_category.add_element(roughness);

    //Emissive
    material_category.add_element(get_texture_ui("Emissive", element.material().emissive(), Box::new(move |texture, scene| {
        let mut scene_write = scene.write().unwrap();
        if let Texture::Texture(file, _) = &texture {
            scene_write.load_texture(file, None);
        }
        if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
            element.material_mut().set_emissive(texture);
        } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
            element.material_mut().set_emissive(texture);
        }
    }), ui.uisettings(), true, false, Some(0.), None, None));

    //Emissive intensity
     material_category.add_element(UIElement::new("Emissive intensity", "emissive_intensity", ElemType::Property(Property::new(Value::Float(element.material().emissive_intensity()),
        Box::new(move |_, value, context, _| {
            if let Some(scene) = context.get_active_scene() {
                let mut scene_write = scene.write().unwrap();
                if let Value::Float(float_value) = value {
                    if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
                        element.material_mut().set_emissive_intensity(float_value);
                    } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
                        element.material_mut().set_emissive_intensity(float_value);
                    }
                }
            }
        }), Box::new(|value, _, _| {
            if let Value::Float(float_value) = value {
                if float_value >= &0. {
                    Ok(())
                } else {
                    Err("Emissive intensity cannot be negative.".to_string())    
                }
            } else {
                Err("Emissive intensity must be a valid float.".to_string())
            }
        }), ui.uisettings())), ui.uisettings()));

    //Opacity
    material_category.add_element(get_texture_ui("Opacity", element.material().opacity(), Box::new(move |texture, scene| {
        let mut scene_write = scene.write().unwrap();
        if let Texture::Texture(file, _) = &texture {
            scene_write.load_texture(file, None);
        }
        if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
            element.material_mut().set_opacity(texture);
        } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
            element.material_mut().set_opacity(texture);
        }
    }), ui.uisettings(), true, false, Some(0.), Some(1.), None));

    //Reflectivity
    material_category.add_element(UIElement::new("Reflectivity", "reflectivity", ElemType::Property(Property::new(Value::Float(element.material().reflectivity()),
    Box::new(move |_, value, context, _| {
        if let Some(scene) = context.get_active_scene() {
            let mut scene_write = scene.write().unwrap();
            if let Value::Float(float_value) = value {
                if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
                    element.material_mut().set_reflectivity(float_value);
                } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
                    element.material_mut().set_reflectivity(float_value);
                }
            }
        }
    }), Box::new(|value, _, _| {
        if let Value::Float(float_value) = value {
            if float_value < &0. {
                return Err("Reflectivity cannot be negative.".to_string());
            }
            if float_value > &1. {
                return Err("Reflectivity cannot be over 1".to_string());
            }
            Ok(())
        } else {
            Err("Reflectivity must be a valid float.".to_string())
        }
    }), ui.uisettings())), ui.uisettings()));


    let mut mapping_category = UIElement::new("Mapping", "mapping", ElemType::Category(Category::collapsed()), ui.uisettings());
    let mut scale_category = UIElement::new("Scale", "scale", ElemType::Category(Category::default()), ui.uisettings());
    let mut shift_category = UIElement::new("Shift", "shift", ElemType::Category(Category::default()), ui.uisettings());

    //U scale
    scale_category.add_element(UIElement::new("u", "u", ElemType::Property(Property::new(Value::Float(element.material().u_scale()),
    Box::new(move |_, value, context, _| {
        if let Some(scene) = context.get_active_scene() {
            let mut scene_write = scene.write().unwrap();
            if let Value::Float(float_value) = value {
                if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
                    element.material_mut().set_u_scale(float_value);
                } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
                    element.material_mut().set_u_scale(float_value);
                }
            }
        }
    }), Box::new(|value, _, _| {
        if let Value::Float(_) = value {
            Ok(())
        } else {
            Err("u Scale must be a valid float.".to_string())
        }
    }), ui.uisettings())), ui.uisettings()));

    //V scale
    scale_category.add_element(UIElement::new("v", "v", ElemType::Property(Property::new(Value::Float(element.material().v_scale()),
    Box::new(move |_, value, context, _| {
        if let Some(scene) = context.get_active_scene() {
            let mut scene_write = scene.write().unwrap();
            if let Value::Float(float_value) = value {
                if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
                    element.material_mut().set_v_scale(float_value);
                } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
                    element.material_mut().set_v_scale(float_value);
                }
            }
        }
    }), Box::new(|value, _, _| {
        if let Value::Float(_) = value {
            Ok(())
        } else {
            Err("v Scale must be a valid float.".to_string())
        }
    }), ui.uisettings())), ui.uisettings()));

    //U shift
    shift_category.add_element(UIElement::new("u", "u", ElemType::Property(Property::new(Value::Float(element.material().u_shift()),
    Box::new(move |_, value, context, _| {
        if let Some(scene) = context.get_active_scene() {
            let mut scene_write = scene.write().unwrap();
            if let Value::Float(float_value) = value {
                if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
                    element.material_mut().set_u_shift(float_value);
                } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
                    element.material_mut().set_u_shift(float_value);
                }
            }
        }
    }), Box::new(|value, _, _| {
        if let Value::Float(_) = value {
            Ok(())
        } else {
            Err("u Shift must be a valid float.".to_string())
        }
    }), ui.uisettings())), ui.uisettings()));

    //V shift
    shift_category.add_element(UIElement::new("v", "v", ElemType::Property(Property::new(Value::Float(element.material().v_shift()),
    Box::new(move |_, value, context, _| {
        if let Some(scene) = context.get_active_scene() {
            let mut scene_write = scene.write().unwrap();
            if let Value::Float(float_value) = value {
                if let Some(element) = scene_write.composed_element_mut_by_element_id(id_element) {
                    element.material_mut().set_v_shift(float_value);
                } else if let Some(element) = scene_write.element_mut_by_id(id_element) {
                    element.material_mut().set_v_shift(float_value);
                }
            }
        }
    }), Box::new(|value, _, _| {
        if let Value::Float(_) = value {
            Ok(())
        } else {
            Err("v Shift must be a valid float.".to_string())
        }
    }), ui.uisettings())), ui.uisettings()));
    mapping_category.add_element(scale_category);
    mapping_category.add_element(shift_category);
    material_category.add_element(mapping_category);


    material_category
}