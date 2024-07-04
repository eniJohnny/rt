use std::{path::Path, sync::{Arc, RwLock}, thread::current};

use image::Rgba;
use winit::dpi::Position;

use crate::{model::{materials::texture::{Texture, TextureType}, maths::vec3::Vec3, scene::Scene}, picker::get_files_in_folder, render::render_threads::start_render_threads, ui::{prefabs::{texture_ui::get_texture_ui, vector_ui::get_vector_ui}, ui::UI, uibox::{BoxPosition, UIBox}, uielement::{Category, UIElement}, uisettings::UISettings, utils::{misc::{ElemType, FnSubmitValue, Property, Value}, style::Style, ui_utils::get_parent_ref, Displayable}}};

pub fn setup_uisettings(ui: &mut UI, scene: &Arc<RwLock<Scene>>) {
    let mut settings_box = UIBox::new("uisettings", BoxPosition::TopRight(50, 50), ui.uisettings().gui_width);
    settings_box.add_elements(ui.uisettings().get_fields("UI settings", ui.uisettings()));
    settings_box.add_elements(scene.read().unwrap().settings().get_fields("Render settings", ui.uisettings()));
    settings_box.add_elements(get_texture_ui("Color", scene.read().unwrap().elements()[0].material().color(), Box::new(
        |value: Texture, scene: &Arc<RwLock<Scene>>| {
            scene.write().unwrap().elements_as_mut()[0].material_mut().set_color(value);
    }), ui.uisettings()));
    settings_box.set_edit_bar(ui.uisettings(), None);

    let index = ui.add_box(settings_box);
    ui.set_active_box(index);
}

pub fn setup_ui(scene: &Arc<RwLock<Scene>>) -> UI {
    let (ra, tb) = start_render_threads(Arc::clone(&scene));
    tb.send(true).unwrap();
    let mut ui = UI::default(ra, tb);
    setup_uisettings(&mut ui, scene);
    ui
}