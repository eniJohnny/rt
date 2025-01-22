use std::{fmt::Debug, sync::{Arc, RwLock}};
use crate::{model::{materials::color::Color, maths::{hit::Hit, ray::Ray}, scene::Scene}, ui::{ui::UI, uielement::UIElement}};
use super::{parallel_light::ParallelLight, point_light::PointLight, spot_light::SpotLight};

#[derive(Debug)]
pub struct AnyLight {
    id: usize,
    light: Box<dyn Light + Sync + Send>
}

impl AnyLight {
    pub fn new(light: Box<dyn Light + Sync + Send>) -> Self {
        Self {
            id: 0,
            light
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn light(&self) -> &Box<dyn Light + Sync + Send> {
        &self.light
    }
    pub fn light_mut(&mut self) -> &mut Box<dyn Light + Sync + Send> {
        &mut self.light
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn get_ui(&self, light: &AnyLight, ui: &mut UI, scene: &Arc<RwLock<Scene>>) -> UIElement {
        self.light().get_ui(light, ui, scene)
    }
}

pub trait Light: Debug + Sync {
    fn get_diffuse(&self, hit: &Hit) -> Color;
    fn get_specular(&self, hit: &Hit, ray: &Ray) -> Color;
    fn is_shadowed(&self, scene: &Scene, hit: &Hit) -> bool;

    fn as_point_light(&self) -> Option<&PointLight> {
        None
    }
    fn as_point_light_mut(&mut self) -> Option<&mut PointLight> {
        None
    }
    fn as_parallel_light(&self) -> Option<&ParallelLight> {
        None
    }
    fn as_parallel_light_mut(&mut self) -> Option<&mut ParallelLight> {
        None
    }
    fn as_spot_light(&self) -> Option<&SpotLight> {
        None
    }
    fn as_spot_light_mut(&mut self) -> Option<&mut SpotLight> {
        None
    }
    fn get_ui(&self, light: &AnyLight, ui: &mut UI, scene: &Arc<RwLock<Scene>>) -> UIElement;
}
