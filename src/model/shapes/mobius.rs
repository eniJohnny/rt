use super::{triangle::Triangle, composed_shape::ComposedShape};
use std::{f64::consts::PI, sync::{Arc, RwLock}};
use crate::{model::{
    composed_element::ComposedElement, element::Element, materials::
        material::Material, maths::vec3::Vec3, scene::Scene
}, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}};

#[derive(Debug)]
pub struct Mobius {
    pub pos: Vec3,
    pub radius: f64,
    pub half_width: f64
}

impl ComposedShape for Mobius {
    fn as_mobius(&self) -> Option<&self::Mobius> {
        return Some(self);
    }
    fn as_mobius_mut(&mut self) -> Option<&mut self::Mobius> {
        return Some(self);
    }

    fn generate_elements(&self, material: Box<dyn Material + Send +Sync>) -> Vec<Element> {
        let mut elements: Vec<Element> = Vec::new();

        let step = 0.1;
        let mut v = 0.0;
        while v < PI {
            v = v.min(PI);
            let mut t = -self.half_width;

            while t < self.half_width {
                t = t.min(self.half_width);

                let p1 = compute_position(v, t, self.pos, self.radius);
                let p2 = compute_position(v + step, t, self.pos, self.radius);
                let p3 = compute_position(v, t + step, self.pos, self.radius);
                let p4 = compute_position(v + step, t + step, self.pos, self.radius);

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
        elements
    }

    fn get_ui(&self, element: &ComposedElement, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
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
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(mobius) = elem.composed_shape_mut().as_mobius_mut() {
                    if let Value::Float(value) = value {
                        mobius.pos.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(mobius) = elem.composed_shape_mut().as_mobius_mut() {
                    if let Value::Float(value) = value {
                        mobius.pos.set_z(value);
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
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(mobius) = elem.composed_shape_mut().as_mobius_mut() {
                            if let Value::Float(value) = value {
                                mobius.set_radius(value);
                            }
                        }
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
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(mobius) = elem.composed_shape_mut().as_mobius_mut() {
                            if let Value::Float(value) = value {
                                mobius.set_half_width(value);
                            }
                        }
                        scene.set_dirty(true);
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));
        }
        category
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

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }
    pub fn set_half_width(&mut self, half_width: f64) {
        self.half_width = half_width;
    }

    // Constructor
    pub fn new(pos: Vec3, radius: f64, half_width: f64) -> Mobius {
        Mobius {
            pos,
            radius,
            half_width
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