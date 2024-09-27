use crate::model::materials::color::Color;
use crate::model::materials::material::{self, Material};
use crate::model::materials::texture::{Texture, TextureType};
use crate::model::maths::vec3::Vec3;
use crate::model::scene::Scene;
use crate::model::shapes::triangle::Triangle;
use crate::model::{shapes, Element};
use crate::ui::utils::misc::Value;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::vec;

#[derive(Debug)]
pub struct Obj {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub textures: Vec<Vec3>,
    pub faces: Vec<Vec<Vec3>>,
    pub triangle_count: Vec<usize>,
    pub params_number: usize,
    pub filepath: String,
    pub texturepath: String,
    pub pos: Vec3,
    pub dir: Vec3,
    pub scale: f64,
}

impl Obj {
    pub fn new() -> Self {
        Obj {
            vertices: Vec::new(),
            normals: Vec::new(),
            textures: Vec::new(),
            faces: Vec::new(),
            triangle_count: Vec::new(),
            params_number: 1,
            filepath: String::new(),
            texturepath: String::new(),
            pos: Vec3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 1.0, 0.0),
            scale: 1.0,
        }
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

    pub fn add_triangle_count(&mut self, shape: usize) {
        self.triangle_count.push(shape);
    }

    pub fn set_filepath(&mut self, filepath: String) {
        self.filepath = filepath;
    }

    pub fn set_texturepath(&mut self, texturepath: String) {
        self.texturepath = texturepath;
    }

    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
    }

    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
    }

    pub fn set_scale(&mut self, scale: f64) {
        self.scale = scale;
    }

    pub fn triangle_count(&self) -> &Vec<usize> {
        &self.triangle_count
    }

    pub fn params_number(&self) -> usize {
        self.params_number
    }

    pub fn filepath(&self) -> &String {
        &self.filepath
    }

    pub fn texturepath(&self) -> &String {
        &self.texturepath
    }

    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn parse_file(&mut self, filepath: String) -> Result<(), std::io::Error> {
        let file = File::open(filepath)?;
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
                            let x = tokens[1].parse::<f64>().unwrap() * self.scale() + *self.pos().x();
                            let y = tokens[2].parse::<f64>().unwrap() * self.scale() + *self.pos().y();
                            let z = tokens[3].parse::<f64>().unwrap() * self.scale() + *self.pos().z();
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

    pub fn set_params_number(&mut self) {
        self.params_number = 1 + (self.textures.len() > 0) as usize + (self.normals.len() > 0) as usize;
    }
}

