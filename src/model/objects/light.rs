use std::fmt::Debug;

use crate::model::{
    materials::Color,
    maths::{hit::Hit, ray::Ray, vec3::Vec3},
    scene::Scene,
};

#[derive(Debug)]
pub struct AmbientLight {
    intensity: f64,
    color: Color,
}

impl AmbientLight {
    // Accessors
    pub fn intensity(&self) -> f64 {
        self.intensity
    }
    pub fn color(&self) -> &Color {
        &self.color
    }

    // Constructor
    pub fn new(intensity: f64, color: Color) -> Self {
        self::AmbientLight { intensity, color }
    }
    pub fn default() -> Self {
        Self {
            intensity: 0.,
            color: Color::new(1., 1., 1.),
        }
    }
}

pub trait Light: Debug + Sync {
    fn get_diffuse(&self, hit: &Hit) -> Color;
    fn get_specular(&self, hit: &Hit, ray: &Ray) -> Color;
    fn is_shadowed(&self, scene: &Scene, hit: &Hit) -> bool;

    fn as_pointlight(&self) -> Option<&PointLight> {
        None
    }
    fn as_parallel_light(&self) -> Option<&ParallelLight> {
        None
    }
    fn as_spot_light(&self) -> Option<&SpotLight> {
        None
    }
}

#[derive(Debug)]
pub struct PointLight {
    pos: Vec3,
    intensity: f64,
    color: Color,
}

impl PointLight {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn intensity(&self) -> f64 {
        self.intensity
    }
    pub fn color(&self) -> &Color {
        &self.color
    }

    // Constructor
    pub fn new(pos: Vec3, intensity: f64, color: Color) -> Self {
        self::PointLight {
            pos,
            intensity,
            color,
        }
    }
}

impl Light for PointLight {
    fn get_diffuse(&self, hit: &Hit) -> Color {
        let to_light = (self.pos() - hit.pos()).normalize();
        let mut ratio = to_light.dot(hit.norm());
        if ratio < 0. {
            return Color::new(0., 0., 0.);
        }
        ratio *= 0_f64
            .max(1. - (self.pos() - hit.pos()).length().powf(2.) / (self.intensity().powf(2.)));
        ratio * self.color()
    }

    fn get_specular(&self, hit: &Hit, ray: &Ray) -> Color {
        let to_light = (self.pos() - hit.pos()).normalize();
        let reflected = (-(&to_light) - hit.norm().dot(&-to_light) * 2. * hit.norm()).normalize();
        let mut ratio = (-ray.get_dir()).normalize().dot(&reflected);
        if ratio < 0. {
            return Color::new(0., 0., 0.);
        }
        ratio = ratio.powf(25.);
        ratio *= 0_f64
            .max(1. - (self.pos() - hit.pos()).length().powf(2.) / (self.intensity().powf(2.)));
        ratio * self.color()
    }

    fn is_shadowed(&self, scene: &Scene, hit: &Hit) -> bool {
        let to_light = (self.pos() - hit.pos()).normalize();
        let shadow_ray = Ray::new(hit.pos() + hit.norm() * 0.001, to_light, 0);
        for element in scene.elements() {
            if let Some(t) = element.shape().intersect(&shadow_ray) {
                if t[0] < (self.pos() - hit.pos()).length() {
                    return true;
                }
            }
        }
        false
    }

    fn as_pointlight(&self) -> Option<&PointLight> {
        Some(self)
    }
}

#[derive(Debug)]
pub struct ParallelLight {
    dir: Vec3,
    intensity: f64,
    color: Color,
}

impl ParallelLight {
    // Accessors
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn intensity(&self) -> f64 {
        self.intensity
    }
    pub fn color(&self) -> &Color {
        &self.color
    }

    // Constructor
    pub fn new(dir: Vec3, intensity: f64, color: Color) -> Self {
        self::ParallelLight {
            dir,
            intensity,
            color,
        }
    }
}

