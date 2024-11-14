use std::sync::{Arc, RwLock};
use image::Rgba;
use crate::{
    model::{scene::Scene, Element},
    render::render_threads::start_render_threads,
    ui::{
        prefabs::material_ui::get_material_ui,
        ui::UI,
        uibox::{BoxPosition, UIBox},
        uielement::{Category, UIElement},
        utils::{misc::ElemType, style::StyleBuilder, Displayable}
    },
    ELEMENT, SCREEN_WIDTH_U32, SETTINGS, TOOLBAR, UISETTINGS
    };

pub fn setup_uisettings(ui: &mut UI, _scene: &Arc<RwLock<Scene>>) {
    let mut settings_box = UIBox::new(UISETTINGS, BoxPosition::CenterLeft(10), ui.uisettings().gui_width, ui.uisettings());
    settings_box.add_elements(ui.uisettings().get_fields("UI settings", ui.uisettings()));
    // settings_box.add_elements(get_texture_ui("Color", scene.read().unwrap().elements()[0].material().color(), Box::new(
    //     |value: Texture, scene: &Arc<RwLock<Scene>>| {
    //         scene.write().unwrap().elements_as_mut()[0].material_mut().set_color(value);
    // }), ui.uisettings()));
    settings_box.set_edit_bar(ui.uisettings(), None);

    ui.add_box(settings_box);
}

pub fn setup_settings(ui: &mut UI, scene: &Arc<RwLock<Scene>>) {
    let mut settings_box = UIBox::new(SETTINGS, BoxPosition::CenterLeft(10), ui.uisettings().gui_width, ui.uisettings());
    settings_box.add_elements(scene.read().unwrap().settings().get_fields("Render settings", ui.uisettings()));
    settings_box.set_edit_bar(ui.uisettings(), None);
    ui.add_box(settings_box);
}

pub fn setup_ui(scene: &Arc<RwLock<Scene>>) -> UI {
    let (ra, tb) = start_render_threads(Arc::clone(&scene));
    tb.send(true).unwrap();
    let mut ui = UI::default(ra, tb);
    setup_toolbar(&mut ui, scene);
    ui
}

pub fn setup_element_ui(element: &Element, ui: &mut UI, scene: &Arc<RwLock<Scene>>) {
    ui.destroy_box(ELEMENT);
    let name = "Element".to_string() + &element.id().to_string();
    let mut elem_box = UIBox::new(ELEMENT, BoxPosition::CenterRight(10), ui.uisettings().gui_width, ui.uisettings());
    let mut category = UIElement::new(&name, &name, ElemType::Category(Category::default()), ui.uisettings());

    category.on_click = Some(Box::new(move |_element,_scene, ui| {
        ui.destroy_box(ELEMENT);
    }));
    category.add_element(element.shape().get_ui(element, ui, scene));
    category.add_element(get_material_ui(element, ui, scene));
    elem_box.add_elements(vec![category]);
    elem_box.set_edit_bar(ui.uisettings(), Some(Box::new(|_, scene, _| {
        scene.write().unwrap().set_dirty(true);
    })));
    ui.add_box(elem_box);
}

pub fn setup_toolbar(ui: &mut UI, _scene: &Arc<RwLock<Scene>>) {
    let exclusive_uis = [SETTINGS, UISETTINGS];

    let mut toolbar_box = UIBox::new(TOOLBAR, BoxPosition::TopLeft(0, 0), SCREEN_WIDTH_U32, ui.uisettings());
    let toolbar_style =StyleBuilder::from_existing(&toolbar_box.style, ui.uisettings())
        .bg_color(None)
        .border_size(0)
        .build();
    toolbar_box.set_style(toolbar_style);
    let mut row = UIElement::new("", "row", ElemType::Row(vec![]), ui.uisettings());
    row.style_mut().bg_color = None;

    let btn_uisettings = UIElement::new("UI Settings", UISETTINGS, ElemType::Button(Some(Box::new(
        move |elem, scene, ui| {
            if let Some(elem) = elem {
                if let Some(_) = ui.get_box(UISETTINGS) {
                    ui.destroy_box(UISETTINGS);
                    elem.set_style(StyleBuilder::from_existing(&elem.style, ui.uisettings())
                        .bg_color(Some(Rgba([200, 200, 200, 255])))
                        .build()
                    );
                } else {
                    for uibox_ref in exclusive_uis.clone() {
                        if let Some(_) = ui.get_box(uibox_ref) {
                            ui.destroy_box(uibox_ref);
                        }
                    }
                    setup_uisettings(ui, scene);
                    elem.set_style(StyleBuilder::from_existing(&elem.style, ui.uisettings())
                        .bg_color(Some(Rgba([100, 100, 100, 255])))
                        .build()
                    );
                }
                if let Some(uibox) = ui.get_box_mut(TOOLBAR) {
                    let row = uibox.elems.get_mut(0).unwrap();
                    if let ElemType::Row(elems) = &mut row.elem_type {
                        for elem in elems {
                            elem.style_mut().bg_color = Some(Rgba([200, 200, 200, 255]));
                        }
                    }
                }
            }
    }))), ui.uisettings());

    let btn_settings = UIElement::new("Render Settings", SETTINGS, ElemType::Button(Some(Box::new(
        move |elem, scene, ui| {
            if let Some(elem) = elem {
                if let Some(_) = ui.get_box(SETTINGS) {
                    ui.destroy_box(SETTINGS);
                    elem.set_style(StyleBuilder::from_existing(&elem.style, ui.uisettings())
                        .bg_color(Some(Rgba([200, 200, 200, 255])))
                        .build()
                    );
                } else {
                    for uibox_ref in exclusive_uis.clone() {
                        if let Some(_) = ui.get_box(uibox_ref) {
                            ui.destroy_box(uibox_ref);
                        }
                    }
                    setup_settings(ui, scene);
                    elem.set_style(StyleBuilder::from_existing(&elem.style, ui.uisettings())
                        .bg_color(Some(Rgba([100, 100, 100, 255])))
                        .build()
                    );
                }
                if let Some(uibox) = ui.get_box_mut(TOOLBAR) {
                    let row = uibox.elems.get_mut(0).unwrap();
                    if let ElemType::Row(elems) = &mut row.elem_type {
                        for elem in elems {
                            elem.style_mut().bg_color = Some(Rgba([200, 200, 200, 255]));
                        }
                    }
                }
            }
    }))), ui.uisettings());


    row.add_element(btn_uisettings);
    row.add_element(btn_settings);
    toolbar_box.add_elements(vec![row]);
    ui.add_box(toolbar_box);
}