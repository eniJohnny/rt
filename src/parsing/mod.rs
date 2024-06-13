use pixels::wgpu::ShaderModule;

use crate::model::materials::color::Color;
use crate::model::materials::diffuse::Diffuse;
use crate::model::materials::material::Material;
use crate::model::materials::texture::Texture;
use crate::model::maths::vec3::Vec3;
use crate::model::objects::camera::Camera;
use crate::model::objects::light::{AmbientLight, Light, ParallelLight, PointLight};
use crate::model::{scene, Element};
use crate::model::{
    scene::Scene, shapes::cone::Cone, shapes::cylinder::Cylinder, shapes::plane::Plane,
    shapes::sphere::Sphere, shapes::polygon::Polygon, shapes::polygon::Triangle, shapes::rectangle::Rectangle
};
use crate::{error, MAX_EMISSIVE};
// use crate::{error, SCENE};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::io::Write;
use std::ops::Add;

pub fn print_scene(scene: &Scene) {
    write!(std::io::stdout(), "{:#?}\n", scene).expect("Error printing scene");
}

pub fn get_scene(scene_file: &String) -> Scene {
    let mut scene = Scene::new();
    let objects = parse_json(scene_file.clone());

    for object in objects {
        match object["type"].as_str() {
            "sphere" => {
                let pos = get_coordinates_value(&object, "pos");
                let radius = get_float_value(&object, "radius");
                let color = get_color(&object);
                let dir = get_coordinates_value(&object, "dir");

                let shape = Box::new(Sphere::new(pos, dir, radius));
                let material = get_material(&object, color);
                scene.add_textures(&material);
                let element = Element::new(shape, material);
                scene.add_element(element)
            }
            "plane" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir");
                let color = get_color(&object);

                let shape = Box::new(Plane::new(pos, dir));
                let material = get_material(&object, color);
                scene.add_textures(&material);
                let element = Element::new(shape, material);
                scene.add_element(element)
            }
            "cylinder" => {
                let pos = get_coordinates_value(&object, "pos");
                let radius = get_float_value(&object, "radius");
                let height = get_float_value(&object, "height");
                let color = get_color(&object);
                let dir: Vec3 = get_coordinates_value(&object, "dir");

                let shape = Box::new(Cylinder::new(pos, dir, radius, height));
                let material = get_material(&object, color);
                scene.add_textures(&material);
                let element = Element::new(shape, material);
                scene.add_element(element)
            }
            "cone" => {
                let pos = get_coordinates_value(&object, "pos");
                let radius = get_float_value(&object, "radius");
                let height = get_float_value(&object, "height");
                let color = get_color(&object);
                let dir: Vec3 = get_coordinates_value(&object, "dir");

                let shape = Box::new(Cone::new(pos, dir, radius, height));
                let material = get_material(&object, color);

                let element = Element::new(shape, material);
                scene.add_element(element)
            }
            "triangle" => {
                let a = get_coordinates_value(&object, "a");
                let b = get_coordinates_value(&object, "b");
                let c = get_coordinates_value(&object, "c");
                let color = get_color(&object);

                let shape = Box::new(Polygon::new(Vec::from([Triangle{ a,b,c }])));
                let material = get_material(&object, color);

                let element = Element::new(shape, material);
                scene.add_element(element)
            }
            "rectangle" => {
                let pos = get_coordinates_value(&object, "pos");
                let length = get_float_value(&object, "length");
                let width = get_float_value(&object, "width");
                let dir_l = get_coordinates_value(&object, "dir_l");
                let dir_w = get_coordinates_value(&object, "dir_w");
                let color = get_color(&object);

                let shape = Box::new(Rectangle::new(pos, length, width, dir_l, dir_w));
                let material = get_material(&object, color);

                scene.add_textures(&material);
                let element = Element::new(shape, material);
                scene.add_element(element)
            }
            "camera" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir");
                let fov = get_float_value(&object, "fov") * 2. * PI / 360.;

                let new_camera = Camera::new(pos, dir, fov);
                scene.add_camera(new_camera);
            }
            "light" => {
                let pos = get_coordinates_value(&object, "pos");
                let intensity = get_float_value(&object, "intensity");
                let color = match get_color(&object) {
                    Some(color) => color,
                    None => panic!("Color must be provided for lights"),
                };
                let new_light = Box::new(PointLight::new(pos, intensity, color))
                    as Box<dyn Light + Sync + Send>;
                scene.add_light(new_light);
            }
            "ambient" => {
                let intensity = get_float_value(&object, "intensity");
                let color = match get_color(&object) {
                    Some(color) => color,
                    None => panic!("Color must be provided for lights"),
                };

                let new_ambient_light = AmbientLight::new(intensity, color);
                scene.add_ambient_light(new_ambient_light);
            }
            "parallel" => {
                let intensity = get_float_value(&object, "intensity");
                let dir = get_coordinates_value(&object, "dir");
                let color = match get_color(&object) {
                    Some(color) => color,
                    None => panic!("Color must be provided for lights"),
                };

                let new_light = Box::new(ParallelLight::new(dir, intensity, color));
                scene.add_light(new_light);
            }
            _ => {}
        }
    }

    return scene;
}

