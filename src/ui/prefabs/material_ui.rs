use std::sync::{Arc, RwLock};

use crate::{model::{scene::Scene, Element}, ui::{ui::UI, uielement::{Category, UIElement}, utils::misc::ElemType}};

use super::texture_ui::get_texture_ui;

pub fn get_material_ui(element: &Element, ui: &mut UI, scene: &Arc<RwLock<Scene>>) -> UIElement {
    let mut material_category = UIElement::new("Material", "material", ElemType::Category(Category::default()), ui.uisettings());

    let id_element = element.id();

    //Color
    material_category.add_element(get_texture_ui("color", element.material().color(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_color(texture);
        }
    }), ui.uisettings()));

    //Norm variation
    material_category.add_element(get_texture_ui("norm", element.material().norm(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_norm(texture);
        }
    }), ui.uisettings()));

    //Metalness
    material_category.add_element(get_texture_ui("metalness", element.material().metalness(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_metalness(texture);
        }
    }), ui.uisettings()));

    //Refraction
    material_category.add_element(get_texture_ui("refraction", element.material().refraction(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_refraction(texture);
        }
    }), ui.uisettings()));

    //Roughness
    material_category.add_element(get_texture_ui("roughness", element.material().roughness(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_roughness(texture);
        }
    }), ui.uisettings()));

    //Emissive
    material_category.add_element(get_texture_ui("emissive", element.material().emissive(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_emissive(texture);
        }
    }), ui.uisettings()));

    //Opacity
    material_category.add_element(get_texture_ui("opacity", element.material().opacity(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_opacity(texture);
        }
    }), ui.uisettings()));

    material_category
}