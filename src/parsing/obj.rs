use crate::model::materials::color::Color;
use crate::model::materials::material::{self, Material};
use crate::model::materials::texture::{Texture, TextureType};
use crate::model::maths::vec3::Vec3;
use crate::model::scene::Scene;
use crate::model::shapes::triangle::Triangle;
use crate::model::Element;
use crate::ui::utils::misc::Value;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::vec;

pub struct Obj {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub textures: Vec<Vec3>,
    pub faces: Vec<Vec<Vec3>>,
}

impl Obj {
    pub fn new() -> Self {
        Obj {
            vertices: Vec::new(),
            normals: Vec::new(),
            textures: Vec::new(),
            faces: Vec::new(),
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
                            let x = tokens[1].parse::<f64>().unwrap();
                            let z = tokens[2].parse::<f64>().unwrap();
                            let y = tokens[3].parse::<f64>().unwrap();
                            self.add_vertex(Vec3::new(x, y, z));
                        }
                        "vn" => {
                            let x = tokens[1].parse::<f64>().unwrap();
                            let z = tokens[2].parse::<f64>().unwrap();
                            let y = tokens[3].parse::<f64>().unwrap();
                            self.add_normal(Vec3::new(x, y, z));
                        }
                        "vt" => {
                            let x = tokens[1].parse::<f64>().unwrap();
                            let z = tokens[2].parse::<f64>().unwrap();
                            let y = tokens[3].parse::<f64>().unwrap();
                            self.add_texture(Vec3::new(x, y, z));
                        }
                        "f" => {
                            let args_number: usize = tokens[1].split("/").count();
                            let mut face: Vec<Vec3> = vec![];
                            for i in 1..4 {
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

        return Ok(());
    }

    // pub fn create_elements(&self, scene: &mut Scene) {
    //     let len = self.faces.len();

    //     for i in 0..len {
    //         let face = &self.faces[i];
    //         let mut vertices: Vec<Vec3> = vec![];
    //         let mut normals: Vec<Vec3> = vec![];
    //         let mut textures: Vec<Vec3> = vec![];

    //         for j in 0..face.len() {
    //             if j % 3 == 0 {
    //                 vertices.push(face[j]);
    //             } else if j % 3 == 1 {
    //                 textures.push(face[j]);
    //             } else {
    //                 normals.push(face[j]);
    //             }
    //         }
    //         let mut material = <dyn Material>::default();
    //         material.set_color(Texture::Value(Vec3::from_value(1.0), TextureType::Float));
    //         material.set_opacity(Texture::Value(Vec3::from_value(1.0), TextureType::Float));

    //         let triangle = Triangle::new(vertices[0], vertices[1], vertices[2]);
    //         let elem = Element::new(Box::new(triangle), material);
    //         scene.add_element(elem);
    //     }
    // }
}

