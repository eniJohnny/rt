use std::collections::HashMap;

use image::RgbaImage;

use crate::model::{materials::{color::Color, material::Projection, texture::Texture}, Element};

use super::vec3::Vec3;

#[derive(Debug, Clone)]
pub enum HitType {
    Reflect,
    Direct,
}

#[derive(Debug, Clone)]
pub struct Hit<'a> {
    element: &'a Element,
    dist: f64,
    pos: Vec3,
    norm: Vec3,
    projected_pos: Option<(f64, f64)>,
    color: Color,
    metalness: f64,
    roughness: f64,
    refraction: f64,
    norm_variation: Vec3,
    emissive: f64
}

impl<'a> Hit<'a> {
    pub fn new(element: &'a Element, dist: f64, pos: Vec3, ray_dir: &Vec3, textures: &HashMap<String, image::RgbaImage>) -> Self {
        let mut hit = Hit {
            element,
            dist,
            norm: Vec3::new(0., 0., 0.),
            pos,
            projected_pos: None,
            color: Color::new(0., 0., 0.),
            metalness: 0.,
            roughness: 0.,
            refraction: 0.,
            norm_variation: Vec3::new(0., 0., 0.),
            emissive: 0.
        };
        Hit::map(&mut hit, ray_dir, textures);
        hit
    }

    pub fn element(&self) -> &'a Element {
        self.element
    }

    pub fn dist(&self) -> &f64 {
        &self.dist
    }

    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }

    pub fn norm(&self) -> &Vec3 {
        &self.norm
    }

    pub fn color(&self) -> &Color {
        &self.color
    }
    pub fn metalness(&self) -> f64 {
        self.metalness
    }
    pub fn refraction(&self) -> f64 {
        self.refraction
    }
    pub fn roughness(&self) -> f64 {
        self.roughness
    }
    pub fn emissive(&self) -> f64 {
        self.emissive
    }

    fn get_projection(&self, projection: Option<Projection>) -> Option<Projection> {
        match projection {
            None => Some(self.element().shape().projection(self)),
            Some(p) => Some(p)
        }
    }

    fn map(&mut self, ray_dir: &Vec3, textures: &HashMap<String, RgbaImage>) {
        let mut projection_opt: Option<Projection> = None;
        let mat = self.element.material();

        self.norm = self.element.shape().norm(&self.pos, ray_dir);
        // match mat.norm() {
        //     Texture::Texture(file) => {
        //         projection = self.get_projection(projection);
        //         if let Some((x, y)) = projection {
        //             todo!()
        //         }
        //     },
        //     Texture::Value(norm) => {
        //         self.norm = todo!();
        //     }
        // }
        
        match mat.color() {
            Texture::Texture(file) => {
                projection_opt = self.get_projection(projection_opt);
                if let Some(projection) = &projection_opt {
					let img = textures.get(file).unwrap();
					let x = (&projection.u * img.width() as f64) as u32;
					let y = ((1. - &projection.v) * img.height() as f64) as u32;
					self.color = Color::from_rgba(img.get_pixel(x, y));
				}
            },
            Texture::Value(color) => {
                self.color = Color::from_vec3(color);
            }
        }

        match mat.roughness() {
            Texture::Texture(file) => {
                projection_opt = self.get_projection(projection_opt);
                if let Some(projection) = &projection_opt {
					let img = textures.get(file).unwrap();
					let x = (&projection.u * img.width() as f64) as u32;
					let y = ((1. - &projection.v) * img.height() as f64) as u32;
					let color = Color::from_rgba(img.get_pixel(x, y));
					self.roughness = Vec3::from_color(color).to_value();
				}
            },
            Texture::Value(roughness) => {
                self.roughness = roughness.to_value() * roughness.to_value();
            }
        }

        match mat.metalness() {
            Texture::Texture(file) => {
                projection_opt = self.get_projection(projection_opt);
                if let Some(projection) = &projection_opt {
					let img = textures.get(file).unwrap();
					let x = (&projection.u * img.width() as f64) as u32;
					let y = ((1. - &projection.v) * img.height() as f64) as u32;
					let color = Color::from_rgba(img.get_pixel(x, y));
					self.metalness = Vec3::from_color(color).to_value();
				}
            },
            Texture::Value(metalness) => {
                self.metalness = metalness.to_value();
            }
        }

        match mat.emissive() {
            Texture::Texture(file) => {
                projection_opt = self.get_projection(projection_opt);
                if let Some(projection) = projection_opt {
					todo!()
                }
            },
            Texture::Value(emissive) => {
                self.emissive = emissive.to_value();
            }
        }

        match mat.refraction() {
            Texture::Texture(file) => {
                projection_opt = self.get_projection(projection_opt);
                if let Some(projection) = projection_opt {
					todo!()
                }
            },
            Texture::Value(refraction) => {
                self.refraction = refraction.to_value();
            }
        }
    }
}
