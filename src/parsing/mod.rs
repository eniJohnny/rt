pub mod json;
pub mod basic;
pub mod elements;
pub mod textures;

use elements::{get_ambient, get_any, get_brick, get_camera, get_cone, get_cube, get_cubehole, get_cylinder, get_ellipse, get_helix, get_hyperboloid, get_light, get_mobius, get_nagone, get_obj, get_parallel, get_plane, get_rectangle, get_sphere, get_spot, get_torus, get_torusphere, get_triangle};
use json::JsonValue;
use basic::get_color_texture;
use textures::get_texture;
use crate::model::{materials::texture::Texture, scene::Scene};
use std::{collections::HashMap, io::{stdout, Write}};

pub fn print_scene(scene: &Scene) {
    write!(stdout(), "{:#?}\n", scene).expect("Error printing scene");
}

fn match_object(scene: &mut Scene, object: HashMap<String, JsonValue>) -> Result<(), String>
{
    if let Some(object_type) = object.get("type") {
        if let JsonValue::String(object_type) = object_type {
            match object_type.as_str() {
                "skybox" => {
                    let skybox_texture = get_color_texture(&object)?;
                    if let Texture::Texture(path, _) = &skybox_texture {
                        scene.load_texture(path, None);
                    }
                    scene.set_skybox(skybox_texture);
                }
                "camera" => {
                    scene.add_camera(get_camera(&object)?);
                }
                "light" => {
                    let light = get_light(&object)?;
                    scene.add_light(light);
                }
                "ambient" => {
                    let ambient_light = get_ambient(&object)?;
                    scene.add_ambient_light(ambient_light);
                }
                "parallel" => {
                    let parallel_light = get_parallel(&object)?;
                    scene.add_light(parallel_light);
                }
                "spot" => {
                    let spot_light = get_spot(&object)?;
                    scene.add_light(spot_light);
                }
                "sphere" => {
                    let sphere = get_sphere(&object)?;
                    scene.load_material_textures(sphere.material());
                    scene.add_element(sphere);
                }
                "plane" => {
                    let plane = get_plane(&object)?;
                    scene.load_material_textures(plane.material());
                    scene.add_element(plane);
                }
                "cylinder" => {
                    let cylinder = get_cylinder(&object)?;
                    scene.load_material_textures(cylinder.material());
                    scene.add_element(cylinder);
                }
                "cone" => {
                    let cone = get_cone(&object)?;
                    scene.load_material_textures(cone.material());
                    scene.add_element(cone);
                }
                "triangle" => {
                    let triangle = get_triangle(&object)?;
                    scene.load_material_textures(triangle.material());
                    scene.add_element(triangle);
                }
                "rectangle" => {
                    let rectangle = get_rectangle(&object)?;
                    scene.load_material_textures(rectangle.material());
                    scene.add_element(rectangle);
                }
                "torus" => {
                    let torus = get_torus(&object)?;
                    scene.load_material_textures(torus.material());
                    scene.add_element(torus);
                }
                "torusphere" => {
                    let torusphere = get_torusphere(&object)?;
                    scene.load_material_textures(torusphere.material());
                    scene.add_composed_element(torusphere);
                }
                "helix" => {
                    let helix = get_helix(&object)?;
                    scene.load_material_textures(helix.material());
                    scene.add_composed_element(helix);
                }
                "brick" => {
                    let brick = get_brick(&object)?;
                    scene.load_material_textures(brick.material());
                    scene.add_composed_element(brick);
                }
                "nagone" => {
                    let nagone = get_nagone(&object)?;
                    scene.load_material_textures(nagone.material());
                    scene.add_composed_element(nagone);
                }
                "mobius" => {
                    let mobius = get_mobius(&object)?;
                    scene.load_material_textures(mobius.material());
                    scene.add_composed_element(mobius);
                }
                "ellipse" => {
                    let ellipse = get_ellipse(&object)?;
                    scene.load_material_textures(ellipse.material());
                    scene.add_element(ellipse);
                }
                "cube" => {
                    let cube = get_cube(&object)?;
                    scene.load_material_textures(cube.material());
                    scene.add_element(cube);
                }
                "cubehole" => {
                    let cubehole = get_cubehole(&object)?;
                    scene.load_material_textures(cubehole.material());
                    scene.add_element(cubehole);
                }
                "hyperboloid" => {
                    let hyperboloid = get_hyperboloid(&object)?;
                    scene.load_material_textures(hyperboloid.material());
                    scene.add_element(hyperboloid);
                }
                "any" => {
                    let any = get_any(&object)?;
                    scene.load_material_textures(any.material());
                    scene.add_element(any);
                }
                "obj" => {
                    let obj = get_obj(&object)?;
                    scene.load_material_textures(obj.material());
                    scene.add_composed_element(obj);
                }
                "texture" => {
                    let (name, img) = get_texture(&object)?;
                    scene.load_texture(&name, Some(img));
                }
                _ => {
                    return Err(format!("Unknown type detected: {}", object_type));
                }
            }
            return Ok(());
        } else {
            return Err("The type of an object must be a string !".to_string());
        }
    } else {
        return Err("The type of an object is missing".to_string());
    }
}

pub fn parse_scene_content(scene: &mut Scene, scene_content: JsonValue) -> Result<(), String> {
    if let JsonValue::Object(json_object) = scene_content {
        return match_object(scene, json_object);
    } else if let JsonValue::Array(objects) = scene_content {
        for object in objects {
            parse_scene_content(scene, object)?;
        }
        return Ok(());
    } else {
        return Err("The scene shoulds only contain arrays or objects".to_string());
    } 
}

pub fn get_scene(scene_file: &String) -> Result<Scene, String> {
    let mut scene = Scene::new();
    match json::parse_json_file(scene_file) {
        Ok(json_value) => {
            match parse_scene_content(&mut scene, json_value) {
                Ok(_) => {
                    return Ok(scene);
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        Err(err) => {
            return Err(err);
        }
    }
}