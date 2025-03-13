use image::Rgba;
use crate::{display::{mainloop::load_scene, ui_setup::{setup_obejcts_ui, setup_settings}}, render::render_thread::UIOrder, ui::{prefabs::file_ui::get_file_box, ui::UI, uibox::{BoxPosition, UIBox}, uielement::UIElement, uisettings::UISettings, utils::{misc::{ElemType, Value}, style::{Style, StyleBuilder}, ui_utils::UIContext}}, ELEMENT, OBJECTS, SCENE_FOLDER, SCENE_TOOLBAR, SCREEN_WIDTH_U32, SETTINGS, TOOLBAR};


pub fn change_scene(context: &mut UIContext, ui: &mut UI, render_id: Option<usize>, element: Option<&mut UIElement>) {
    let uisettings = ui.uisettings().clone();
    
    context.previous_active_scene = context.active_scene;
    context.active_scene = render_id;
    if let Some(id) = render_id {
        context.transmitter.send(UIOrder::AskImage(context.next_scene_id)).unwrap();
        if let Some(element) = element {
            element.set_style(get_selected_scene_tab_style(&uisettings));
        }
        setup_scene_options(ui, context, id.clone());
    } else {
        ui.destroy_box(SCENE_TOOLBAR);
    }
    ui.set_editing(None);
    ui.destroy_box(ELEMENT);
    ui.destroy_box(SETTINGS);


    if let Some(uibox) = ui.get_box_mut(TOOLBAR) {
        let row = uibox.elems.get_mut(0).unwrap();
        if let ElemType::Row(elems) = &mut row.elem_type {
            let mut first = true;
            for elem in elems {
                let mut style = get_unselected_scene_tab_style(&uisettings, first);
                if let Some(render_id) = render_id {
                    if elem.reference == format!("{}.scene_{}", row.reference, render_id) {
                        style = get_selected_scene_tab_style(&uisettings);
                    }
                }
                elem.set_style(style);
                first = false;
            }
        }
    }
    ui.set_dirty();

}

pub fn add_scene_to_ui(ui: &mut UI, _context: &mut UIContext, id: usize, scene_path: &str) {
    let uisettings = ui.uisettings().clone();
    if let Some(row) = ui.get_element_mut(TOOLBAR.to_string() + ".row") {
        let scene_name = scene_path.split("/").last().unwrap_or_default();
        let mut btn_scene = UIElement::new(scene_name, &format!("{}.scene_{}", row.reference.clone(), id), ElemType::Button(Some(Box::new(
            move |element, context, ui| {
                if context.active_scene.is_none() || context.active_scene.unwrap() != id {
                    change_scene(context, ui, Some(id), element);
                }
        }))), &uisettings);
        
        btn_scene.set_style(get_selected_scene_tab_style(&uisettings));
        let scene_btn = row.remove_element(row.nb_elements() - 1);
        row.add_element(btn_scene);
        if let Some(scene_btn) = scene_btn {
            row.add_element(scene_btn);
        }
    }
}

