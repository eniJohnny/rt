use super::{triangle::Triangle, ComposedShape};
use std::f64::consts::PI;
use crate::{model::{
    materials::{
        diffuse::Diffuse,
        material::Material,
        texture::{Texture, TextureType}
    },
    maths::vec3::Vec3,
    Element
}, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}};

#[derive(Debug)]
pub struct Mobius {
    pub pos: Vec3,
    pub radius: f64,
    pub half_width: f64,
    pub color: Vec3,
    pub material: Box<dyn Material + Sync>,
    pub elements: Vec<Element>,
}

impl ComposedShape for Mobius {
    fn material(&self) -> &dyn Material {
        return self.material.as_ref();
    }
    fn elements(&self) -> &Vec<Element> {
        return &self.elements();
    }
    fn elements_as_mut(&mut self) -> &mut Vec<Element> {
        return &mut self.elements;
    }
    fn as_mobius(&self) -> Option<&self::Mobius> {
        return Some(self);
    }
    fn as_mobius_mut(&mut self) -> Option<&mut self::Mobius> {
        return Some(self);
    }

    fn get_ui(&self, element: &crate::model::ComposedElement, ui: &mut crate::ui::ui::UI, _scene: &std::sync::Arc<std::sync::RwLock<crate::model::scene::Scene>>) -> crate::ui::uielement::UIElement {
        let mut category = UIElement::new("Mobius", "mobius", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(mobius) = self.as_mobius() {
            let id = element.id();

            // pos
            category.add_element(get_vector_ui(mobius.pos.clone(), "Position", "pos", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(mobius) = elem.composed_shape_mut().as_mobius_mut() {
                    if let Value::Float(value) = value {
                        mobius.pos.set_x(value);
                        elem.update();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(mobius) = elem.composed_shape_mut().as_mobius_mut() {
                    if let Value::Float(value) = value {
                        mobius.pos.set_y(value);
                        elem.update();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(mobius) = elem.composed_shape_mut().as_mobius_mut() {
                    if let Value::Float(value) = value {
                        mobius.pos.set_z(value);
                        elem.update();
                    }
                }
            }),
            false, None, None));

            // radius
            category.add_element(UIElement::new(
                "Radius",
                "radius", 
                ElemType::Property(Property::new(
                    Value::Float(mobius.radius), 
                    Box::new(move |_, value, scene, _: &mut UI| {
                        let next_id = scene.read().unwrap().get_next_element_id();
                        let mut id_increment = 0;
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(mobius) = elem.composed_shape_mut().as_mobius_mut() {
                            if let Value::Float(value) = value {
                                let m = mobius.elements().len() as u32;
                                mobius.set_radius(value, next_id);
                                let n = mobius.elements().len() as u32;
                                id_increment = next_id + n - m;
                            }
                        }
                        scene.set_next_element_id(id_increment);
                        scene.set_dirty(true);
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));

            // half_width
            category.add_element(UIElement::new(
                "Half Width",
                "half_width", 
                ElemType::Property(Property::new(
                    Value::Float(mobius.half_width), 
                    Box::new(move |_, value, scene, _: &mut UI| {
                        let next_id = scene.read().unwrap().get_next_element_id();
                        let mut id_increment = 0;
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(mobius) = elem.composed_shape_mut().as_mobius_mut() {
                            if let Value::Float(value) = value {
                                let m = mobius.elements().len() as u32;
                                mobius.set_half_width(value, next_id);
                                let n = mobius.elements().len() as u32;
                                id_increment = next_id + n - m;
                            }
                        }
                        scene.set_next_element_id(id_increment);
                        scene.set_dirty(true);
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));

            // color
            category.add_element(get_vector_ui(mobius.color.clone(), "Color", "color", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(mobius) = elem.composed_shape_mut().as_mobius_mut() {
                    if let Value::Float(value) = value {
                        mobius.color.set_x(value);
                        mobius.update(0);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(mobius) = elem.composed_shape_mut().as_mobius_mut() {
                    if let Value::Float(value) = value {
                        mobius.color.set_y(value);
                        mobius.update(0);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(mobius) = elem.composed_shape_mut().as_mobius_mut() {
                    if let Value::Float(value) = value {
                        mobius.color.set_z(value);
                        mobius.update(0);
                    }
                }
            }),
            false, None, None));
        }
        category
    }

    fn update(&mut self) {
        self.update(0);
    }
}

impl Mobius {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn half_width(&self) -> f64 {
        self.half_width
    }
    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }
    pub fn color(&self) -> &Vec3 {
        &self.color
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.update(0);
    }
    pub fn set_radius(&mut self, radius: f64, next_id: u32) {
        self.radius = radius;
        self.update(next_id);
    }
    pub fn set_half_width(&mut self, half_width: f64, next_id: u32) {
        self.half_width = half_width;
        self.update(next_id);
    }
    pub fn set_elements(&mut self, elements: Vec<Element>) {
        self.elements = elements;
        self.update(0);
    }
    pub fn set_color(&mut self, color: Vec3) {
        self.color = color;
        self.material.set_color(Texture::Value(color, TextureType::Color));
        self.update(0);
    }

    // Constructor
    pub fn new(pos: Vec3, radius: f64, half_width: f64, color: Vec3) -> Mobius {
        let mut elements: Vec<Element> = Vec::new();
        let mut material: Box<Diffuse> = Diffuse::default();
        material.set_color(Texture::Value(color, TextureType::Color));
        material.set_opacity(Texture::Value(Vec3::from_value(1.0), TextureType::Float));

        let step = 0.1;
        let mut v = 0.0;
        while v < PI {
            v = v.min(PI);
            let mut t = -half_width;

            while t < half_width {
                t = t.min(half_width);

                let p1 = compute_position(v, t, pos, radius);
                let p2 = compute_position(v + step, t, pos, radius);
                let p3 = compute_position(v, t + step, pos, radius);
                let p4 = compute_position(v + step, t + step, pos, radius);

                let triangle1 = Triangle::new(p1, p2, p3);
                let triangle2 = Triangle::new(p3, p2, p4);

                let element1 = Element::new(Box::new(triangle1), material.clone());
                let element2 = Element::new(Box::new(triangle2), material.clone());

                elements.push(element1);
                elements.push(element2);

                t += step;
            }

            v += step;
        }

        Mobius {
            pos,
            radius,
            half_width,
            color,
            material,
            elements,
        }
    }

    // Methods
    pub fn update(&mut self, next_id: u32) {
        let mut next_id = next_id;
        let mut elem_ids: Vec<u32> = Vec::new();
        for elem in self.elements() {
            elem_ids.push(elem.id());
        }

        let pos = self.pos;
        let radius = self.radius;
        let half_width = self.half_width;
        let color = self.color;

        *self = Mobius::new(pos, radius, half_width, color);

        for (i, elem) in self.elements.iter_mut().enumerate() {
            if i < elem_ids.len() {
                elem.set_id(elem_ids[i]);
            } else {
                elem.set_id(next_id);
                next_id += 1;
            }
        }
    }
}

fn compute_position(v: f64, t: f64, pos: Vec3, radius: f64) -> Vec3 {
    let cdv = (v * 2.0).cos();
    let sdv = (v * 2.0).sin();
    let ctv = v.cos();
    let stv = v.sin();
    let c = radius + t * ctv;

    Vec3::new(
        c * cdv,
        c * sdv,
        t * stv,
    ) + pos
}