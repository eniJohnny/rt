use std::{fmt::Debug, sync::{Arc, RwLock}};

use crate::{model::{composed_element::ComposedElement, element::Element, materials::material::Material, scene::Scene}, ui::{ui::UI, uielement::UIElement}};

use super::{brick::Brick, helix::Helix, mobius::Mobius, nagone::Nagone, obj::Obj, torusphere::Torusphere};


pub trait ComposedShape: Debug + Sync + Send {
    fn generate_elements(&self, material: Box<dyn Material + Send +Sync>) -> Vec<Element>;

    fn as_torusphere(&self) -> Option<&Torusphere> { None }
    fn as_helix(&self) -> Option<&Helix> { None }
    fn as_brick(&self) -> Option<&Brick> { None }
    fn as_nagone(&self) -> Option<&Nagone> { None }
    fn as_mobius(&self) -> Option<&Mobius> { None }
    fn as_obj(&self) -> Option<&Obj> { None }

    fn as_torusphere_mut(&mut self) -> Option<&mut Torusphere> { None }
    fn as_helix_mut(&mut self) -> Option<&mut Helix> { None }
    fn as_brick_mut(&mut self) -> Option<&mut Brick> { None }
    fn as_nagone_mut(&mut self) -> Option<&mut Nagone> { None }
    fn as_mobius_mut(&mut self) -> Option<&mut Mobius> { None }
    fn as_obj_mut(&mut self) -> Option<&mut Obj> { None }

    fn get_ui(&self, element: &ComposedElement, ui: &mut UI, scene: &Arc<RwLock<Scene>>) -> UIElement;
}