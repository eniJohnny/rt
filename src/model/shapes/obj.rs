use std::{fs::File, io::{BufRead, BufReader}};

use super::{triangle::Triangle, ComposedShape};
use crate::{model::{
    materials::{
        diffuse::Diffuse,
        material::Material,
        texture::{Texture, TextureType}
    }, maths::vec3::Vec3, Element
}, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}};

#[derive(Debug)]
pub struct Obj {
    pub pos: Vec3,
    pub dir: Vec3,
    pub scale: f64,
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub textures: Vec<Vec3>,
    pub faces: Vec<Vec<Vec3>>,
    pub triangle_count: Vec<usize>,
    pub params_number: usize,
	pub filepath: String,
    pub material: Box<dyn Send + Sync + Material>,
    pub elements: Vec<Element>,
}

impl ComposedShape for Obj {
    fn material(&self) -> &Box<dyn Send + Sync + Material> {
        return &self.material
    }
    fn material_mut(&mut self) -> &mut Box<dyn Send + Sync + Material> {
        return &mut self.material;
    }
    fn elements(&self) -> &Vec<Element> {
        return &self.elements();
    }
    fn elements_as_mut(&mut self) -> &mut Vec<Element> {
        return &mut self.elements;
    }
    fn as_obj(&self) -> Option<&self::Obj> {
        return Some(self);
    }
    fn as_obj_mut(&mut self) -> Option<&mut self::Obj> {
        return Some(self);
    }