impl Light for ParallelLight {
    fn get_diffuse(&self, hit: &Hit) -> Color {
        let mut ratio = (-self.dir()).dot(hit.norm());
        if ratio < 0. {
            return Color::new(0., 0., 0.);
        }
        ratio *= 0_f64.max(self.intensity());
        (ratio * self.color()).clamp(0., 1.)
    }

    fn get_specular(&self, hit: &Hit, ray: &Ray) -> Color {
        let to_light = -self.dir();
        let reflected = (-(&to_light) - hit.norm().dot(&-to_light) * 2. * hit.norm()).normalize();
        let mut ratio = (-ray.get_dir()).normalize().dot(&reflected);
        if ratio < 0. {
            return Color::new(0., 0., 0.);
        }
        ratio = ratio.powf(50.);
        ratio *= self.intensity().powi(2);
        (ratio * self.color()).clamp(0., 1.)
    }

    fn is_shadowed(&self, scene: &Scene, hit: &Hit) -> bool {
        let shadow_ray = Ray::new(hit.pos() + hit.norm() * 0.001, -self.dir(), 0);
        for element in scene.elements() {
            if let Some(t) = element.shape().intersect(&shadow_ray) {
                if t[0] > 0. {
                    return true;
                }
            }
        }
        false
    }

    fn as_parallel_light(&self) -> Option<&ParallelLight> {
        Some(self)
    }
}

#[derive(Debug)]
pub struct SpotLight {
    pos: Vec3,
    dir: Vec3,
    intensity: f64,
    color: Color,
    fov: f64,
}

impl SpotLight {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn intensity(&self) -> f64 {
        self.intensity
    }
    pub fn color(&self) -> &Color {
        &self.color
    }
    pub fn fov(&self) -> f64 {
        self.fov
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, intensity: f64, color: Color, fov: f64) -> Self {
        self::SpotLight {
            pos,
            dir,
            intensity,
            color,
            fov,
        }
    }
}

impl Light for SpotLight {
    fn get_diffuse(&self, hit: &Hit) -> Color {
        let to_light = (self.pos() - hit.pos()).normalize();
        let angle = self.dir().dot(&-&to_light).acos();
        if angle > self.fov() / 2. {
            return Color::new(0., 0., 0.);
        }
        let mut ratio = to_light.dot(hit.norm());
        ratio *= 0_f64
            .max(1. - (self.pos() - hit.pos()).length().powf(2.) / (self.intensity().powf(2.)));
        if ratio < 0. {
            return Color::new(0., 0., 0.);
        }
        ratio *= 1. - angle / (self.fov() / 2.);
        ratio * self.color()
    }

    fn get_specular(&self, hit: &Hit, ray: &Ray) -> Color {
        let to_light = (self.pos() - hit.pos()).normalize();
        let angle = self.dir().dot(&-&to_light).acos();
        if angle > self.fov() / 2. {
            return Color::new(0., 0., 0.);
        }
        let reflected = (-(&to_light) - hit.norm().dot(&-to_light) * 2. * hit.norm()).normalize();
        let mut ratio = (-ray.get_dir()).normalize().dot(&reflected);
        ratio *= 1. - angle / (self.fov() / 2.);
        if ratio < 0. {
            return Color::new(0., 0., 0.);
        }
        ratio = ratio.powf(25.);
        ratio *= 0_f64
            .max(1. - (self.pos() - hit.pos()).length().powf(2.) / (self.intensity().powf(2.)));
        ratio * self.color()
    }

    fn is_shadowed(&self, scene: &Scene, hit: &Hit) -> bool {
        let to_light = (self.pos() - hit.pos()).normalize();
        let shadow_ray = Ray::new(hit.pos() + hit.norm() * 0.001, to_light, 0);
        for element in scene.elements() {
            if let Some(t) = element.shape().intersect(&shadow_ray) {
                if t[0] < (self.pos() - hit.pos()).length() {
                    return true;
                }
            }
        }
        false
    }

    fn as_spot_light(&self) -> Option<&SpotLight> {
        Some(self)
    }
}
