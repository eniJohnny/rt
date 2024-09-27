use obj::Obj;

use crate::model::materials::color::Color;
use crate::model::materials::diffuse::Diffuse;
use crate::model::materials::material::Material;
use crate::model::materials::texture::{self, Texture, TextureType};
use crate::model::maths::vec3::Vec3;
use crate::model::objects::camera::Camera;
use crate::model::objects::light::{AmbientLight, AnyLight, ParallelLight, PointLight};
use crate::model::{ComposedElement, Element};
use crate::model::scene::Scene;
use crate::model::shapes::{ 
    cone::Cone, cylinder::Cylinder, plane::Plane,
    sphere::Sphere, rectangle::Rectangle, triangle::Triangle,
    ComposedShape, helix::Helix, torusphere::Torusphere,
    brick::Brick, nagone::Nagone, mobius::Mobius, ellipse::Ellipse,
    cube::Cube, cubehole::Cubehole, hyperboloid::Hyperboloid, any::Any
};
use crate::{error, AABB_OPACITY};
// use crate::{error, SCENE};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::io::Write;
use std::ops::Add;

pub mod obj;

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
                let aabb_shape = Box::new(shape.aabb().clone());

                let material = get_material(&object, color);
                scene.add_textures(&material);
                let mut aabb_material = get_material(&object, Some(Color::new(255., 255., 255.)));
                aabb_material.set_opacity(Texture::Value(Vec3::from_value(AABB_OPACITY), TextureType::Float));
                
                let element = Element::new(shape, material);
                let aabb_element = Element::new(aabb_shape, aabb_material);

                scene.add_element(element);
                scene.add_element(aabb_element);
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
                let aabb_shape = Box::new(shape.aabb().clone());

                let material = get_material(&object, color);
                scene.add_textures(&material);
                let mut aabb_material = get_material(&object, Some(Color::new(255., 255., 255.)));
                aabb_material.set_opacity(Texture::Value(Vec3::from_value(AABB_OPACITY), TextureType::Float));

                let element = Element::new(shape, material);
                let aabb_element = Element::new(aabb_shape, aabb_material);

                scene.add_element(element);
                scene.add_element(aabb_element);
            }
            "cone" => {
                let pos = get_coordinates_value(&object, "pos");
                let radius = get_float_value(&object, "radius");
                let height = get_float_value(&object, "height");
                let color = get_color(&object);
                let dir: Vec3 = get_coordinates_value(&object, "dir");

                let shape = Box::new(Cone::new(pos, dir, radius, height));
                let aabb_shape = Box::new(shape.aabb().clone());

                let material = get_material(&object, color);
                let mut aabb_material = get_material(&object, Some(Color::new(255., 255., 255.)));
                aabb_material.set_opacity(Texture::Value(Vec3::from_value(AABB_OPACITY), TextureType::Float));
                scene.add_textures(&material);

                let element = Element::new(shape, material);
                let aabb_element = Element::new(aabb_shape, aabb_material);
                scene.add_element(element);
                scene.add_element(aabb_element);
            }
            "triangle" => {
                let a = get_coordinates_value(&object, "a");
                let b = get_coordinates_value(&object, "b");
                let c = get_coordinates_value(&object, "c");
                let color = get_color(&object);

                let shape = Box::new(Triangle::new(a,b,c));
                let aabb_shape = Box::new(shape.aabb().clone());

                let material = get_material(&object, color);
                let mut aabb_material = get_material(&object, Some(Color::new(255., 255., 255.)));
                aabb_material.set_opacity(Texture::Value(Vec3::from_value(AABB_OPACITY), TextureType::Float));
                
                scene.add_textures(&material);

                let element = Element::new(shape, material);
                let aabb_element = Element::new(aabb_shape, aabb_material);

                scene.add_element(element);
                scene.add_element(aabb_element);
            }
            "rectangle" => {
                let pos = get_coordinates_value(&object, "pos");
                let length = get_float_value(&object, "length");
                let width = get_float_value(&object, "width");
                let dir_l = get_coordinates_value(&object, "dir_l");
                let dir_w = get_coordinates_value(&object, "dir_w");
                let color = get_color(&object);

                let shape = Box::new(Rectangle::new(pos, length, width, dir_l, dir_w));
                let aabb_shape = Box::new(shape.aabb().clone());

                let material = get_material(&object, color);
                let mut aabb_material = get_material(&object, Some(Color::new(255., 255., 255.)));
                aabb_material.set_opacity(Texture::Value(Vec3::from_value(AABB_OPACITY), TextureType::Float));
                
                scene.add_textures(&material);

                let element = Element::new(shape, material);
                let aabb_element = Element::new(aabb_shape, aabb_material);

                // println!("{:?}", &element);
                scene.add_element(element);
                scene.add_element(aabb_element);
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
                let new_light = AnyLight::new(Box::new(PointLight::new(pos, intensity, color)));
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

                let new_light = AnyLight::new(Box::new(ParallelLight::new(dir, intensity, color)));
                scene.add_light(new_light);
            }
            "torusphere" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir");
                let radius = get_float_value(&object, "radius");
                let steps = get_float_value(&object, "steps") as usize;
                let color = match get_color(&object) {
                    Some(color) => Vec3::new(color.r(), color.g(), color.b()),
                    None => panic!("Color must be provided for toruspheres"),
                };

                let torusphere = Torusphere::new(pos, dir, radius, steps, color);
                let composed_shape = Box::new(torusphere) as Box<dyn ComposedShape + Sync + Send>;
                let composed_element = ComposedElement::new(composed_shape);
                scene.add_composed_element(composed_element);
            }
            "helix" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir");
                let height = get_float_value(&object, "height");

                let helix = Helix::new(pos, dir, height);
                let composed_shape = Box::new(helix) as Box<dyn ComposedShape + Sync + Send>;
                let composed_element = ComposedElement::new(composed_shape);
                scene.add_composed_element(composed_element);
            }
            "brick" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir");
                let dimensions = get_coordinates_value(&object, "dimensions");
                let color = match get_color(&object) {
                    Some(color) => Vec3::new(color.r(), color.g(), color.b()),
                    None => panic!("Color must be provided for bricks"),
                };

                let brick = Brick::new(pos, dir, dimensions, color);
                let composed_shape = Box::new(brick) as Box<dyn ComposedShape + Sync + Send>;
                let composed_element = ComposedElement::new(composed_shape);
                scene.add_composed_element(composed_element);
            }
            "nagone" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir");
                let radius = get_float_value(&object, "radius");
                let angles = get_float_value(&object, "angles") as usize;
                let color = match get_color(&object) {
                    Some(color) => Vec3::new(color.r(), color.g(), color.b()),
                    None => panic!("Color must be provided for nagones"),
                };

                let nagone = Nagone::new(pos, dir, radius, angles, color);
                let composed_shape = Box::new(nagone) as Box<dyn ComposedShape + Sync + Send>;
                let composed_element = ComposedElement::new(composed_shape);
                scene.add_composed_element(composed_element);
            }
            "mobius" => {
                let pos = get_coordinates_value(&object, "pos");
                let radius = get_float_value(&object, "radius");
                let half_width = get_float_value(&object, "half_width");
                let color = match get_color(&object) {
                    Some(color) => Vec3::new(color.r(), color.g(), color.b()),
                    None => panic!("Color must be provided for mobius"),
                };

                let mobius = Mobius::new(pos, radius, half_width, color);
                let composed_shape = Box::new(mobius) as Box<dyn ComposedShape + Sync + Send>;
                let composed_element = ComposedElement::new(composed_shape);
                scene.add_composed_element(composed_element);
            }
            "ellipse" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir").normalize();
                let u = get_float_value(&object, "u");
                let v = get_float_value(&object, "v");
                let color = get_color(&object);

                let shape = Box::new(Ellipse::new(pos, dir, u, v));
                let aabb_shape = Box::new(shape.aabb().clone());

                let material = get_material(&object, color);
                scene.add_textures(&material);
                let mut aabb_material = get_material(&object, Some(Color::new(255., 255., 255.)));
                aabb_material.set_opacity(Texture::Value(Vec3::from_value(AABB_OPACITY), TextureType::Float));
                
                let element = Element::new(shape, material);
                let aabb_element = Element::new(aabb_shape, aabb_material);

                scene.add_element(element);
                scene.add_element(aabb_element);
            }
            "cube" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir");
                let width = get_float_value(&object, "width");
                let color = get_color(&object);

                let shape = Box::new(Cube::new(pos, dir, width));

                let material = get_material(&object, color);
                scene.add_textures(&material);
                
                let element = Element::new(shape, material);

                scene.add_element(element);
            }
            "cubehole" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir");
                let width = get_float_value(&object, "width");
                let color = get_color(&object);

                let shape = Box::new(Cubehole::new(pos, dir, width));

                let material = get_material(&object, color);
                scene.add_textures(&material);
                
                let element = Element::new(shape, material);

                scene.add_element(element);
            }
            "hyperboloid" => {
                let pos = get_coordinates_value(&object, "pos");
                let z_shift = get_float_value(&object, "z_shift");
                let color = get_color(&object);

                let shape = Box::new(Hyperboloid::new(pos, z_shift));

                let material = get_material(&object, color);
                scene.add_textures(&material);

                let element = Element::new(shape, material);

                scene.add_element(element);
            }
            "any" => {
                let equation = get_string_value(&object, "equation");
                let color = get_color(&object);

                let shape = Box::new(Any::new(equation));

                let material = get_material(&object, color);
                scene.add_textures(&material);

                let element = Element::new(shape, material);

                scene.add_element(element);
            }
            "obj" => {
                let mut obj = Obj::new();
                let file = get_string_value(&object, "file");
                let texturefile = get_string_value(&object, "texture");
                let pos = get_coordinates_value(&object, "pos");
                let scale = get_float_value(&object, "scale");
                let dir = get_coordinates_value(&object, "dir");

                obj.set_pos(pos);
                obj.set_dir(dir);
                obj.set_scale(scale);
                obj.set_filepath(file);
                obj.set_texturepath(texturefile);

                scene.add_obj(&mut obj);
            }
            _ => {}
        }
    }

    return scene;
}

