use std::sync::{Arc, RwLock};
use crate::{
    model::{element::Element, materials::texture::Texture, scene::Scene}, render::common::start_threads, ui::{
        prefabs::{material_ui::get_material_ui, texture_ui::get_texture_ui}, ui::UI, ui_setup::scene_ui::setup_scene_toolbar, uibox::{BoxPosition, UIBox}, uielement::{Category, UIElement}, utils::{misc::ElemType, ui_utils::UIContext, Displayable}
    }, ELEMENT, OBJECTS, SCREEN_HEIGHT_U32, SETTINGS
    };

pub fn setup_settings(ui: &mut UI, context: &mut UIContext) {
    let scene = match context.active_scene {
        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
        None => return,
    };
    let mut settings_box = UIBox::new(SETTINGS, BoxPosition::CenterLeft(10), ui.uisettings().gui_width, ui.uisettings());
    settings_box.add_elements(scene.read().unwrap().settings().get_fields("Render settings", context, ui.uisettings()));
    settings_box.set_edit_bar(ui.uisettings(), None);
    ui.add_box(settings_box);
}

pub fn setup_objects_ui(ui: &mut UI, context: &mut UIContext) {
    let scene = match context.active_scene {
        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
        None => return,
    };
    let mut objects_box = UIBox::new(OBJECTS, BoxPosition::CenterLeft(10), ui.uisettings().gui_width, ui.uisettings());
    objects_box.max_height = SCREEN_HEIGHT_U32 - 100;
    let mut ui_elements = vec![];
    
    ui_elements.push(scene.read().unwrap().camera().get_ui(ui));

    ui_elements.push(get_texture_ui("Skybox", scene.read().unwrap().skybox(), Box::new(
        |value, scene| {
            let mut scene = scene.write().unwrap();
            if let Texture::Texture(path, _) = &value {
                scene.load_texture(&path, None);
            }
            scene.set_skybox(value);
            scene.set_dirty(true);
        },
    ), ui.uisettings(), true, false, None, None, None));

    ui_elements.push(scene.read().unwrap().ambient_light().get_ui(ui, scene));

    for light in scene.read().unwrap().lights() {
        ui_elements.push(light.get_ui(light, ui, scene));
    }
    
    objects_box.add_elements(ui_elements);
    objects_box.set_edit_bar(ui.uisettings(), None);
    ui.add_box(objects_box);
}

pub fn setup_ui() -> (UI, UIContext) {
    let (ra, tb) = start_threads();
    let mut ui = UI::default();
    let context= UIContext::new(ra, tb);
    setup_scene_toolbar(&mut ui, &context);
    (ui, context)
}

pub fn setup_element_ui(element: &Element, ui: &mut UI, scene: &Arc<RwLock<Scene>>) {
    ui.destroy_box(ELEMENT);
    let name = match element.composed_id() {
        Some(composed_id) => "ComposedElement".to_string() + &composed_id.to_string(),
        None => "Element".to_string() + &element.id().to_string()
    };
    let mut elem_box = UIBox::new(ELEMENT, BoxPosition::CenterRight(10), ui.uisettings().gui_width, ui.uisettings());
    let mut category = UIElement::new(&name, &name, ElemType::Category(Category::default()), ui.uisettings());

    category.on_click = Some(Box::new(move |_element,_scene, ui| {
        ui.destroy_box(ELEMENT);
    }));
    let mut is_composed = false;
    if let Some(composed_id) = element.composed_id() {
        if let Some(composed_element) = scene.read().unwrap().composed_element_by_id(composed_id) {
            is_composed = true;
            category.add_element(composed_element.composed_shape().get_ui(composed_element, ui, scene));
        }
    }
    if !is_composed {
        category.add_element(element.shape().get_ui(element, ui, scene));
    }
    let composed_id = element.composed_id().clone();
    category.add_element(get_material_ui(element, ui, scene));
    elem_box.add_elements(vec![category]);
    elem_box.set_edit_bar(ui.uisettings(), Some(Box::new(move |_, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
        let mut scene_write = scene.write().unwrap();
        if let Some(composed_id) = composed_id {
            scene_write.update_composed_element_material(composed_id);
            scene_write.update_composed_element_shape(composed_id);
            scene_write.determine_full_bvh_traversal();
        }
        scene_write.update_bvh();
        scene_write.set_dirty(true);
    })));
    ui.add_box(elem_box);
}