use rusttype::Font;

use crate::{
    model::{
        materials::color::Color,
        maths::vec3::Vec3,
        objects::light::{AmbientLight, ParallelLight},
    }, ui::{uielement::{Category, UIElement}, uisettings::UISettings, utils::{misc::{ElemType, Property, Value}, Displayable}}, ANTIALIASING, BASE_FONT_SIZE, FIELD_PADDING_X, FIELD_PADDING_Y, GUI_HEIGHT, GUI_WIDTH, INDENT_PADDING, MARGIN, MAX_DEPTH, MAX_ITERATIONS, VIEW_MODE
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
    fn get_fields(&self, name: &str, settings: &UISettings) -> Vec<UIElement> {
        let mut category = Category::default();
        category.elems.push(UIElement::new(
            "Iterations",
            "iterations",
            ElemType::Property(Property::new(
                Value::Unsigned(self.iterations as u32),
                Box::new(|_, value: Value, scene, ui| {
                    if let Value::Unsigned(value) = value {
                        scene.write().unwrap().settings_mut().iterations = value as usize;
                        scene.write().unwrap().set_dirty(true);
                    }
                }),
                Box::new(|_| Ok(())),
                settings,
            )),
            settings,
        ));
        let chkReflect = UIElement::new(
            "Reflections",
            "chk_reflect",
            ElemType::Property(Property::new(
                Value::Bool(self.reflections),
                Box::new(|_, value, scene, ui| {
                    if let Value::Bool(value) = value {
                        scene.write().unwrap().settings_mut().reflections = value;
                        scene.write().unwrap().set_dirty(true);
                    }
                }),
                Box::new(|_| Ok(())),
                settings,
            )),
            settings,
        );
        let chkIndirect = UIElement::new(
            "Indirect light",
            "chk_indirect",
            ElemType::Property(Property::new(
                Value::Bool(self.indirect),
                Box::new(|_, value, scene, ui| {
                    if let Value::Bool(value) = value {
                        scene.write().unwrap().settings_mut().indirect = value;
                        scene.write().unwrap().set_dirty(true);
                    }
                }),
                Box::new(|_| Ok(())),
                settings,
            )),
            settings,
        );
        let vec = vec![chkIndirect, chkReflect];
        let mut row = UIElement::new("", "row_indirect_reflection", ElemType::Row(vec), settings);
        category.elems.push(row);

        let mut view_mode_radio = UIElement::new("", "viewmode", ElemType::Row(vec![]), settings);
        let mut simple = UIElement::new(
            "Simple",
            "simple",
            ElemType::Button(Some(Box::new(|_, scene, ui| {
                scene.write().unwrap().settings_mut().view_mode = ViewMode::Simple(
                    Color::new(0.2, 0.2, 0.2),
                    ParallelLight::new(Vec3::new(0.5, -0.5, 0.5), 1., Color::new(1., 1., 1.)),
                );
                scene.write().unwrap().set_dirty(true);
            }))),
            settings,
        );
        let mut gi = UIElement::new(
            "Global Illumination",
            "gi",
            ElemType::Button(Some(Box::new(|_, scene, ui| {
                scene.write().unwrap().settings_mut().view_mode = ViewMode::HighDef;
                scene.write().unwrap().set_dirty(true);
            }))),
            settings,
        );
        gi.style_mut().fill_width = true;
        simple.style_mut().fill_width = true;
        view_mode_radio.add_element(gi);
        view_mode_radio.add_element(simple);
        let mut category = UIElement::new(
            name,
            "settings",
            ElemType::Category(category),
            settings,
        );
        category.add_element(view_mode_radio);

        vec![category]
    }
}