fn parse_json(scene_file: String) -> Vec<HashMap<String, String>> {
    let content = std::fs::read_to_string(scene_file).expect("Error reading file");
    let content = content.replace('\t', "    ");
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
	let displacement_string = object.get("displacement").unwrap_or(&default);
    let color_texture = match object.get("color") {
        Some(path) => Texture::Texture(path.clone(), TextureType::Color),
        None => match color_opt {
            Some(color) => Texture::Value(
                Vec3::new(color.r(), color.g(), color.b()),
                TextureType::Color,
            ),
            None => panic!("Color must be provided for non-textured materials"),
        },
    };
    Box::new(Diffuse::new(
        color_texture,
        Texture::from_float_litteral(metalness_string, 0.),
        Texture::from_float_litteral(roughness_string, 0.),
        Texture::from_float_scaled(emissive_string, 0.),
        Texture::from_float_litteral(refraction_string, 0.),
        Texture::from_vector(normal_string, Vec3::new(0., 0., 1.)),
        Texture::from_float_litteral(opacity_string, 1.),
		Texture::from_float_litteral(displacement_string, 0.),
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

fn get_string_value(object: &HashMap<String, String>, key: &str) -> String {
    if object[key].parse::<String>().is_err() {
        error(&(key.to_owned() + " must be a string."));
    }

    object[key]
        .parse::<String>()
        .expect(&("Error parsing ".to_owned() + key))
}