    fn get_ui(&self, element: &crate::model::ComposedElement, ui: &mut crate::ui::ui::UI, _scene: &std::sync::Arc<std::sync::RwLock<crate::model::scene::Scene>>) -> crate::ui::uielement::UIElement {
        let mut category = UIElement::new("Obj", "obj", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(obj) = self.as_obj() {
            let id = element.id();

            // pos
            category.add_element(get_vector_ui(obj.pos.clone(), "Position", "pos", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(obj) = elem.composed_shape_mut().as_obj_mut() {
                    if let Value::Float(value) = value {
                        obj.pos.set_x(value);
                        elem.update();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(obj) = elem.composed_shape_mut().as_obj_mut() {
                    if let Value::Float(value) = value {
                        obj.pos.set_y(value);
                        elem.update();
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(obj) = elem.composed_shape_mut().as_obj_mut() {
                    if let Value::Float(value) = value {
                        obj.pos.set_z(value);
                        elem.update();
                    }
                }
            }),
            false, None, None));

            // dir
            category.add_element(get_vector_ui(obj.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(obj) = elem.composed_shape_mut().as_obj_mut() {
                    if let Value::Float(value) = value {
                        obj.dir.set_x(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(obj) = elem.composed_shape_mut().as_obj_mut() {
                    if let Value::Float(value) = value {
                        obj.dir.set_y(value);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(obj) = elem.composed_shape_mut().as_obj_mut() {
                    if let Value::Float(value) = value {
                        obj.dir.set_z(value);
                    }
                }
            }),
            false, None, None));

			// scale
            category.add_element(UIElement::new(
                "Scale",
                "scale", 
                ElemType::Property(Property::new(
                    Value::Float(obj.scale), 
                    Box::new(move |_, value, scene, _: &mut UI| {
                        let mut scene = scene.write().unwrap();
                        let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                        if let Some(obj) = elem.composed_shape_mut().as_obj_mut() {
                            if let Value::Float(value) = value {
                                obj.set_scale(value);
                            }
                        }
                        scene.set_dirty(true);
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));
        }

        return category;
    }

    fn update(&mut self) {
        self.update(0);
    }

    fn update_material(&mut self) {
        self.update_material();
    }
}

impl Obj {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn scale(&self) -> f64 {
        self.scale
    }
    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }
    pub fn elements_mut(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }
    pub fn filepath(&self) -> &String {
        &self.filepath
    }
    pub fn material(&self) -> &Box<dyn Send + Sync + Material> {
        &self.material
    }
    pub fn material_mut(&mut self) -> &mut Box<dyn Send + Sync + Material> {
        &mut self.material
    }
    pub fn triangle_count(&self) -> &Vec<usize> {
        &self.triangle_count
    }
    pub fn vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }
    pub fn normals(&self) -> &Vec<Vec3> {
        &self.normals
    }
    pub fn textures(&self) -> &Vec<Vec3> {
        &self.textures
    }
    pub fn faces(&self) -> &Vec<Vec<Vec3>> {
        &self.faces
    }
    pub fn params_number(&self) -> usize {
        self.params_number
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
    }
    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }
    pub fn set_elements(&mut self, elements: Vec<Element>) {
        self.elements = elements;
    }
    pub fn set_filepath(&mut self, filepath: String) {
        self.filepath = filepath;
    }
    pub fn set_material(&mut self, material: Box<dyn Send + Sync + Material>) {
        self.material = material;
    }
    pub fn set_params_number(&mut self) {
        self.params_number = 1 + (self.textures.len() > 0) as usize + (self.normals.len() > 0) as usize;
    }

    pub fn add_triangle_count(&mut self, triangle_count: usize) {
        self.triangle_count.push(triangle_count);
    }
    pub fn add_vertex(&mut self, vertex: Vec3) {
        self.vertices.push(vertex);
    }
    pub fn add_normal(&mut self, normal: Vec3) {
        self.normals.push(normal);
    }
    pub fn add_texture(&mut self, texture: Vec3) {
        self.textures.push(texture);
    }
    pub fn add_face(&mut self, face: Vec<Vec3>) {
        self.faces.push(face);
    }
    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, scale: f64, filepath: String, material: Box<dyn Send + Sync + Material>) -> Obj {
        Obj {
            vertices: Vec::new(),
            normals: Vec::new(),
            textures: Vec::new(),
            faces: Vec::new(),
            triangle_count: Vec::new(),
            params_number: 1,
            pos: pos,
            dir: dir,
            scale: scale,
            filepath: filepath,
            material: material,
			elements: Vec::new(),
        }
    }

    pub fn rotated_vertex(&mut self, x: f64, y: f64, z: f64) -> Vec3 {
        let point_pos = Vec3::new(x, y, z);
        let default_dir = Vec3::new(0.0, 1.0, 0.0);

        let rotation_axis = default_dir.cross(&self.dir());
        if rotation_axis.length() == 0.0 {
            return point_pos;
        }

        let rotation_angle = (default_dir.dot(&self.dir()) / (default_dir.length() * self.dir().length())).acos();

        let rotated_point = point_pos.rotate_from_axis_angle(rotation_angle, &rotation_axis);
        rotated_point
    }

    pub fn parse_file(&mut self) -> Result<(), std::io::Error> {
        let file = File::open(self.filepath.clone())?;
        let reader = BufReader::new(file);
        let lines = reader.lines();

        for line in lines {
            match line {
                Ok(line) => {
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if tokens.len() == 0 {
                        continue;
                    }

                    match tokens[0] {
                        "v" => {
                            let x = tokens[1].parse::<f64>().unwrap() * self.scale();
                            let y = tokens[2].parse::<f64>().unwrap() * self.scale();
                            let z = tokens[3].parse::<f64>().unwrap() * self.scale();

                            let rotated_coords = self.rotated_vertex(x, y, z) + self.pos();
                            self.add_vertex(rotated_coords);
                        }
                        "vn" => {
                            let x = tokens[1].parse::<f64>().unwrap();
                            let z = tokens[2].parse::<f64>().unwrap();
                            let y = tokens[3].parse::<f64>().unwrap();
                            self.add_normal(Vec3::new(x, y, z));
                        }
                        "vt" => {
                            let texture_vec = match tokens.len() {
                                2 => {
                                    let x = tokens[1].parse::<f64>().unwrap();
                                    Vec3::new(x, 0.0, 0.0)
                                }
                                3 => {
                                    let x = tokens[1].parse::<f64>().unwrap();
                                    let y = tokens[2].parse::<f64>().unwrap();
                                    Vec3::new(x, y, 0.0)
                                }
                                4 => {
                                    let x = tokens[1].parse::<f64>().unwrap();
                                    let y = tokens[2].parse::<f64>().unwrap();
                                    let z = tokens[3].parse::<f64>().unwrap();
                                    Vec3::new(x, y, z)
                                }
                                _ => Vec3::new(0.0, 0.0, 0.0),
                            };
                            
                            self.add_texture(texture_vec);
                        }
                        "f" => {
                            let args_number: usize = tokens[1].split("/").count();
                            let tokens_number = tokens.len();
                            // let index = (tokens_number % 4).min(2);
                            self.add_triangle_count(tokens_number - 3);
                                
                            let mut face: Vec<Vec3> = vec![];
                            for i in 1..tokens_number {
                                let indices: Vec<&str> = tokens[i].split("/").collect();

                                let vertex_index = indices[0].parse::<usize>().unwrap();
                                face.push(self.vertices[vertex_index - 1]);

                                if args_number > 1 {
                                    let texture_index = indices[1].parse::<usize>().unwrap();
                                    face.push(self.textures[texture_index - 1]);
                                }

                                if args_number > 2 {
                                    let normal_index = indices[2].parse::<usize>().unwrap();
                                    face.push(self.normals[normal_index - 1]);
                                }
                            }
                            self.add_face(face);
                        }
                        _ => {}
                    }
                }
                Err(_) => {}
            }
        }
        self.set_params_number();
        return Ok(());
    }

    pub fn update(&mut self, next_id: u32) {
        let mut next_id = next_id;
        let mut elem_ids: Vec<u32> = Vec::new();
        let composed_id = self.id();

        for elem in self.elements() {
            elem_ids.push(elem.id());
        }

        self.update_logic();

        for (i, elem) in self.elements.iter_mut().enumerate() {
            if i < elem_ids.len() {
                elem.set_id(elem_ids[i]);
            } else {
                elem.set_id(next_id);
                next_id += 1;
            }

            if let Some(composed_id) = composed_id {
                elem.set_composed_id(composed_id);
            }
        }
    }

    pub fn update_material(&mut self) {
        let material = self.material.clone();
        for elem in self.elements_as_mut() {
            elem.set_material(material.clone());
        }
    }

    pub fn update_logic(&mut self) {
        let len = self.faces.len();

        for i in 0..len {
            let face = &self.faces[i];
            let mut vertices: Vec<Vec3> = vec![];
            let mut normals: Vec<Vec3> = vec![];
            let mut textures: Vec<Vec3> = vec![];
            let modulo = self.params_number();

            for j in 0..face.len() {
                match j % modulo {
                    0 => vertices.push(face[j]),
                    1 => textures.push(face[j]),
                    2 => normals.push(face[j]),
                    _ => {}
                }
            }        

            for k in 0..self.triangle_count()[i] {
                let material = self.material.clone();
                let (a, b, c) = (vertices[0], vertices[k + 1], vertices[k + 2]);
                let (a_uv, b_uv, c_uv) = (textures[0].xy(), textures[k + 1].xy(), textures[k + 2].xy());

                let mut triangle = Triangle::new(a, b, c);
                triangle.set_is_obj(true);
                triangle.set_a_uv(a_uv);
                triangle.set_b_uv(b_uv);
                triangle.set_c_uv(c_uv);

                let elem = Element::new(Box::new(triangle), material);
                self.add_element(elem);
            }
        }
    }
}
