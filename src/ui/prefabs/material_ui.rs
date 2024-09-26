use std::sync::{Arc, RwLock};

use crate::{model::{materials::texture::Texture, maths::vec3::Vec3, scene::Scene, Element}, ui::{ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Value}}};

use super::texture_ui::get_texture_ui;

pub fn get_material_ui(element: &Element, ui: &mut UI, scene: &Arc<RwLock<Scene>>) -> UIElement {
    let mut material_category = UIElement::new("Material", "material", ElemType::Category(Category::default()), ui.uisettings());

    let id_element = element.id();

    //Color
    material_category.add_element(get_texture_ui("color", element.material().color(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_color(texture);
        }
    }), ui.uisettings(), Some(0.), Some(1.)));

    //Norm variation
    let mut normVariation = get_texture_ui("norm", element.material().norm(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_norm(texture);
        }
    }), ui.uisettings(), Some(-1.), Some(1.));
    let zProperty = normVariation.get_property_mut("z").unwrap();
    zProperty.fn_submit = Box::new(move |_, _, scene, _| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            let norm_texture = element.material().norm();
            let mut normalized: Vec3 = Vec3::new(0., 0., 0.);
            let mut is_value = false;
            if let Texture::Value(vec, _) = norm_texture {
                if vec.length() < f64::EPSILON {
                    normalized = Vec3::new(0., 0., 1.);
                } else {
                    normalized = vec.clone().normalize();
                }
                is_value = false;
            }
            if is_value {
                element.material_mut().set_norm(Texture::from_vector("", normalized));
            }
        }
    });
    material_category.add_element(normVariation);

    //Metalness
    let mut metalness = get_texture_ui("metalness", element.material().metalness(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_metalness(texture);
        }
    }), ui.uisettings(), Some(0.), Some(1.));
    material_category.add_element(metalness);

    //Refraction
    let mut refraction = get_texture_ui("refraction", element.material().refraction(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_refraction(texture);
        }
    }), ui.uisettings(), Some(0.), Some(1.));
    material_category.add_element(refraction);

    //Roughness
    let mut roughness = get_texture_ui("roughness", element.material().roughness(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_roughness(texture);
        }
    }), ui.uisettings(), Some(0.), Some(1.));
    material_category.add_element(roughness);

    //Emissive
    material_category.add_element(get_texture_ui("emissive", element.material().emissive(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_emissive(texture);
        }
    }), ui.uisettings(), Some(0.), None));

    //Opacity
    material_category.add_element(get_texture_ui("opacity", element.material().opacity(), Box::new(move |texture, scene| {
        if let Some(element) = scene.write().unwrap().element_mut_by_id(id_element) {
            element.material_mut().set_opacity(texture);
        }
    }), ui.uisettings(), Some(0.), Some(1.)));

    material_category
}