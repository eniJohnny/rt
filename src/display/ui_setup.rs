use std::sync::{Arc, RwLock};

use winit::dpi::Position;

use crate::{model::scene::Scene, render::render_threads::start_render_threads, ui::{elements::Displayable, settings, ui::UI, uibox::{BoxPosition, UIBox}}};

pub fn setup_uisettings(ui: &mut UI, scene: &Arc<RwLock<Scene>>) {
    let mut settings_box = UIBox::new("uisettings".to_string(), BoxPosition::TopRight(50, 50), ui.uisettings().gui_width);
    settings_box.add_elements(ui.uisettings().get_fields(ui.uisettings()));
    settings_box.add_elements(scene.read().unwrap().settings().get_fields(ui.uisettings()));
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