fn parse_json(scene_file: String) -> Vec<HashMap<String, String>> {
    let content = std::fs::read_to_string(scene_file).expect("Error reading file");
    let mut objects: Vec<HashMap<String, String>> = Vec::new();
    let mut i = 0;

    while i < content.len() && content[i..].find('{') != None {
        let mut object: HashMap<String, String> = HashMap::new();
        let remaining = &content[i..];
        let start = remaining.find('{').expect("No opening bracket found");
        let end = remaining.find("\n    }").expect("No closing bracket found") + 6;
        let object_str = &remaining[start..end];
        i += end;

        for prop in object_str.split(",\n        \"") {
            let prop = prop.trim();
            let mut prop = prop.split(": ");
            let key: String = prop
                .next()
                .expect("Error parsing key")
                .trim_matches(['"', ' ', '\n', '{', '}'])
                .to_string();
            let value: String = prop
                .next()
                .expect("Error parsing key")
                .trim_matches(['{', '"', ' ', '\n', '}'])
                .to_string();

            if value.contains('[') {
                let str = value.trim_matches(['[', ']']).replace(", ", ",");
                let tmp: Vec<&str> = str.split(",").collect();

                if key == "color" {
                    object.insert("color_r".to_string(), tmp[0].to_string());
                    object.insert("color_g".to_string(), tmp[1].to_string());
                    object.insert("color_b".to_string(), tmp[2].to_string());
                }
                else {
                    object.insert(key.clone().add("_x"), tmp[0].to_string());
                    object.insert(key.clone().add("_y"), tmp[1].to_string());
                    object.insert(key.clone().add("_z"), tmp[2].to_string());
                }
            } else {
                object.insert(key, value);
            }
        }
        objects.push(object);
    }

    // Here, objects is a vector of HashMaps, each representing an object in the scene.
    return objects;
}

fn get_color(object: &HashMap<String, String>) -> Option<Color> {
    if object.get("color").is_some() {
        return None;
    }
    // Testing if the color is in the format [r, g, b]
    let rgb_str = [&object["color_r"], &object["color_g"], &object["color_b"]];

    for i in 0..3 {
        if rgb_str[i].parse::<u8>().is_err() {
            error("Color must be in the format [r, g, b] where r, g, b are integers.");
        }
    }

    let r = object["color_r"]
        .parse::<u8>()
        .expect("Error parsing color");
    let g = object["color_g"]
        .parse::<u8>()
        .expect("Error parsing color");
    let b = object["color_b"]
        .parse::<u8>()
        .expect("Error parsing color");

    return Some(Color::new(
        r as f64 / 255.,
        g as f64 / 255.,
        b as f64 / 255.,
    ));
}

fn get_coordinates_value(object: &HashMap<String, String>, key: &str) -> Vec3 {
    // Testing if the position is in the format [x, y, z]
    let pos_str = [
        &object[&(key.to_owned() + "_x")],
        &object[&(key.to_owned() + "_y")],
        &object[&(key.to_owned() + "_z")],
    ];

    for i in 0..3 {
        if pos_str[i].parse::<f64>().is_err() {
            error(&(key.to_owned() + " must be in the format [x, y, z] where x, y, z are floats."));
        }
    }

    Vec3::new(
        object[&(key.to_owned() + "_x")]
            .parse::<f64>()
            .expect(&("Error parsing ".to_owned() + &key)),
        object[&(key.to_owned() + "_y")]
            .parse::<f64>()
            .expect(&("Error parsing ".to_owned() + &key)),
        object[&(key.to_owned() + "_z")]
            .parse::<f64>()
            .expect(&("Error parsing ".to_owned() + &key)),
    )
}

fn get_material(
    object: &HashMap<String, String>,
    color_opt: Option<Color>,
) -> Box<dyn Material + Sync + Send> {
    let default: String = String::from("");
    let metalness_string = object.get("metalness").unwrap_or(&default);
    let roughness_string = object.get("roughness").unwrap_or(&default);
    let refraction_string = object.get("refraction").unwrap_or(&default);
    let emissive_string = object.get("emissive").unwrap_or(&default);
    let normal_string = object.get("normal").unwrap_or(&default);
    let opacity_string = object.get("opacity").unwrap_or(&default);
    let color_texture = match object.get("color") {
        Some(path) => Texture::Texture(path.clone()),
        None => match color_opt {
            Some(color) => Texture::Value(Vec3::new(color.r(), color.g(), color.b())),
            None => panic!("Color must be provided for non-textured materials"),
        },
    };
    Box::new(Diffuse::new(
        color_texture,
        Texture::from_float_litteral(metalness_string, 0.),
        Texture::from_float_litteral(roughness_string, 0.),
        Texture::from_float_scaled(emissive_string, 0., MAX_EMISSIVE),
        Texture::from_float_litteral(refraction_string, 0.),
        Texture::from_file_or(normal_string, Vec3::new(0.5, 0.5, 1.)),
        Texture::from_float_litteral(opacity_string, 1.),
    ))
}

fn get_float_value(object: &HashMap<String, String>, key: &str) -> f64 {
    // Testing if the intensity is a float
    if object[key].parse::<f64>().is_err() {
        error(&(key.to_owned() + " must be a float."));
    }

    object[key]
        .parse::<f64>()
        .expect(&("Error parsing ".to_owned() + key))
}