use super::{cylinder::Cylinder, sphere::Sphere, ComposedShape};
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
pub struct Nagone {
    pub pos: Vec3,
    pub dir: Vec3,
    pub radius: f64,
    pub angles: usize,
    pub color: Vec3,
    pub material: Box<dyn Material>,
    pub elements: Vec<Element>,
}

impl ComposedShape for Nagone {
    fn material(&self) -> &dyn Material {
        return self.material.as_ref();
    }
    fn elements(&self) -> &Vec<Element> {
        return &self.elements();
    }
    fn elements_as_mut(&mut self) -> &mut Vec<Element> {
        return &mut self.elements;
    }
    fn as_nagone(&self) -> Option<&self::Nagone> {
        return Some(self);
    }
    fn as_nagone_mut(&mut self) -> Option<&mut self::Nagone> {
        return Some(self);
    }

    fn get_ui(&self, element: &crate::model::ComposedElement, ui: &mut crate::ui::ui::UI, _scene: &std::sync::Arc<std::sync::RwLock<crate::model::scene::Scene>>) -> crate::ui::uielement::UIElement {
        let mut category = UIElement::new("Nagone", "nagone", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(nagone) = self.as_nagone() {
            let id = element.id();

            // pos
            category.add_element(get_vector_ui(nagone.pos.clone(), "Position", "pos", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.pos.set_x(value);
                        elem.update();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.pos.set_y(value);
                        elem.update();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.pos.set_z(value);
                        elem.update();
                    }
                }
            }),
            false, None, None));

            // dir
            category.add_element(get_vector_ui(nagone.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.dir.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.dir.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.dir.set_z(value);
                    }
                }
            }),
            false, None, None));

            // radius
            category.add_element(UIElement::new(
                "Radius",
                "radius", 
                ElemType::Property(Property::new(
                    Value::Float(nagone.radius), 
                    Box::new(move |_, value, scene, _: &mut UI| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                            if let Value::Float(value) = value {
                                nagone.set_radius(value);
                            }
                        }
                        scene.set_dirty(true);
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));

            // angles
            category.add_element(UIElement::new(
                "Angles",
                "angles", 
                ElemType::Property(Property::new(
                    Value::Unsigned(nagone.angles as u32), 
                    Box::new(move |_, value, scene, _: &mut UI| {
                        let next_id = scene.read().unwrap().get_next_element_id();
                        let mut id_increment = 0;
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                            if let Value::Unsigned(value) = value {
                                nagone.set_angles(value as usize, next_id);
                                id_increment = next_id + value - nagone.angles as u32;
                            }
                        }
                        scene.set_next_element_id(id_increment);
                        scene.set_dirty(true);
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));

            // color
            category.add_element(get_vector_ui(nagone.color.clone(), "Color", "color", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.color.set_x(value);
                        nagone.update(0);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.color.set_y(value);
                        nagone.update(0);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(nagone) = elem.composed_shape_mut().as_nagone_mut() {
                    if let Value::Float(value) = value {
                        nagone.color.set_z(value);
                        nagone.update(0);
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

impl Nagone {
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, angles: usize, color: Vec3) -> Nagone {
        if angles < 3 {
            panic!("Nagone must have at least 3 angles");
        }

        let mut elements: Vec<Element> = Vec::new();
        let mut material: Box<Diffuse> = Diffuse::default();
        
        material.set_color(Texture::Value(color, TextureType::Color));
        material.set_opacity(Texture::Value(Vec3::from_value(1.0), TextureType::Float));

        let dir_y = dir.normalize();
        let dir_x;

        if dir == Vec3::new(0.0, 1.0, 0.0) {
            dir_x = Vec3::new(1.0, 0.0, 0.0);
        } else {
            dir_x = Vec3::new(0.0, 1.0, 0.0);
        }

        let mut origins_dirs: Vec<Vec3> = Vec::new();
        let sphere_radius = radius / angles as f64 * 1.3;
        let cylinder_radius = 0.5 * sphere_radius;

        for i in 1..angles + 1 {
            let factor = (i * 2) as f64;
            origins_dirs.push((PI * factor / angles as f64).sin() * dir_y + (PI * factor / angles as f64).cos() * dir_x);
        }

        for i in 0..angles {
            let sphere = Sphere::new(pos + origins_dirs[i] * radius, dir_y, sphere_radius);
            elements.push(Element::new(Box::new(sphere), material.clone()));

            let next_i = (i + 1) % angles;
            let cylinder_dir = ((pos + origins_dirs[next_i] * radius) - (pos + origins_dirs[i] * radius)).normalize();
            let cylinder_height = ((pos + origins_dirs[next_i] * radius) - (pos + origins_dirs[i] * radius)).length();

            let cylinder = Cylinder::new(pos + origins_dirs[i] * radius, cylinder_dir, cylinder_radius, cylinder_height);
            elements.push(Element::new(Box::new(cylinder), material.clone()));
        }

        Nagone {
            pos,
            dir,
            radius,
            angles,
            color,
            material,
            elements,
        }

    }

    // Accessors
    pub fn pos(&self) -> &Vec3 { &self.pos }
    pub fn dir(&self) -> &Vec3 { &self.dir }
    pub fn radius(&self) -> f64 { self.radius }
    pub fn angles(&self) -> usize { self.angles }
    pub fn color(&self) -> &Vec3 { &self.color }
    pub fn material(&self) -> &dyn Material { self.material.as_ref() }
    pub fn elements(&self) -> &Vec<Element> { &self.elements }

    // Setters
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.update(0);
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
        self.update(0);
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
        self.update(0);
    }
    pub fn set_angles(&mut self, angles: usize, next_id: u32) {
        self.angles = angles;
        self.update(next_id);
    }
    pub fn set_color(&mut self, color: Vec3) {
        self.color = color;
        self.material.set_color(Texture::Value(color, TextureType::Color));
        self.update(0);
    }

    // Methods
    pub fn update(&mut self, next_id: u32) {
        let mut next_id = next_id;
        let mut elem_ids: Vec<u32> = Vec::new();
        for elem in self.elements() {
            elem_ids.push(elem.id());
        }

        let pos = self.pos;
        let dir = self.dir;
        let radius = self.radius;
        let color = self.color;
        let angles = self.angles;

        *self = Nagone::new(pos, dir, radius, angles, color);

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