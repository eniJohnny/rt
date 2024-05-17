use crate::model::materials::metal::Metal;
use crate::model::materials::{Color, Material};
use crate::model::maths::vec3::Vec3;
use crate::model::objects::camera::Camera;
use crate::model::objects::light::{AmbientLight, Light, ParallelLight, PointLight};
use crate::model::Element;
use crate::model::{
    scene::Scene, shapes::cone::Cone, shapes::cylinder::Cylinder, shapes::plane::Plane,
    shapes::sphere::Sphere,
};
use crate::error;
// use crate::{error, SCENE};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::io::Write;

pub fn print_scene(scene: &Scene) {
    write!(std::io::stdout(), "{:#?}\n", scene).expect("Error printing scene");
}

pub fn get_scene(scene_file: &String) -> Scene {
    let mut scene = Scene::new();
    let objects = parse_json(scene_file.clone());

    for object in objects {
        match object["type"].as_str() {
            "sphere" => {
                let pos = get_position(&object);
                let radius = get_radius(&object);
                let color = get_color(&object);
                let dir = get_direction(&object);

                let shape = Box::new(Sphere::new(pos, dir, radius));
                let material = get_material(&object, color);

                let element = Element::new(shape, material);
                scene.add_element(element)
            }
            "plane" => {
                let pos = get_position(&object);
                let dir = get_direction(&object);
                let color = get_color(&object);

                let shape = Box::new(Plane::new(pos, dir));
                let material = get_material(&object, color);

                let element = Element::new(shape, material);
                scene.add_element(element)
            }
            "cylinder" => {
                let pos = get_position(&object);
                let radius = get_radius(&object);
                let height = get_height(&object);
                let color = get_color(&object);
                let dir: Vec3 = get_direction(&object);

                let shape = Box::new(Cylinder::new(pos, dir, radius, height));
                let material = get_material(&object, color);

                let element = Element::new(shape, material);
                scene.add_element(element)
            }
            "cone" => {
                let pos = get_position(&object);
                let radius = get_radius(&object);
                let height = get_height(&object);
                let color = get_color(&object);
                let dir: Vec3 = get_direction(&object);

                let shape = Box::new(Cone::new(pos, dir, radius, height));
                let material = get_material(&object, color);

                let element = Element::new(shape, material);
                scene.add_element(element)
            }
            "camera" => {
                let pos = get_position(&object);
                let dir = get_direction(&object);
                let fov = get_fov(&object) * 2. * PI / 360.;

                let new_camera = Camera::new(pos, dir, fov);
                scene.add_camera(new_camera);
            }
            "light" => {
                let pos = get_position(&object);
                let intensity = get_intensity(&object);
                let color = get_color(&object);

                let new_light = Box::new(PointLight::new(pos, intensity, color))
                    as Box<dyn Light + Sync + Send>;
                scene.add_light(new_light);
            }
            "ambient" => {
                let intensity = get_intensity(&object);
                let color = get_color(&object);

                let new_ambient_light = AmbientLight::new(intensity, color);
                scene.add_ambient_light(new_ambient_light);
            }
            "parallel" => {
                let intensity = get_intensity(&object);
                let dir = get_direction(&object);
                let color = get_color(&object);

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

    return Color::new(r as f64 / 255., g as f64 / 255., b as f64 / 255.);
}

fn get_position(object: &HashMap<String, String>) -> Vec3 {
    // Testing if the position is in the format [x, y, z]
    let pos_str = [
        &object["position_x"],
        &object["position_y"],
        &object["position_z"],
    ];

    for i in 0..3 {
        if pos_str[i].parse::<f64>().is_err() {
            error("Position must be in the format [x, y, z] where x, y, z are floats.");
        }
    }

    Vec3::new(
        object["position_x"]
            .parse::<f64>()
            .expect("Error parsing position"),
        object["position_y"]
            .parse::<f64>()
            .expect("Error parsing position"),
        object["position_z"]
            .parse::<f64>()
            .expect("Error parsing position"),
    )
}

fn get_direction(object: &HashMap<String, String>) -> Vec3 {
    // Testing if the direction is in the format [x, y, z]
    let dir = [
        &object["direction_x"],
        &object["direction_y"],
        &object["direction_z"],
    ];

    for i in 0..3 {
        if dir[i].parse::<f64>().is_err() {
            error("Direction must be in the format [x, y, z] where x, y, z are floats.");
        }
    }

    Vec3::new(
        dir[0].parse::<f64>().expect("Error parsing direction"),
        dir[1].parse::<f64>().expect("Error parsing direction"),
        dir[2].parse::<f64>().expect("Error parsing direction"),
    )
    .normalize()
}

fn get_material(object: &HashMap<String, String>, color: Color) -> Box<dyn Material + Sync + Send> {
    let default: &String = &"0.0".to_string();
    let metalness_string = object.get("metalness").unwrap_or(default);
    let roughness_string = object.get("roughness").unwrap_or(default);

    let mut metalness = 0.0;
    let mut roughness = 0.0;

    if let Ok(value) = metalness_string.parse::<f64>() {
        metalness = value;
    }

    if let Ok(value) = roughness_string.parse::<f64>() {
        roughness = value;
    }

    Box::new(Metal::new(color, metalness, roughness))

    // if let Some(mat_str) = object.get("metalness") {
    //     if let Ok(metalness) = mat_str.parse::<f64>() {
    //         Box::new(Metal::new(color, metalness))
    //     } else {
    //         error("Metalness must be a float");
    //         Box::new(Unicolor::from(color))
    //     }
    // } else {
    //     Box::new(Unicolor::from(color))
    // }
}

fn get_radius(object: &HashMap<String, String>) -> f64 {
    // Testing if the radius is a float
    if object["radius"].parse::<f64>().is_err() {
        error("Radius must be a float.");
    }

    object["radius"]
        .parse::<f64>()
        .expect("Error parsing radius")
}

fn get_height(object: &HashMap<String, String>) -> f64 {
    // Testing if the height is a float
    if object["height"].parse::<f64>().is_err() {
        error("Height must be a float.");
    }

    object["height"]
        .parse::<f64>()
        .expect("Error parsing height")
}

fn get_intensity(object: &HashMap<String, String>) -> f64 {
    // Testing if the intensity is a float
    if object["intensity"].parse::<f64>().is_err() {
        error("Intensity must be a float.");
    }

    object["intensity"]
        .parse::<f64>()
        .expect("Error parsing intensity")
}

fn get_fov(object: &HashMap<String, String>) -> f64 {
    // Testing if the fov is a float
    if object["fov"].parse::<f64>().is_err() {
        error("Field of view must be a float.");
    }

    object["fov"].parse::<f64>().expect("Error parsing fov")
}
