use crate::model::Scene;
use std::collections::HashMap;

pub fn getScene() -> Scene {
    let scene = Scene;
    let objects = parse_json();

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

                if key == "position" {
                    object.insert("position_x".to_string(), tmp[0].to_string());
                    object.insert("position_y".to_string(), tmp[1].to_string());
                    object.insert("position_z".to_string(), tmp[2].to_string());
                } else if key == "direction" {
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