pub mod json;
pub mod basic;
pub mod elements;

use std::{
    array, collections::HashMap, f64::consts::PI, fmt::format, io::Write, ops::Add, process::exit
};
use elements::{get_ambient, get_any, get_brick, get_camera, get_cone, get_cube, get_cubehole, get_cylinder, get_ellipse, get_helix, get_hyperboloid, get_light, get_mobius, get_nagone, get_obj, get_parallel, get_plane, get_rectangle, get_sphere, get_spot, get_torus, get_torusphere, get_triangle};
use json::JsonValue;
use basic::{get_color_texture};

use web_sys::js_sys::Object;
use winit::dpi::Position;

use crate::{
    error, model::{
        composed_element::ComposedElement, element::Element, materials::{
            color::Color,
            diffuse::Diffuse,
            material::Material,
            texture::{Texture, TextureType}
        }, maths::vec3::Vec3, objects::{
            self, camera::Camera, light::{AmbientLight, AnyLight, ParallelLight, PointLight}
        }, scene::Scene, shapes::{ 
            any::Any, brick::Brick, composed_shape::ComposedShape, cone::Cone, cube::Cube, cubehole::Cubehole, cylinder::Cylinder, ellipse::Ellipse, helix::Helix, hyperboloid::Hyperboloid, mobius::Mobius, nagone::Nagone, obj::Obj, plane::Plane, rectangle::Rectangle, sphere::Sphere, torusphere::Torusphere, triangle::Triangle
        }
    }, AABB_OPACITY
};

pub fn print_scene(scene: &Scene) {
    write!(std::io::stdout(), "{:#?}\n", scene).expect("Error printing scene");
}

pub fn get_scene(scene_file: &String) -> Result<Scene, String> {
    let mut scene = Scene::new();
    match json::parse_json_file(scene_file) {
        Ok(json_value) => {
            if let JsonValue::Array(objects) = json_value {
                for object in objects {
                    if let JsonValue::Object(json_object) = object {
                        if let Some(object_type) = json_object.get("type") {
                            if let JsonValue::String(object_type) = object_type {
                                match object_type.as_str() {
                                    "skybox" => {
                                        let skybox_texture = get_color_texture(&json_object)?;
                                        if let Texture::Texture(path, _) = &skybox_texture {
                                            scene.load_texture(path);
                                        }
                                        scene.set_skybox(skybox_texture);
                                    }
                                    "camera" => {
                                        scene.add_camera(get_camera(&json_object)?);
                                    }
                                    "light" => {
                                        let light = get_light(&json_object)?;
                                        scene.add_light(light);
                                    }
                                    "ambient" => {
                                        let ambient_light = get_ambient(&json_object)?;
                                        scene.add_ambient_light(ambient_light);
                                    }
                                    "parallel" => {
                                        let parallel_light = get_parallel(&json_object)?;
                                        scene.add_light(parallel_light);
                                    }
									"spot" => {
										let spot_light = get_spot(&json_object)?;
										scene.add_light(spot_light);
									}
                                    "sphere" => {
                                        let sphere = get_sphere(&json_object)?;
                                        scene.load_material_textures(sphere.material());
                                        scene.add_element(sphere);
                                    }
                                    "plane" => {
                                    	let plane = get_plane(&json_object)?;
                                        scene.load_material_textures(plane.material());
                                        scene.add_element(plane);
                                    }
                                    "cylinder" => {
                                        let cylinder = get_cylinder(&json_object)?;
                                        scene.load_material_textures(cylinder.material());
                                        scene.add_element(cylinder);
                                    }
                                    "cone" => {
                                        let cone = get_cone(&json_object)?;
                                        scene.load_material_textures(cone.material());
                                        scene.add_element(cone);
                                    }
                                    "triangle" => {
                                        let triangle = get_triangle(&json_object)?;
                                        scene.load_material_textures(triangle.material());
                                        scene.add_element(triangle);
                                    }
                                    "rectangle" => {
                                        let rectangle = get_rectangle(&json_object)?;
                                        scene.load_material_textures(rectangle.material());
                                        scene.add_element(rectangle);
                                    }
                                    "torus" => {
                                        let torus = get_torus(&json_object)?;
                                        scene.load_material_textures(torus.material());
                                        scene.add_element(torus);
                                    }
                                    "torusphere" => {
                                        let torusphere = get_torusphere(&json_object)?;
                                        scene.load_material_textures(torusphere.material());
                                        scene.add_composed_element(torusphere);
                                    }
                                    "helix" => {
                                        let helix = get_helix(&json_object)?;
                                        scene.load_material_textures(helix.material());
                                        scene.add_composed_element(helix);
                                    }
                                    "brick" => {
                                        let brick = get_brick(&json_object)?;
                                        scene.load_material_textures(brick.material());
                                        scene.add_composed_element(brick);
                                    }
                                    "nagone" => {
                                        let nagone = get_nagone(&json_object)?;
                                        scene.load_material_textures(nagone.material());
                                        scene.add_composed_element(nagone);
                                    }
                                    "mobius" => {
                                        let mobius = get_mobius(&json_object)?;
                                        scene.load_material_textures(mobius.material());
                                        scene.add_composed_element(mobius);
                                    }
                                    "ellipse" => {
                                        let ellipse = get_ellipse(&json_object)?;
                                        scene.load_material_textures(ellipse.material());
                                        scene.add_element(ellipse);
                                    }
                                    "cube" => {
                                        let cube = get_cube(&json_object)?;
                                        scene.load_material_textures(cube.material());
                                        scene.add_element(cube);
                                    }
                                    "cubehole" => {
                                        let cubehole = get_cubehole(&json_object)?;
                                        scene.load_material_textures(cubehole.material());
                                        scene.add_element(cubehole);
                                    }
                                    "hyperboloid" => {
                                        let hyperboloid = get_hyperboloid(&json_object)?;
                                        scene.load_material_textures(hyperboloid.material());
                                        scene.add_element(hyperboloid);
                                    }
                                    "any" => {
                                        let any = get_any(&json_object)?;
                                        scene.load_material_textures(any.material());
                                        scene.add_element(any);
                                    }
                                    "obj" => {
                                        let obj = get_obj(&json_object)?;
                                        scene.load_material_textures(obj.material());
                                        scene.add_composed_element(obj);
                                    }
                                    _ => {
                                        return Err(format!("Unknown type detected: {}", object_type));
                                    }
                                }
                            } else {
                                return Err("The type of an object must be a string !".to_string());
                            }
                        } else {
                            return Err("The type of an object is missing".to_string());
                        }
                    } else {
                        return Err("The main array should only contain objects".to_string());
                    }
                }
                return Ok(scene);
            }
            return Err("The scene should only contain an array".to_string());
        }
        Err(err) => {
            return Err(err);
        }
    }
}