use std::sync::{Arc, RwLock};
use crate::{
    model::{scene::Scene, Element},
    ui::{ui::UI, uielement::UIElement}
};

pub trait ShapeUI {
    fn get_shape_ui(element: &Element, ui: &mut UI, scene: &Arc<RwLock<Scene>>) -> UIElement;
}