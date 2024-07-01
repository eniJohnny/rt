use rusttype::Font;

use crate::{
    ui::elements::{uielement::Category, utils::{ElemType, Property, Value}}, model::{
        materials::color::Color,
        maths::vec3::Vec3,
        objects::light::{AmbientLight, ParallelLight},
    }, ANTIALIASING, BASE_FONT_SIZE, FIELD_PADDING_X, FIELD_PADDING_Y, GUI_HEIGHT, GUI_WIDTH, INDENT_PADDING, MARGIN, MAX_DEPTH, MAX_ITERATIONS, VIEW_MODE
};

use super::{
    elements::{
        uielement::UIElement, Displayable
    }, ui::UI, uisettings::UISettings
};

#[derive(Debug, Clone)]
pub enum ViewMode {
    Simple(Color, ParallelLight),
    Norm,
    HighDef,
    BVH,
}

#[derive(Debug)]
pub struct Settings {
    pub reflections: bool,
    pub indirect: bool,
    pub iterations: usize,
    pub depth: usize,
    pub anti_alisaing: f64,
    pub view_mode: ViewMode,
}

impl Settings {
    pub fn default() -> Self {
        Self {
            view_mode: ViewMode::Simple(
                Color::new(0.2, 0.2, 0.2),
                ParallelLight::new(Vec3::new(0.5, -0.5, 0.5), 1., Color::new(1., 1., 1.)),
            ),
            reflections: true,
            indirect: true,
            iterations: MAX_ITERATIONS,
            depth: MAX_DEPTH,
            anti_alisaing: ANTIALIASING,
        }
    }
}

impl Displayable for Settings {
    fn get_fields(&self, reference: &String, settings: &UISettings) -> Vec<UIElement> {
        let mut category = Category::default();
        category.elems.push(UIElement::new(
            "Iterations",
            "iterations",
            ElemType::Property(Property::new(
                Value::Unsigned(self.iterations as u32),
                Box::new(|value: Value, scene, ui| {
                    if let Value::Unsigned(value) = value {
                        println!("settings iterations");
                        scene.write().unwrap().settings_mut().iterations = value as usize;
                        scene.write().unwrap().set_dirty(true);
                        println!("set done");
                    }
                }),
                Box::new(|_| Ok(())),
                settings,
            )),
            settings,
        ));
        category.elems.push(UIElement::new(
            "Reflections",
            "chk_reflect",
            ElemType::Property(Property::new(
                Value::Bool(self.reflections),
                Box::new(|value, scene, ui| {
                    if let Value::Bool(value) = value {
                        scene.write().unwrap().settings_mut().reflections = value;
                        scene.write().unwrap().set_dirty(true);
                    }
                }),
                Box::new(|_| Ok(())),
                settings,
            )),
            settings,
        ));

        let mut view_mode_radio = UIElement::new("", "viewmode", ElemType::Row(vec![]), settings);
        let mut simple = UIElement::new(
            "Simple",
            "simple",
            ElemType::Button(Box::new(|scene, ui| {
                scene.settings_mut().view_mode = ViewMode::Simple(
                    Color::new(0.2, 0.2, 0.2),
                    ParallelLight::new(Vec3::new(0.5, -0.5, 0.5), 1., Color::new(1., 1., 1.)),
                );
                scene.set_dirty(true);
            })),
            settings,
        );
        let mut gi = UIElement::new(
            "Global Illumination",
            "gi",
            ElemType::Button(Box::new(|scene, ui| {
                scene.settings_mut().view_mode = ViewMode::HighDef;
                scene.set_dirty(true);
            })),
            settings,
        );
        gi.style_mut().fill_width = true;
        simple.style_mut().fill_width = true;
        view_mode_radio.add_element(gi);
        view_mode_radio.add_element(simple);
        let mut category = UIElement::new(
            "Render settings",
            "settings",
            ElemType::Category(category),
            settings,
        );
        category.add_element(view_mode_radio);

        vec![category]
    }
}
