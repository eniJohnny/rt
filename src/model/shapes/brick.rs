use super::{rectangle::Rectangle, ComposedShape};
use crate::{model::{
    materials::{
        diffuse::Diffuse,
        material::Material,
        texture::{Texture, TextureType}
    },
    maths::vec3::Vec3,
    Element
}, ui::{prefabs::vector_ui::get_vector_ui, uielement::{Category, UIElement}, utils::misc::{ElemType, Value}}};

#[derive(Debug)]
pub struct Brick {
    pub pos: Vec3,
    pub dir: Vec3,
    pub dimensions: Vec3,
    pub color: Vec3,
    pub material: Box<dyn Material>,
    pub elements: Vec<Element>,
}

impl ComposedShape for Brick {
    fn material(&self) -> &dyn Material {
        return self.material.as_ref();
    }
    fn elements(&self) -> &Vec<Element> {
        return &self.elements();
    }
    fn elements_as_mut(&mut self) -> &mut Vec<Element> {
        return &mut self.elements;
    }
    fn as_brick(&self) -> Option<&self::Brick> {
        return Some(self);
    }
    fn as_brick_mut(&mut self) -> Option<&mut self::Brick> {
        return Some(self);
    }

    fn get_ui(&self, element: &crate::model::ComposedElement, ui: &mut crate::ui::ui::UI, _scene: &std::sync::Arc<std::sync::RwLock<crate::model::scene::Scene>>) -> crate::ui::uielement::UIElement {
        let mut category = UIElement::new("Brick", "brick", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(brick) = self.as_brick() {
            let id = element.id();

            // pos
            category.add_element(get_vector_ui(brick.pos.clone(), "Position", "pos", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.pos.set_x(value);
                        elem.update();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.pos.set_y(value);
                        elem.update();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.pos.set_z(value);
                        elem.update();
                    }
                }
            }),
            false, None, None));

            // dir
            category.add_element(get_vector_ui(brick.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.dir.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.dir.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.dir.set_z(value);
                    }
                }
            }),
            false, None, None));

            // dimensions
            category.add_element(get_vector_ui(brick.dimensions.clone(), "Dimensions", "dimensions", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.dimensions.set_x(value);
                        elem.update();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.dimensions.set_y(value);
                        elem.update();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(brick) = elem.composed_shape_mut().as_brick_mut() {
                    if let Value::Float(value) = value {
                        brick.dimensions.set_z(value);
                        elem.update();
                    }
                }
            }),
            false, None, None));
        }

        return category;
    }

    fn update(&mut self) {
        self.update();
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
    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }
    pub fn color(&self) -> &Vec3 {
        &self.color
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
    pub fn set_elements(&mut self, elements: Vec<Element>) {
        self.elements = elements;
    }
    pub fn set_color(&mut self, color: Vec3) {
        self.color = color;
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, dimensions: Vec3, color: Vec3) -> Brick {
        let mut elements: Vec<Element> = Vec::new();
        let mut material: Box<Diffuse> = Diffuse::default();
        material.set_color(Texture::Value(color, TextureType::Color));
        material.set_opacity(Texture::Value(Vec3::from_value(1.0), TextureType::Float));

        let h = *dimensions.x();
        let w = *dimensions.y();
        let l = *dimensions.z();

        let dir_h = dir.clone().normalize();
        let dir_w;
        if dir_h == Vec3::new(0.0, 1.0, 0.0) {
            dir_w = dir_h.cross(&Vec3::new(0.0, 0.0, 1.0)).normalize();
        } else {
            dir_w = dir_h.cross(&Vec3::new(0.0, 1.0, 0.0)).normalize();
        }
        let dir_l = dir_h.cross(&-dir_w).normalize();


        // Create the 6 rectangles of the brick in this order:
        // TOP, BOTTOM, FRONT, BACK, LEFT, RIGHT

        // The positions of the 6 rectangles (center)
        let rectangle_positions = [
            pos.clone() + dir_h.clone() * h / 2.,
            pos.clone() - dir_h.clone() * h / 2.,
            pos.clone() + dir_l.clone() * l / 2.,
            pos.clone() - dir_l.clone() * l / 2.,
            pos.clone() + dir_w.clone() * w / 2.,
            pos.clone() - dir_w.clone() * w / 2.,
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
            );

            material.set_color(Texture::Value(color, TextureType::Color));
            elements.push(Element::new(Box::new(rectangle), material.clone()));
        }

        // Create and return the brick
        Brick {
            pos,
            dir,
            dimensions,
            color,
            material,
            elements,
        }
    }

    pub fn update(&mut self) {
        let mut elem_ids: Vec<u32> = Vec::new();
        for elem in self.elements() {
            elem_ids.push(elem.id());
        }

        *self = Brick::new(self.pos.clone(), self.dir.clone(), self.dimensions.clone(), self.color.clone());

        for (i, elem) in self.elements.iter_mut().enumerate() {
            elem.set_id(elem_ids[i]);
        }
    }
}
