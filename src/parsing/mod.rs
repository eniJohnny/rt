use crate::model::materials::{Color, Material};
use crate::model::maths::vec3::Vec3;
use crate::model::Element;
use crate::model::{scene::Scene, shapes::sphere::Sphere, shapes::plane::Plane, shapes::cylinder::Cylinder, shapes::cone::Cone};
use crate::model::objects::{camera::Camera, light::Light, light::AmbientLight};
use std::collections::HashMap;

pub fn get_scene() -> Scene {
    let mut scene = Scene::new();
    let objects = parse_json();

    for object in objects {
        match object["type"].as_str() {
            "sphere" => {
                let pos = get_position(&object);
                let radius = get_radius(&object);
                let color = get_color(&object);
                let dir = get_direction(&object);

                let shape = Box::new(Sphere::new(pos, dir, radius));
                let material = <dyn Material>::new(color);

                let element = Element::new(shape, material);
                scene.add_element(element)
            },
            "plane" => {
                let pos = get_position(&object);
                let dir = get_direction(&object);
                let color = get_color(&object);

                let shape = Box::new(Plane::new(pos, dir));
                let material = <dyn Material>::new(color);

                let element = Element::new(shape, material);
                scene.add_element(element)
            
            },
            "cylinder" => {
                let pos = get_position(&object);
                let radius = get_radius(&object);
                let height = get_height(&object);
                let color = get_color(&object);
                let dir: Vec3 = Vec3::new(1.0, 0.0, 0.0);

                let shape = Box::new(Cylinder::new(pos, dir, radius, height));
                let material = <dyn Material>::new(color);

                let element = Element::new(shape, material);
                scene.add_element(element)
            },
            "cone" => {
                let pos = get_position(&object);
                let radius = get_radius(&object);
                let height = get_height(&object);
                let color = get_color(&object);
                let dir: Vec3 = Vec3::new(1.0, 0.0, 0.0);

                let shape = Box::new(Cone::new(pos, dir, radius, height));
                let material = <dyn Material>::new(color);

                let element = Element::new(shape, material);
                scene.add_element(element)
            },
            "camera" => {
                let pos = get_position(&object);
                let dir = get_direction(&object);
                let fov = object.get("fov").unwrap().parse::<f64>().unwrap();

                let new_camera = Camera::new(pos, dir, fov);
                scene.add_camera(new_camera);
            },
            "light" => {
                let pos = get_position(&object);
                let intensity = get_intensity(&object);
                let color = get_color(&object);

                let new_light = Light::new(pos, intensity, color);
                scene.add_light(new_light);
            },
            "ambient" => {
                let intensity = get_intensity(&object);
                let color = get_color(&object);

                let new_ambient_light = AmbientLight::new(intensity, color);
                scene.add_ambient_light(new_ambient_light);
            },
            _ => {}
        }
    }

    return scene;
}

fn parse_json() -> Vec<HashMap<String, String>> {
    let content = std::fs::read_to_string("scenes/scene.json").unwrap();
    let mut objects: Vec<HashMap<String, String>> = Vec::new();
    let mut i = 0;

    while i < content.len() && content[i..].find('{') != None {
        let mut object: HashMap<String, String> = HashMap::new();
        let remaining = &content[i..];
        let start = remaining.find('{').unwrap();
        let end = remaining.find("\n    }").unwrap() + 6;
        let object_str = &remaining[start..end];
        i += end;

        for prop in object_str.split(",\n        \"") {
            let prop = prop.trim();
            let mut prop = prop.split(": ");
            let key: String = prop.next().unwrap().trim_matches(['"', ' ', '\n', '{', '}']).to_string();
            let value: String = prop.next().unwrap().trim_matches(['{', '"', ' ', '\n', '}']).to_string();

            if value.contains('[') {
                let str = value.trim_matches(['[', ']']).replace(", ", ",");
                let tmp: Vec<&str> = str.split(",").collect();

                if key == "pos" {
                    object.insert("position_x".to_string(), tmp[0].to_string());
                    object.insert("position_y".to_string(), tmp[1].to_string());
                    object.insert("position_z".to_string(), tmp[2].to_string());
                } else if key == "dir" {
                    object.insert("direction_x".to_string(), tmp[0].to_string());
                    object.insert("direction_y".to_string(), tmp[1].to_string());
                    object.insert("direction_z".to_string(), tmp[2].to_string());
                } else if key == "color" {
                    object.insert("color_r".to_string(), tmp[0].to_string());
                    object.insert("color_g".to_string(), tmp[1].to_string());
                    object.insert("color_b".to_string(), tmp[2].to_string());
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

fn get_color(object: &HashMap<String, String>) -> Color {
    let r = object["color_r"].parse::<u8>().unwrap();
    let g = object["color_g"].parse::<u8>().unwrap();
    let b = object["color_b"].parse::<u8>().unwrap();

    return Color::new(r, g, b);
}

fn get_position(object: &HashMap<String, String>) -> Vec3 {
    Vec3::new(
        object["position_x"].parse::<f64>().unwrap(),
        object["position_y"].parse::<f64>().unwrap(),
        object["position_z"].parse::<f64>().unwrap()
    )
}

fn get_direction(object: &HashMap<String, String>) -> Vec3 {
    Vec3::new(
        object["direction_x"].parse::<f64>().unwrap(),
        object["direction_y"].parse::<f64>().unwrap(),
        object["direction_z"].parse::<f64>().unwrap()
    )
}

fn get_radius(object: &HashMap<String, String>) -> f64 {
    object["radius"].parse::<f64>().unwrap()
}

fn get_height(object: &HashMap<String, String>) -> f64 {
    object["height"].parse::<f64>().unwrap()
}

fn get_intensity(object: &HashMap<String, String>) -> f64 {
    object["intensity"].parse::<f64>().unwrap()
}