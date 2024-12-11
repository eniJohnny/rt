use super::texture_ui::get_texture_ui;
use std::sync::{Arc, RwLock};
use crate::{
    model::{materials::texture::Texture, maths::vec3::Vec3, scene::Scene, ComposedElement, Element},
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
        let mut texture_to_load = "".to_string();
        if let Some(element) = scene.write().unwrap().composed_element_mut_by_id(id_element) {
            for element in element.composed_shape_mut().elements_as_mut() {
                element.material_mut().set_color(texture.clone());
            }
        } else if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            if let Texture::Texture(file, _) = &texture {
                texture_to_load = file.to_string();
            }
            element.material_mut().set_color(texture);
        }
        scene.write().unwrap().load_texture(&texture_to_load);
    }), ui.uisettings(), true, false, Some(0.), Some(1.), None));

    //Displacement
    material_category.add_element(get_texture_ui("Displacement", element.material().displacement(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().composed_element_mut_by_id(id_element) {
            for elem in element.composed_shape_mut().elements_as_mut() {
                elem.material_mut().set_displacement(texture.clone());
            }
        } else if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_displacement(texture);
        }
    }), ui.uisettings(), true, true, None, None, None));

    //Norm variation
    let norm_variation = get_texture_ui("Norm", element.material().norm(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().composed_element_mut_by_id(id_element) {
            for elem in element.composed_shape_mut().elements_as_mut() {
                elem.material_mut().set_norm(texture.clone());
            }
        } else if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_norm(texture);
        }
    }), ui.uisettings(), true, true, None, None, Some(Vec3::new(0., 0., 1.)));
    material_category.add_element(norm_variation);

    //Metalness
    let metalness = get_texture_ui("Metalness", element.material().metalness(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().composed_element_mut_by_id(id_element) {
            for elem in element.composed_shape_mut().elements_as_mut() {
                elem.material_mut().set_metalness(texture.clone());
            }
        } else if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_metalness(texture);
        }
    }), ui.uisettings(), true, false, Some(0.), Some(1.), None);
    material_category.add_element(metalness);

    //Refraction
    let refraction = UIElement::new("Refraction", "refraction", ElemType::Property(Property::new(Value::Float(element.material().refraction()),
        Box::new(move |_, value, scene, _| {
            if let Some(composed_element) = scene.write().unwrap().composed_element_mut_by_id(id_element) {
                for element in composed_element.composed_shape_mut().elements_as_mut() {
                    if let Value::Float(float_value) = value {
                        element.material_mut().set_refraction(float_value);
                    }
                }
            } else if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
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
        if let Some(element) = scene.write().unwrap().composed_element_mut_by_id(id_element) {
            for elem in element.composed_shape_mut().elements_as_mut() {
                elem.material_mut().set_transparency(texture.clone());
            }
        } else if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_transparency(texture);
        }
    }), ui.uisettings(), true, false, Some(0.), Some(1.), None);
    material_category.add_element(transparency);

    //Roughness
    let roughness = get_texture_ui("Roughness", element.material().roughness(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().composed_element_mut_by_id(id_element) {
            for elem in element.composed_shape_mut().elements_as_mut() {
                elem.material_mut().set_roughness(texture.clone());
            }
        } else if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_roughness(texture);
        }
    }), ui.uisettings(), true, false, Some(0.), Some(1.), None);
    material_category.add_element(roughness);

    //Emissive
    material_category.add_element(get_texture_ui("Emissive", element.material().emissive(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().composed_element_mut_by_id(id_element) {
            for elem in element.composed_shape_mut().elements_as_mut() {
                elem.material_mut().set_emissive(texture.clone());
            }
        } else if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_emissive(texture);
        }
    }), ui.uisettings(), true, false, Some(0.), None, None));

    //Opacity
    material_category.add_element(get_texture_ui("Opacity", element.material().opacity(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().composed_element_mut_by_id(id_element) {
            for elem in element.composed_shape_mut().elements_as_mut() {
                elem.material_mut().set_opacity(texture.clone());
            }
        } else if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_opacity(texture);
        }
    }), ui.uisettings(), true, false, Some(0.), Some(1.), None));

    material_category
}