use std::{f64::consts::PI, fs::File, io::{BufRead, BufReader}, sync::{Arc, RwLock}};

use super::{triangle::Triangle, composed_shape::ComposedShape};
use crate::{model::{
    composed_element::ComposedElement, element::Element, materials::
        material::Material, maths::vec3::Vec3, scene::Scene
}, ui::{prefabs::vector_ui::get_vector_ui, ui::UI, uielement::{Category, UIElement}, utils::misc::{ElemType, Property, Value}}};

#[derive(Debug)]
pub struct Obj {
    pub pos: Vec3,
    pub dir: Vec3,
    pub rotation: f64,
    pub scale: f64,
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub textures: Vec<Vec3>,
    pub faces: Vec<Vec<Vec3>>,
    pub triangle_count: Vec<usize>,
    pub params_number: usize,
	pub filepath: String
}

impl ComposedShape for Obj {
    fn as_obj(&self) -> Option<&self::Obj> {
        return Some(self);
    }
    fn as_obj_mut(&mut self) -> Option<&mut self::Obj> {
        return Some(self);
    }

    fn generate_elements(&self, material: Box<dyn Material + Send +Sync>) -> Vec<Element> {
        let mut elements = vec![];

        let len = self.faces.len();

        for i in 0..len {
            let face = self.faces[i].clone();
            let mut vertices: Vec<Vec3> = vec![];
            let mut normals: Vec<Vec3> = vec![];
            let mut textures: Vec<Vec3> = vec![];
            let modulo = self.params_number();

            for j in 0..face.len() {
                match j % modulo {
                    0 => {
                        let vec = face[j].clone() * self.scale() + self.pos();
                        vertices.push(self.rotated_vertex(*vec.x(), *vec.y(), *vec.z()));
                    },
                    1 => textures.push(face[j]),
                    2 => {
                        normals.push(self.rotated_vertex(*face[j].x(), *face[j].y(), *face[j].z()))
                    },
                    _ => {}
                }
            }

            // println!("{}", i);
            for k in 0..self.triangle_count()[i] {
                let (a, b, c) = (vertices[0], vertices[k + 1], vertices[k + 2]);
                let mut triangle = Triangle::new(a, b, c);
                triangle.set_is_obj(true);

                if textures.len() > k + 2 {
                    let (a_uv, b_uv, c_uv) = (textures[0].xy(), textures[k + 1].xy(), textures[k + 2].xy());
                    triangle.set_a_uv(a_uv);
                    triangle.set_b_uv(b_uv);
                    triangle.set_c_uv(c_uv);
                }

                let elem = Element::new(Box::new(triangle), material.clone());
                elements.push(elem);
            }
        }
        elements
    }

    fn get_ui(&self, element: &ComposedElement, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
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
                        let newpos = Vec3::new(value, *obj.pos.y(), *obj.pos.z());
                        obj.set_pos(newpos);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(obj) = elem.composed_shape_mut().as_obj_mut() {
                    if let Value::Float(value) = value {
                        let newpos = Vec3::new(*obj.pos.x(), value, *obj.pos.z());
                        obj.set_pos(newpos);
                    }
                }
            }),
            Box::new(move |_, value, scene, _| {
                let mut scene = scene.write().unwrap();
                let elem = scene.composed_element_mut_by_id(id.clone()).unwrap();
                if let Some(obj) = elem.composed_shape_mut().as_obj_mut() {
                    if let Value::Float(value) = value {
                        let newpos = Vec3::new(*obj.pos.x(), *obj.pos.y(), value);
                        obj.set_pos(newpos);
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
    pub fn filepath(&self) -> &String {
        &self.filepath
    }
    pub fn triangle_count(&self) -> &Vec<usize> {
        &self.triangle_count
    }
    pub fn vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }
    pub fn vertices_mut(&mut self) -> &mut Vec<Vec3> {
        &mut self.vertices
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
    pub fn set_filepath(&mut self, filepath: String) {
        self.filepath = filepath;
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

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, rotation: f64, scale: f64, filepath: String) -> Obj {
        Obj {
            vertices: Vec::new(),
            normals: Vec::new(),
            textures: Vec::new(),
            faces: Vec::new(),
            triangle_count: Vec::new(),
            params_number: 1,
            pos,
            dir,
            rotation,
            scale,
            filepath
        }
    }

    pub fn rotated_vertex(&self, x: f64, y: f64, z: f64) -> Vec3 {
        let point_pos = Vec3::new(x, y, z) - self.pos();
        let default_dir = Vec3::new(0.0, 1.0, 0.0);

        let rotation_axis = default_dir.cross(&self.dir());
        let point_with_dir = match rotation_axis.length() == 0.0 {
            true => {
                if default_dir.dot(&self.dir()) < 0.0 {
                    -point_pos
                } else {
                    point_pos
                }
            },
            false => {
                let rotation_angle = (default_dir.dot(&self.dir()) / (default_dir.length() * self.dir().length())).acos();
                point_pos.rotate_from_axis_angle(rotation_angle, &rotation_axis)
            }
        };

        if self.rotation == 0. {
            return point_with_dir + self.pos();
        }
        let point_with_dir_and_rotation = point_with_dir.rotate_from_axis_angle(self.rotation * PI * 2. / 360., &self.dir);
        point_with_dir_and_rotation + self.pos()
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
                            let x = tokens[1].parse::<f64>().unwrap();
                            let y = tokens[2].parse::<f64>().unwrap();
                            let z = tokens[3].parse::<f64>().unwrap();

                            self.add_vertex(Vec3::new(x, y, z));
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
}
