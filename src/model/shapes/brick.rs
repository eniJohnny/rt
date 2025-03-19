use std::sync::{Arc, RwLock};

use super::{composed_shape::ComposedShape, rectangle::Rectangle, utils::get_cross_axis};
use crate::{model::{
    materials::
        material::Material
    , maths::vec3::Vec3, scene::Scene, composed_element::ComposedElement, element::Element
}, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Value}}};

#[derive(Debug)]
pub struct Brick {
    pub pos: Vec3,
    pub dir: Vec3,
    pub dimensions: Vec3
}

impl ComposedShape for Brick {
    fn as_brick(&self) -> Option<&self::Brick> {
        return Some(self);
    }
    fn as_brick_mut(&mut self) -> Option<&mut self::Brick> {
        return Some(self);
    }

    fn generate_elements(&self, material: Box<dyn Material + Send +Sync>) -> Vec<Element> {
        let mut elements = vec![];

        let h = *self.dimensions.x();
        let w = *self.dimensions.y();
        let l = *self.dimensions.z();

        let dir_h = self.dir.clone().normalize();
        let dir_w = get_cross_axis(&dir_h);
        let dir_l = dir_h.cross(&-dir_w).normalize();


        // Create the 6 rectangles of the brick in this order:
        // TOP, BOTTOM, FRONT, BACK, LEFT, RIGHT

        // The positions of the 6 rectangles (center)
        let rectangle_positions = [
            self.pos.clone() + dir_h.clone() * h / 2.,
            self.pos.clone() - dir_h.clone() * h / 2.,
            self.pos.clone() + dir_l.clone() * l / 2.,
            self.pos.clone() - dir_l.clone() * l / 2.,
            self.pos.clone() + dir_w.clone() * w / 2.,
            self.pos.clone() - dir_w.clone() * w / 2.,
        ];

        // The directions of the 6 rectangles (length, width)
        let rectangle_dirs = [
            (dir_l, dir_w),
            (dir_l, dir_w),
            (dir_w, dir_h),
            (dir_w, dir_h),
            (dir_l, dir_h),
            (dir_l, dir_h),
        ];

        // The dimensions of the 6 rectangles (length, width)
        let rectangle_dims = [
            (l, w),
            (l, w),
            (w, h),
            (w, h),
            (l, h),
            (l, h),
        ];

        // Create the 6 rectangles and add them to the elements vector
        for i in 0..rectangle_positions.len() {
            let rectangle = Rectangle::new(
                rectangle_positions[i],
                rectangle_dims[i].0,
                rectangle_dims[i].1,
                rectangle_dirs[i].0,
                rectangle_dirs[i].1,
                false
            );

            elements.push(Element::new(Box::new(rectangle), material.clone()));
        }
        elements
    }

    fn get_ui(&self, element: &ComposedElement, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Brick", "brick", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(brick) = self.as_brick() {
            let id = element.id();

            // pos
            category.add_element(get_vector_ui(brick.pos.clone(), "Position", "pos", &ui.uisettings_mut(),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.pos.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.pos.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.pos.set_z(value);
                    }
                }
            }),
            false, None, None));

            // dir
            category.add_element(get_vector_ui(brick.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.dir.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.dir.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.dir.set_z(value);
                        brick.dir = brick.dir.normalize();
                    }
                }
            }),
            false, None, None));

            // dimensions
            category.add_element(get_vector_ui(brick.dimensions.clone(), "Dimensions", "dimensions", &ui.uisettings_mut(),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.dimensions.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.dimensions.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.dimensions.set_z(value);
                    }
                }
            }),
            false, None, None));
        }

        return category;
    }
}

impl Brick {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn dimensions(&self) -> &Vec3 {
        &self.dimensions
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
    }
    pub fn set_dimensions(&mut self, dimensions: Vec3) {
        self.dimensions = dimensions;
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, dimensions: Vec3) -> Brick {
        Brick {
            pos,
            dir,
            dimensions
        }
    }

}
