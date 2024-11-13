use super::texture_ui::get_texture_ui;
use std::sync::{Arc, RwLock};
use crate::{
    model::{scene::Scene, Element},
    ui::{
        ui::UI,
        uielement::{Category, UIElement},
        utils::misc::ElemType
    }
};

pub fn get_material_ui(element: &Element, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
    let mut material_category = UIElement::new("Material", "material", ElemType::Category(Category::default()), ui.uisettings());

    let id_element = element.id();

    //Color
    material_category.add_element(get_texture_ui("Color", element.material().color(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_color(texture);
        }
    }), ui.uisettings(), false, Some(0.), Some(1.)));

    //Displacement
    material_category.add_element(get_texture_ui("Displacement", element.material().displacement(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_displacement(texture);
        }
    }), ui.uisettings(), true, None, None));

    //Norm variation
    let norm_variation = get_texture_ui("Norm", element.material().norm(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_norm(texture);
        }
    }), ui.uisettings(), true, None, None);
    material_category.add_element(norm_variation);

    //Metalness
    let metalness = get_texture_ui("Metalness", element.material().metalness(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_metalness(texture);
        }
    }), ui.uisettings(), false, Some(0.), Some(1.));
    material_category.add_element(metalness);

    //Refraction
    let refraction = get_texture_ui("Refraction", element.material().refraction(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_refraction(texture);
        }
    }), ui.uisettings(), false, Some(0.), Some(1.));
    material_category.add_element(refraction);

    //Roughness
    let roughness = get_texture_ui("Roughness", element.material().roughness(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_roughness(texture);
        }
    }), ui.uisettings(), false, Some(0.), Some(1.));
    material_category.add_element(roughness);

    //Emissive
    material_category.add_element(get_texture_ui("Emissive", element.material().emissive(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_emissive(texture);
        }
    }), ui.uisettings(), false, Some(0.), None));

    //Opacity
    material_category.add_element(get_texture_ui("Opacity", element.material().opacity(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_opacity(texture);
        }
    }), ui.uisettings(), false, Some(0.), Some(1.)));

    material_category
}