pub fn setup_scene_options(ui: &mut UI, context: &UIContext, render_id: usize) {
    // let exclusive_uis = [SETTINGS, OBJECTS];
    ui.destroy_box(SCENE_TOOLBAR);

    let mut toolbar_box = UIBox::new(SCENE_TOOLBAR, BoxPosition::TopLeft(20, 0), SCREEN_WIDTH_U32, ui.uisettings());
    let toolbar_style =StyleBuilder::from_existing(&toolbar_box.style, ui.uisettings())
        .bg_color(None)
        .border_size(0)
        .build();
    toolbar_box.set_style(toolbar_style);
    let mut row = UIElement::new("", "row", ElemType::Row(vec![]), ui.uisettings());

    let btn_settings = UIElement::new("Scene Settings", SETTINGS, ElemType::Button(Some(Box::new(
        move |elem, scene, ui| {
            if let Some(elem) = elem {
                if let Some(_) = ui.get_box(SETTINGS) {
                    ui.destroy_box(SETTINGS);
                    elem.set_style(StyleBuilder::from_existing(&elem.style, ui.uisettings())
                        .bg_color(Some(Rgba([200, 200, 200, 255])))
                        .build()
                    );
                } else {
                    if let Some(_) = ui.get_box(OBJECTS) {
                        ui.destroy_box(OBJECTS);
                    }
                    
                    let ui_settings = ui.uisettings().clone();
                    if let Some(obj_elem) = ui.get_element_mut(format!("{}.row.{}", SCENE_TOOLBAR, OBJECTS)) {
                        obj_elem.set_style(StyleBuilder::from_existing(&obj_elem.style, &ui_settings)
                            .bg_color(Some(Rgba([200, 200, 200, 255])))
                            .build()
                        );
                    }
                    
                    setup_settings(ui, scene);
                    elem.set_style(StyleBuilder::from_existing(&elem.style, ui.uisettings())
                        .bg_color(Some(Rgba([100, 100, 100, 255])))
                        .build()
                    );
                }
            }
    }))), ui.uisettings());

    let btn_objects = UIElement::new("Objects", OBJECTS, ElemType::Button(Some(Box::new(
        move |elem, scene, ui| {
            if let Some(elem) = elem {
                if let Some(_) = ui.get_box(OBJECTS) {
                    ui.destroy_box(OBJECTS);
                    elem.set_style(StyleBuilder::from_existing(&elem.style, ui.uisettings())
                        .bg_color(Some(Rgba([200, 200, 200, 255])))
                        .build()
                    );
                } else {
                    if let Some(_) = ui.get_box(SETTINGS) {
                        ui.destroy_box(SETTINGS);
                    }

                    let ui_settings = ui.uisettings().clone();
                    if let Some(obj_elem) = ui.get_element_mut(format!("{}.row.{}", SCENE_TOOLBAR, SETTINGS)) {
                        obj_elem.set_style(StyleBuilder::from_existing(&obj_elem.style, &ui_settings)
                            .bg_color(Some(Rgba([200, 200, 200, 255])))
                            .build()
                        );
                    }

                    setup_obejcts_ui(ui, scene);
                    elem.set_style(StyleBuilder::from_existing(&elem.style, ui.uisettings())
                        .bg_color(Some(Rgba([100, 100, 100, 255])))
                        .build()
                    );
                }
            }
    }))), ui.uisettings());

    
    let text = match context.scene_list.get(&render_id).unwrap().read().unwrap().paused() {
        true => "Start",
        false => "Pause"
    };
    
    let btn_pause = UIElement::new(text, "pause", ElemType::Button(Some(Box::new(
        move |elem, context, _| {
            if let Some(elem) = elem {
                let mut scene = context.scene_list.get(&render_id).unwrap().write().unwrap();
            if scene.paused() {
                elem.text = "Pause".to_string();
                context.transmitter.send(UIOrder::SceneStart(render_id)).unwrap();
                scene.set_paused(false);
                
            } else {
                elem.text = "Start".to_string();
                context.transmitter.send(UIOrder::ScenePause(render_id)).unwrap();
                scene.set_paused(true);
            }
        }
    }))), ui.uisettings());
    
    let row_reference = row.reference.clone();
    
    let btn_close = UIElement::new("Close", "close", ElemType::Button(Some(Box::new(
        move |elem, context, ui| {
            if let Some(elem) = elem {
                let id = context.active_scene.unwrap();
                context.scene_list.remove(&id);
                context.transmitter.send(UIOrder::CloseScene(id)).unwrap();
                ui.remove_element_by_reference(format!("{}.{}.scene_{}",TOOLBAR, row_reference, id));
                
                context.active_scene = None;
                change_scene(context,ui, context.previous_active_scene.clone(), None);
                elem.style_mut().visible = false;
            }
        }))), ui.uisettings());
        
    row.add_element(btn_settings);
    row.add_element(btn_objects);
    row.add_element(btn_pause);
    row.add_element(btn_close);

    toolbar_box.add_elements(vec![row]);
    ui.add_box(toolbar_box);
    ui.set_dirty();
}

pub fn setup_scene_toolbar(ui: &mut UI, _context: &UIContext) {
    let mut toolbar_box = UIBox::new(TOOLBAR, BoxPosition::TopLeft(0, 0), SCREEN_WIDTH_U32, ui.uisettings());
    let toolbar_style =StyleBuilder::from_existing(&toolbar_box.style, ui.uisettings())
        .bg_color(None)
        .border_size(0)
        .build();
    toolbar_box.set_style(toolbar_style);
    let mut row = UIElement::new("", "row", ElemType::Row(vec![]), ui.uisettings());
    row.style_mut().bg_color = None;
    row.style_mut().padding_top = 0;
    row.style_mut().padding_left = 0;
    row.style_mut().padding_right = 0;
    row.style_mut().margin = 0;
    let mut btn_open_scene = UIElement::new("New scene", "open_scene", ElemType::Button(Some(Box::new(
        move |_, _, ui| {
            let file_box = get_file_box(SCENE_FOLDER.to_string(), "open_scene_box".to_string(), Box::new(move |_, value, context, ui| {
                if let Value::Text(scene_path) = value {
                    load_scene(scene_path.as_str(), context, ui);
                }
            }), ui.uisettings(), "".to_string());
            let box_reference = file_box.reference.clone();
            ui.add_box(file_box);
            ui.set_active_box(box_reference);
    }))), ui.uisettings());

    btn_open_scene.set_style(get_unselected_scene_tab_style(ui.uisettings(), true));

    row.add_element(btn_open_scene);
    toolbar_box.add_elements(vec![row]);
    ui.add_box(toolbar_box);
}


fn get_selected_scene_tab_style(settings: &UISettings) -> Style {
    StyleBuilder::from_btn(settings)
        .bg_color(Some(Rgba([100, 100, 100, 255])))
        .font_color(Rgba([230, 230, 230, 255]))
        .border_top(0)
        .border_bot(0)
        .border_left(3)
        .border_right(1)
        .border_radius(0)
        .fill_width(true)
        .text_center(true)
        .build()
}

fn get_unselected_scene_tab_style(settings: &UISettings, first: bool) -> Style {
    let mut style = StyleBuilder::from_btn(settings)
    .bg_color(Some(Rgba([50, 50, 50, 255])))
    .font_color(Rgba([150, 150, 150, 255]))
    .border_top(0)
    .border_bot(2)
    .border_left(2)
    .border_right(0)
    .border_radius(0)
    .fill_width(true)
    .text_center(true)
    .build();
    if first {
        style.border_left = 0;
    }
    style
}