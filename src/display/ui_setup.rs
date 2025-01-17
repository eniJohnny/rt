use std::sync::{Arc, RwLock};
use crate::{
    model::{element::Element, scene::Scene}, render::common::start_threads, ui::{
        prefabs::material_ui::get_material_ui, ui::UI, ui_setup::scene_ui::setup_scene_toolbar, uibox::{BoxPosition, UIBox}, uielement::{Category, UIElement}, utils::{misc::ElemType, ui_utils::UIContext, Displayable}
    }, ELEMENT, SETTINGS
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

// pub fn setup_toolbar(ui: &mut UI, _context: &UIContext) {
//     let exclusive_uis = [SETTINGS, UISETTINGS];

//     let mut toolbar_box = UIBox::new(TOOLBAR, BoxPosition::TopLeft(0, 0), SCREEN_WIDTH_U32, ui.uisettings());
//     let toolbar_style =StyleBuilder::from_existing(&toolbar_box.style, ui.uisettings())
//         .bg_color(None)
//         .border_size(0)
//         .build();
//     toolbar_box.set_style(toolbar_style);
//     let mut row = UIElement::new("", "row", ElemType::Row(vec![]), ui.uisettings());
//     row.style_mut().bg_color = None;

    // let btn_uisettings = UIElement::new("UI Settings", UISETTINGS, ElemType::Button(Some(Box::new(
    //     move |elem, context, ui| {
    //         if let Some(elem) = elem {
    //             if let Some(_) = ui.get_box(UISETTINGS) {
    //                 ui.destroy_box(UISETTINGS);
    //                 elem.set_style(StyleBuilder::from_existing(&elem.style, ui.uisettings())
    //                     .bg_color(Some(Rgba([200, 200, 200, 255])))
    //                     .build()
    //                 );
    //             } else {
    //                 for uibox_ref in exclusive_uis.clone() {
    //                     if let Some(_) = ui.get_box(uibox_ref) {
    //                         ui.destroy_box(uibox_ref);
    //                     }
    //                 }
    //                 setup_uisettings(ui, context);
    //                 elem.set_style(StyleBuilder::from_existing(&elem.style, ui.uisettings())
    //                     .bg_color(Some(Rgba([100, 100, 100, 255])))
    //                     .build()
    //                 );
    //             }
    //             if let Some(uibox) = ui.get_box_mut(TOOLBAR) {
    //                 let row = uibox.elems.get_mut(0).unwrap();
    //                 if let ElemType::Row(elems) = &mut row.elem_type {
    //                     for elem in elems {
    //                         elem.style_mut().bg_color = Some(Rgba([200, 200, 200, 255]));
    //                     }
    //                 }
    //             }
    //         }
    // }))), ui.uisettings());

	// let btn_scene_settings = UIElement::new("Scene Settings", SCENE_SETTINGS, ElemType::Button(Some(Box::new(
	// 	move |elem, scene, ui| {
    //         if let Some(elem) = elem {
    //             if let Some(_) = ui.get_box(SCENE_SETTINGS) {
    //                 ui.destroy_box(SCENE_SETTINGS);
    //                 elem.set_style(StyleBuilder::from_existing(&elem.style, ui.uisettings())
    //                     .bg_color(Some(Rgba([200, 200, 200, 255])))
    //                     .build()
    //                 );
    //             } else {
    //                 for uibox_ref in exclusive_uis.clone() {
    //                     if let Some(_) = ui.get_box(uibox_ref) {
    //                         ui.destroy_box(uibox_ref);
    //                     }
    //                 }
	// 				setup_scene_settings(ui, scene);
	// 				elem.set_style(StyleBuilder::from_existing(&elem.style, ui.uisettings())
    //                     .bg_color(Some(Rgba([100, 100, 100, 255])))
    //                     .build()
    //                 );
    //             }
    //             if let Some(uibox) = ui.get_box_mut(TOOLBAR) {
    //                 let row = uibox.elems.get_mut(0).unwrap();
    //                 if let ElemType::Row(elems) = &mut row.elem_type {
    //                     for elem in elems {
    //                         elem.style_mut().bg_color = Some(Rgba([200, 200, 200, 255]));
    //                     }
    //                 }
    //             }
	// 		}
	// 	}
	// ))), ui.uisettings());


//     row.add_element(btn_uisettings);
//     toolbar_box.add_elements(vec![row]);
//     ui.add_box(toolbar_box);
// }
