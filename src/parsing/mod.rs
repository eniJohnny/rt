use std::{
    collections::HashMap,
    f64::consts::PI,
    io::Write,
    ops::Add
};
use crate::{
    error, model::{
        materials::{
            color::Color,
		    diffuse::Diffuse,
		    material::Material,
		    texture::{Texture, TextureType}
        }, maths::vec3::Vec3, objects::{
            camera::Camera, light::{AmbientLight, AnyLight, ParallelLight, PointLight}
        }, scene::Scene, shapes::{ 
            any::Any, brick::Brick, cone::Cone, cube::Cube, cubehole::Cubehole, cylinder::Cylinder, ellipse::Ellipse, helix::Helix, hyperboloid::Hyperboloid, mobius::Mobius, nagone::Nagone, obj::Obj, plane::Plane, rectangle::Rectangle, sphere::Sphere, torusphere::Torusphere, triangle::Triangle, composed_shape::ComposedShape
        }, composed_element::ComposedElement, element::Element
    }, AABB_OPACITY
};

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
                scene.load_material_textures(&material);
                let mut aabb_material = get_material(&object, Some(Color::new(255., 255., 255.)));
                aabb_material.set_opacity(Texture::Value(Vec3::from_value(AABB_OPACITY), TextureType::Float));
                
                let element = Element::new(shape, material);

                scene.add_element(element);
            }
            "plane" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir");
                let color = get_color(&object);

                let shape = Box::new(Plane::new(pos, dir));
                let material = get_material(&object, color);
                scene.load_material_textures(&material);
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
                scene.load_material_textures(&material);
                let mut aabb_material = get_material(&object, Some(Color::new(255., 255., 255.)));
                aabb_material.set_opacity(Texture::Value(Vec3::from_value(AABB_OPACITY), TextureType::Float));

                let element = Element::new(shape, material);

                scene.add_element(element);
            }
            "cone" => {
                let pos = get_coordinates_value(&object, "pos");
                let radius = get_float_value(&object, "radius");
                let height = get_float_value(&object, "height");
                let color = get_color(&object);
                let dir: Vec3 = get_coordinates_value(&object, "dir");

                let shape = Box::new(Cone::new(pos, dir, radius, height));

                let material = get_material(&object, color);
                let mut aabb_material = get_material(&object, Some(Color::new(255., 255., 255.)));
                aabb_material.set_opacity(Texture::Value(Vec3::from_value(AABB_OPACITY), TextureType::Float));
                scene.load_material_textures(&material);

                let element = Element::new(shape, material);
                scene.add_element(element);
            }
            "triangle" => {
                let a = get_coordinates_value(&object, "a");
                let b = get_coordinates_value(&object, "b");
                let c = get_coordinates_value(&object, "c");
                let color = get_color(&object);

                let shape = Box::new(Triangle::new(a,b,c));

                let material = get_material(&object, color);
                let mut aabb_material = get_material(&object, Some(Color::new(255., 255., 255.)));
                aabb_material.set_opacity(Texture::Value(Vec3::from_value(AABB_OPACITY), TextureType::Float));
                
                scene.load_material_textures(&material);

                let element = Element::new(shape, material);

                scene.add_element(element);
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
                let mut aabb_material = get_material(&object, Some(Color::new(255., 255., 255.)));
                aabb_material.set_opacity(Texture::Value(Vec3::from_value(AABB_OPACITY), TextureType::Float));
                
                scene.load_material_textures(&material);

                let element = Element::new(shape, material);

                scene.add_element(element);
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
                let color = get_color(&object);

                let material = get_material(&object, color);
                scene.load_material_textures(&material);

                let torusphere = Torusphere::new(pos, dir, radius, steps);
                let composed_shape = Box::new(torusphere) as Box<dyn ComposedShape + Sync + Send>;
                let composed_element = ComposedElement::new(composed_shape, material);
                scene.add_composed_element(composed_element);
            }
            "helix" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir");
                let height = get_float_value(&object, "height");
                let color = get_color(&object);

                let material = get_material(&object, color);
                scene.load_material_textures(&material);

                let helix = Helix::new(pos, dir, height);
                let composed_shape = Box::new(helix) as Box<dyn ComposedShape + Sync + Send>;
                let composed_element = ComposedElement::new(composed_shape, material);
                scene.add_composed_element(composed_element);
            }
            "brick" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir");
                let dimensions = get_coordinates_value(&object, "dimensions");
                let color = get_color(&object);

                let material = get_material(&object, color);
                scene.load_material_textures(&material);

                let brick = Brick::new(pos, dir, dimensions);
                let composed_shape = Box::new(brick) as Box<dyn ComposedShape + Sync + Send>;
                let composed_element = ComposedElement::new(composed_shape, material);
                scene.add_composed_element(composed_element);
            }
            "nagone" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir");
                let radius = get_float_value(&object, "radius");
                let angles = get_float_value(&object, "angles") as usize;
                let color = get_color(&object);

                let material = get_material(&object, color);
                scene.load_material_textures(&material);

                let nagone = Nagone::new(pos, dir, radius, angles);
                let composed_shape = Box::new(nagone) as Box<dyn ComposedShape + Sync + Send>;
                let composed_element = ComposedElement::new(composed_shape, material);
                scene.add_composed_element(composed_element);
            }
            "mobius" => {
                let pos = get_coordinates_value(&object, "pos");
                let radius = get_float_value(&object, "radius");
                let half_width = get_float_value(&object, "half_width");
                let color = get_color(&object);

                let material = get_material(&object, color);
                scene.load_material_textures(&material);

                let mobius = Mobius::new(pos, radius, half_width);
                let composed_shape = Box::new(mobius) as Box<dyn ComposedShape + Sync + Send>;
                let composed_element = ComposedElement::new(composed_shape, material);
                scene.add_composed_element(composed_element);
            }
            "ellipse" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir").normalize();
                let u = get_float_value(&object, "u");
                let v = get_float_value(&object, "v");
                let color = get_color(&object);

                let shape = Box::new(Ellipse::new(pos, dir, u, v));

                let material = get_material(&object, color);
                scene.load_material_textures(&material);
                let mut aabb_material = get_material(&object, Some(Color::new(255., 255., 255.)));
                aabb_material.set_opacity(Texture::Value(Vec3::from_value(AABB_OPACITY), TextureType::Float));
                
                let element = Element::new(shape, material);

                scene.add_element(element);
            }
            "cube" => {
                let pos = get_coordinates_value(&object, "pos");
                let dir = get_coordinates_value(&object, "dir");
                let width = get_float_value(&object, "width");
                let color = get_color(&object);

                let shape = Box::new(Cube::new(pos, dir, width));

                let material = get_material(&object, color);
                scene.load_material_textures(&material);
                
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
                scene.load_material_textures(&material);
                
                let element = Element::new(shape, material);

                scene.add_element(element);
            }
            "hyperboloid" => {
                let pos = get_coordinates_value(&object, "pos");
                let z_shift = get_float_value(&object, "z_shift");
                let color = get_color(&object);

                let shape = Box::new(Hyperboloid::new(pos, z_shift));

                let material = get_material(&object, color);
                scene.load_material_textures(&material);

                let element = Element::new(shape, material);

                scene.add_element(element);
            }
            "any" => {
                let equation = get_string_value(&object, "equation");
                let color = get_color(&object);

                let shape = Box::new(Any::new(equation));

                let material = get_material(&object, color);
                scene.load_material_textures(&material);

                let element = Element::new(shape, material);

                scene.add_element(element);
            }
            "obj" => {
                let file = get_string_value(&object, "file");
                let pos = get_coordinates_value_or(&object, "pos", Some(Vec3::new(0., 0., 0.)));
                let scale = get_float_value_or(&object, "scale", Some(1.));
                let rotation = get_float_value_or(&object, "rotation", Some(0.));
                let dir = get_coordinates_value_or(&object, "dir", Some(Vec3::new(0., 1., 0.)));
                let color = get_color(&object);

                let material = get_material(&object, color);
                scene.load_material_textures(&material);

                let mut obj = Obj::new(pos, dir, rotation, scale, file);

                let result = obj.parse_file();
                if result.is_err() {
                    error(&result.err().unwrap().to_string());
                }

                let composed_shape = Box::new(obj);
                let composed_element = ComposedElement::new(composed_shape, material);
                scene.add_composed_element(composed_element);
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
    if !object.contains_key("color_r") || !object.contains_key("color_g") || !object.contains_key("color_b"){
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
    get_coordinates_value_or(object, key, None)
}

fn get_coordinates_value_or(object: &HashMap<String, String>, key: &str, default: Option<Vec3>) -> Vec3 {
    if !object.contains_key(&(key.to_owned() + "_x")) && !object.contains_key(&(key.to_owned() + "_y")) && !object.contains_key(&(key.to_owned() + "_z")) {
        return default.expect(&format!("{} must be provided.", key));
    }
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
    let transparency_string = object.get("transparency").unwrap_or(&default);
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
            None => Texture::Texture("default.jpg".to_string(), TextureType::Color),
        },
    };
    let refraction = match refraction_string.parse::<f64>() {
        Ok(value) => value,
        _ => 1.
    };
    Box::new(Diffuse::new(
        color_texture,
        Texture::from_float_litteral(metalness_string, 0.),
        Texture::from_float_litteral(roughness_string, 0.),
        Texture::from_float_scaled(emissive_string, 0.),
        Texture::from_float_litteral(transparency_string, 0.),
        Texture::from_vector(normal_string, Vec3::new(0., 0., 1.)),
        Texture::from_float_litteral(opacity_string, 1.),
		Texture::from_float_litteral(displacement_string, 0.),
        refraction,
    ))
}

fn get_float_value(object: &HashMap<String, String>, key: &str) -> f64 {
    get_float_value_or(object, key, None)
}

fn get_float_value_or(object: &HashMap<String, String>, key: &str, default: Option<f64>) -> f64 {
    if !object.contains_key(key) {
        return default.expect(&format!("{}  was not found", key));
    }
    let str_value = object.get(key).unwrap();

    return str_value.parse::<f64>().expect(&format!("{}  doesn't have a correct float value.", key));
}

fn get_string_value(object: &HashMap<String, String>, key: &str) -> String {
    return object.get(key).expect(&format!("{}  was not found", key)).to_string();
}