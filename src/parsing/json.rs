use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug)]
pub enum JsonValue {
    String(String),
    Number(f64),
    Bool(bool),
    Object(HashMap<String, JsonValue>),
    Array(Vec<JsonValue>),
    Null
}

pub fn parse_json_file(path: &String) -> Result<JsonValue, String> {
    match read_to_string(path) {
        Ok(mut content) => {
            remove_json_comments(&mut content);
            parse_json_content(&content)
        }
        Err(err) => {
            Err(format!("Failed to open file: {}", err.to_string()))
        }
    }
}

fn remove_json_comments(content: &mut String) {
    let mut new_content = String::new();
    let mut in_string = false;
    let mut in_comment = false;
    let mut in_multiline_comment = false;
    let mut previous_char = '\0';
    for c in content.chars() {
        if !in_comment && !in_multiline_comment {
            new_content.push(c);
        }
        if c == '\"' && !in_comment && !in_multiline_comment {
            in_string = !in_string;
        } else if previous_char == '/' && c == '/' && !in_string && !in_multiline_comment {
            in_comment = true;
            new_content.pop();
            new_content.pop();
        } else if c == '\n' && in_comment {
            in_comment = false;
        } else if previous_char == '/' && c == '*' && !in_string && !in_comment {
            in_multiline_comment = true;
            new_content.pop();
            new_content.pop();
        } else if previous_char == '*' && c == '/' && in_multiline_comment {
            in_multiline_comment = false;
        }
        previous_char = c;
    }
    *content = new_content
}

fn parse_json_content(content: &str) -> Result<JsonValue, String> {
    let content = content.trim();
    if content.starts_with('{') && content.ends_with('}') {
        // it's an object
        let mut object: HashMap<String, JsonValue> = HashMap::new();
        let inner = content[1 .. content.len() - 1].trim();
        let pairs = split_at_comma(inner);
        for pair in pairs {
            let pair = pair.trim();
            if pair.len() > 0 {
                if let Some((key, value_str)) = pair.split_once(':') {
                    let key = key.trim();
                    if !key.starts_with('\"') || !key.ends_with('\"') || key.len() < 2 {
                        return Err(format!("Failed to parse key: {}", key));
                    }
                    let key = key[1..key.len()-1].to_string();
                    match parse_json_content(value_str) {
                        Ok(json_value) => {
                            object.insert(key, json_value);
                        }
                        Err(err) => {
                            return Err(err)
                        }
                    }
                } else {
                    return Err(format!("Failed to parse this pair: {}", pair));
                }
            }
        }
        return Ok(JsonValue::Object(object));
    } else if content.starts_with('[') && content.ends_with(']') {
        // it's an array
        let mut object: Vec<JsonValue> = Vec::new();
        let inner = content[1 .. content.len() - 1].trim();
        let values = split_at_comma(inner);
        for value in &values {
            let value = value.trim();
            if value.len() > 0 {
                match parse_json_content(value) {
                    Ok(json_value) => {
                        object.push(json_value);
                    }
                    Err(err) => {
                        return Err(err)
                    }
                }
            }
        }
        return Ok(JsonValue::Array(object));
    } else if content.starts_with('\"') && content.ends_with('\"') && content.len() > 1 {
        // it's a string
        return Ok(JsonValue::String(content[1..content.len()-1].to_string()));
    } else if let Ok(value) = content.parse::<f64>() {
        // it's a number
        return Ok(JsonValue::Number(value));
    } else if let Ok(value) = content.parse::<bool>() {
        // it's a bool
        return Ok(JsonValue::Bool(value));
    } else if content == "null" {
        // it's a null
        return Ok(JsonValue::Null)
    }
    Err(format!("Failed to recognize type: {}", content))
}

fn split_at_comma(content: &str) -> Vec<String> {
    let mut pairs: Vec<String> = Vec::new();
    let mut token_stack: Vec<char> = Vec::new();
    let mut last_pair_index = 0;
    for (i, c) in content.char_indices() {
        if c == '[' || c == '{' {
            token_stack.push(c);
        } else if c == ']' {
            if let Some(last) = token_stack.last() {
                if last == &'[' {
                    token_stack.pop();
                }
            }
        } else if c == '}' {
            if let Some(last) = token_stack.last() {
                if last == &'{' {
                    token_stack.pop();
                }
            }
        } else if c == ',' {
            if token_stack.len() == 0 {
                pairs.push(content[last_pair_index..i].to_string());
                last_pair_index = i + 1;
            }
        }
    }
    pairs.push(content[last_pair_index..content.len()].to_string());
    pairs
}