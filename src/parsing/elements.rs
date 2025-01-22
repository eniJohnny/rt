use std::{collections::HashMap, f64::consts::PI};
use crate::model::{composed_element::ComposedElement, element::Element, materials::{diffuse::Diffuse, material::Material}, objects::{camera::Camera, lights::{ambient_light::AmbientLight, light::AnyLight, parallel_light::ParallelLight, point_light::PointLight, spot_light::SpotLight}}, shapes::{any::Any, brick::Brick, composed_shape::ComposedShape, cone::Cone, cube::Cube, cubehole::Cubehole, cylinder::Cylinder, ellipse::Ellipse, helix::Helix, hyperboloid::Hyperboloid, mobius::Mobius, nagone::Nagone, obj::Obj, plane::Plane, rectangle::Rectangle, sphere::Sphere, torus::Torus, torusphere::Torusphere, triangle::Triangle}};
use super::{
    basic::{
        get_color, get_color_texture, get_displacement_texture, get_normal_texture, get_number, get_opacity_texture, get_string, get_vec1_texture, get_vec3
    },
    json::JsonValue
};

pub fn get_material(json_object: &HashMap<String, JsonValue>) -> Result<Box<dyn Material + Sync + Send>, String> {
    let color = get_color_texture(json_object)?;
    let metalness = get_vec1_texture(json_object, "metalness", Some(0.), Some(1.), 0.)?;
    let roughness = get_vec1_texture(json_object, "roughness", Some(0.), Some(1.), 1.)?;
    let emissive = get_vec1_texture(json_object, "emissive", Some(0.), None, 0.)?;
    let transparency = get_vec1_texture(json_object, "transparency", Some(0.), Some(1.), 0.)?;
    let norm_variation = get_normal_texture(json_object)?;
    let opacity = get_opacity_texture(json_object)?;
    let displacement = get_displacement_texture(json_object)?;
    let refraction = get_number(json_object, "refraction", Some(1.), None, Some(1.))?;
    let emissive_intensity = get_number(json_object, "emissive_intensity", Some(0.), None, Some(1.))?;
    let u_scale = get_number(json_object, "u_scale", None, None, Some(1.))?;
    let v_scale = get_number(json_object, "v_scale", None, None, Some(1.))?;
    let u_shift = get_number(json_object, "u_shift", None, None, Some(0.))?;
    let v_shift = get_number(json_object, "v_shift", None, None, Some(0.))?;

    Ok(Box::new(Diffuse::new(
        color,
        metalness,
        roughness,
        emissive,
        emissive_intensity,
        transparency,
        norm_variation,
        opacity,
        displacement,
        refraction,
        u_scale,
        v_scale,
        u_shift,
        v_shift,
    )))
}

pub fn get_camera(json_camera: &HashMap<String, JsonValue>) -> Result<Camera, String> {
    let pos = get_vec3(json_camera, "pos", None, None, None)?;
    let dir = get_vec3(json_camera, "dir", None, None, None)?.normalize();
    let fov = get_number(json_camera, "fov", Some(0.), Some(360.), None)? * PI / 180.;
    Ok(Camera::new(pos, dir, fov))
}

pub fn get_light(json_light: &HashMap<String, JsonValue>) -> Result<AnyLight, String> {
    let pos = get_vec3(json_light, "pos", None, None, None)?;
    let intensity = get_number(json_light, "intensity", Some(0.), None, None)?;
    let color = get_color(json_light, "color")?;
    Ok(AnyLight::new(Box::new(PointLight::new(pos, intensity, color))))
}

pub fn get_ambient(json_ambient: &HashMap<String, JsonValue>) -> Result<AmbientLight, String> {
    let intensity = get_number(json_ambient, "intensity", Some(0.), None, None)?;
    let color = get_color(json_ambient, "color")?;
    Ok(AmbientLight::new(intensity, color))
}

pub fn get_parallel(json_parralel: &HashMap<String, JsonValue>) -> Result<AnyLight, String> {
    let dir = get_vec3(json_parralel, "dir", None, None, None)?.normalize();
    let intensity = get_number(json_parralel, "intensity", Some(0.), None, None)?;
    let color = get_color(json_parralel, "color")?;
    Ok(AnyLight::new(Box::new(ParallelLight::new(dir, intensity, color))))
}

pub fn get_spot(json_spot: &HashMap<String, JsonValue>) -> Result<AnyLight, String> {
    let pos = get_vec3(json_spot, "pos", None, None, None)?;
    let dir = get_vec3(json_spot, "dir", None, None, None)?.normalize();
    let intensity = get_number(json_spot, "intensity", Some(0.), None, None)?;
    let fov = get_number(json_spot, "fov", Some(0.), Some(360.), None)?;
    let color = get_color(json_spot, "color")?;
    Ok(AnyLight::new(Box::new(SpotLight::new(pos, dir, intensity, color, fov))))
}

pub fn get_sphere(json_sphere: &HashMap<String, JsonValue>) -> Result<Element, String> {
    let pos = get_vec3(&json_sphere, "pos", None, None, None)?;
    let dir = get_vec3(&json_sphere, "dir", None, None, None)?.normalize();
    let radius = get_number(&json_sphere, "radius", Some(0.), None, None)?;

    let shape = Box::new(Sphere::new(pos, dir, radius));
    let material = get_material(&json_sphere)?;
    let element = Element::new(shape, material);
    Ok(element)
}

pub fn get_plane(json_plane: &HashMap<String, JsonValue>) -> Result<Element, String> {
    let pos = get_vec3(&json_plane, "pos", None, None, None)?;
    let dir = get_vec3(&json_plane, "dir", None, None, None)?.normalize();

    let shape = Box::new(Plane::new(pos, dir));
    let material = get_material(&json_plane)?;
    let element = Element::new(shape, material);
    Ok(element)
}

pub fn get_cylinder(json_cylinder: &HashMap<String, JsonValue>) -> Result<Element, String> {
    let pos = get_vec3(&json_cylinder, "pos", None, None, None)?;
    let dir = get_vec3(&json_cylinder, "dir", None, None, None)?.normalize();
    let radius = get_number(&json_cylinder, "radius", Some(0.), None, None)?;
    let height = get_number(&json_cylinder, "height", Some(0.), None, None)?;

    let shape = Box::new(Cylinder::new(pos, dir, radius, height));
    let material = get_material(&json_cylinder)?;
    let element = Element::new(shape, material);
    Ok(element)
}

pub fn get_cone(json_cone: &HashMap<String, JsonValue>) -> Result<Element, String> {
    let pos = get_vec3(&json_cone, "pos", None, None, None)?;
    let dir = get_vec3(&json_cone, "dir", None, None, None)?.normalize();
    let radius = get_number(&json_cone, "radius", Some(0.), None, None)?;
    let height = get_number(&json_cone, "height", Some(0.), None, None)?;

    let shape = Box::new(Cone::new(pos, dir, radius, height));
    let material = get_material(&json_cone)?;
    let element = Element::new(shape, material);
    Ok(element)
}

pub fn get_triangle(json_triangle: &HashMap<String, JsonValue>) -> Result<Element, String> {
    let pos1 = get_vec3(&json_triangle, "a", None, None, None)?;
    let pos2 = get_vec3(&json_triangle, "b", None, None, None)?;
    let pos3 = get_vec3(&json_triangle, "c", None, None, None)?;

    let shape = Box::new(Triangle::new(pos1, pos2, pos3));
    let material = get_material(&json_triangle)?;
    let element = Element::new(shape, material);
    Ok(element)
}

pub fn get_rectangle(json_rectangle: &HashMap<String, JsonValue>) -> Result<Element, String> {
    let pos = get_vec3(&json_rectangle, "pos", None, None, None)?;
    let length = get_number(&json_rectangle, "length", Some(0.), None, None)?;
    let width = get_number(&json_rectangle, "width", Some(0.), None, None)?;
    let dir_l = get_vec3(&json_rectangle, "dir_l", None, None, None)?.normalize();
    let dir_w = get_vec3(&json_rectangle, "dir_w", None, None, None)?.normalize();
    let one_sided = get_number(&json_rectangle, "one_sided", None, None, Some(0.))?;

    let shape = Box::new(Rectangle::new(pos, length, width, dir_l, dir_w, one_sided > 0.));
    let material = get_material(&json_rectangle)?;
    let element = Element::new(shape, material);
    Ok(element)
}

pub fn get_torus(json_torus: &HashMap<String, JsonValue>) -> Result<Element, String> {
    let pos = get_vec3(&json_torus, "pos", None, None, None)?;
    let dir = get_vec3(&json_torus, "dir", None, None, None)?.normalize();
    let radius = get_number(&json_torus, "radius", Some(0.), None, None)?;
    let radius2 = get_number(&json_torus, "radius2", Some(0.), None, None)?;
    let material = get_material(&json_torus)?;

    let shape = Box::new(Torus::new(pos, dir, radius, radius2));
    let element = Element::new(shape, material);
    Ok(element)
}

pub fn get_torusphere(json_torusphere: &HashMap<String, JsonValue>) -> Result<ComposedElement, String> {
    let pos = get_vec3(&json_torusphere, "pos", None, None, None)?;
    let dir = get_vec3(&json_torusphere, "dir", None, None, None)?.normalize();
    let radius = get_number(&json_torusphere, "radius", Some(0.), None, None)?;
    let steps = get_number(&json_torusphere, "steps", Some(0.), None, None)? as usize;
    let material = get_material(&json_torusphere)?;

    let shape = Box::new(Torusphere::new(pos, dir, radius, steps)) as Box<dyn ComposedShape + Sync + Send>;
    let composed_element = ComposedElement::new(shape, material);
    Ok(composed_element)
}

pub fn get_helix(json_helix: &HashMap<String, JsonValue>) -> Result<ComposedElement, String> {
    let pos = get_vec3(&json_helix, "pos", None, None, None)?;
    let dir = get_vec3(&json_helix, "dir", None, None, None)?.normalize();
    let height = get_number(&json_helix, "height", Some(0.), None, None)?;
    let material = get_material(&json_helix)?;

    let shape = Box::new(Helix::new(pos, dir, height)) as Box<dyn ComposedShape + Sync + Send>;
    let composed_element = ComposedElement::new(shape, material);
    Ok(composed_element)
}

pub fn get_brick(json_brick: &HashMap<String, JsonValue>) -> Result<ComposedElement, String> {
    let pos = get_vec3(&json_brick, "pos", None, None, None)?;
    let dir = get_vec3(&json_brick, "dir", None, None, None)?.normalize();
    let dimensions = get_vec3(&json_brick, "dimensions", Some(0.), None, None)?;
    let material = get_material(&json_brick)?;

    let shape = Box::new(Brick::new(pos, dir, dimensions)) as Box<dyn ComposedShape + Sync + Send>;
    let composed_element = ComposedElement::new(shape, material);
    Ok(composed_element)
}

pub fn get_nagone(json_nagone: &HashMap<String, JsonValue>) -> Result<ComposedElement, String> {
    let pos = get_vec3(&json_nagone, "pos", None, None, None)?;
    let dir = get_vec3(&json_nagone, "dir", None, None, None)?.normalize();
    let radius = get_number(&json_nagone, "radius", Some(0.), None, None)?;
    let angles = get_number(&json_nagone, "angles", Some(0.), None, None)? as usize;
    let material = get_material(&json_nagone)?;

    let shape = Box::new(Nagone::new(pos, dir, radius, angles)) as Box<dyn ComposedShape + Sync + Send>;
    let composed_element = ComposedElement::new(shape, material);
    Ok(composed_element)
}

pub fn get_mobius(json_mobius: &HashMap<String, JsonValue>) -> Result<ComposedElement, String> {
    let pos = get_vec3(&json_mobius, "pos", None, None, None)?;
    let radius = get_number(&json_mobius, "radius", Some(0.), None, None)?;
    let half_width = get_number(&json_mobius, "half_width", Some(0.), None, None)?;
    let material = get_material(&json_mobius)?;

    let shape = Box::new(Mobius::new(pos, radius, half_width)) as Box<dyn ComposedShape + Sync + Send>;
    let composed_element = ComposedElement::new(shape, material);
    Ok(composed_element)
}

pub fn get_ellipse(json_ellipse: &HashMap<String, JsonValue>) -> Result<Element, String> {
    let pos = get_vec3(&json_ellipse, "pos", None, None, None)?;
    let dir = get_vec3(&json_ellipse, "dir", None, None, None)?.normalize();
    let u = get_number(&json_ellipse, "u", None, None, None)?;
    let v = get_number(&json_ellipse, "v", None, None, None)?;

    let shape = Box::new(Ellipse::new(pos, dir, u, v));
    let material = get_material(&json_ellipse)?;
    let element = Element::new(shape, material);
    Ok(element)
}

pub fn get_cube(json_cube: &HashMap<String, JsonValue>) -> Result<Element, String> {
    let pos = get_vec3(&json_cube, "pos", None, None, None)?;
    let dir = get_vec3(&json_cube, "dir", None, None, None)?.normalize();
    let width = get_number(&json_cube, "width", Some(0.), None, None)?;

    let shape = Box::new(Cube::new(pos, dir, width));
    let material = get_material(&json_cube)?;
    let element = Element::new(shape, material);
    Ok(element)
}


pub fn get_cubehole(json_cube: &HashMap<String, JsonValue>) -> Result<Element, String> {
    let pos = get_vec3(&json_cube, "pos", None, None, None)?;
    let dir = get_vec3(&json_cube, "dir", None, None, None)?.normalize();
    let width = get_number(&json_cube, "width", Some(0.), None, None)?;

    let shape = Box::new(Cubehole::new(pos, dir, width));
    let material = get_material(&json_cube)?;
    let element = Element::new(shape, material);
    Ok(element)
}

pub fn get_hyperboloid(json_hyperboloid: &HashMap<String, JsonValue>) -> Result<Element, String> {
    let pos = get_vec3(&json_hyperboloid, "pos", None, None, None)?;
    let z_shift = get_number(&json_hyperboloid, "z_shift", Some(0.), None, None)?;

    let shape = Box::new(Hyperboloid::new(pos, z_shift));
    let material = get_material(&json_hyperboloid)?;
    let element = Element::new(shape, material);
    Ok(element)
}

pub fn get_any(json_any: &HashMap<String, JsonValue>) -> Result<Element, String> {
    let equation = get_string(&json_any, "equation", None)?;

    let shape = Box::new(Any::new(equation));
    let material = get_material(&json_any)?;
    let element = Element::new(shape, material);
    Ok(element)
}

pub fn get_obj(json_obj: &HashMap<String, JsonValue>) -> Result<ComposedElement, String> {
    let pos = get_vec3(&json_obj, "pos", None, None, None)?;
    let dir = get_vec3(&json_obj, "dir", None, None, None)?.normalize();
    let scale = get_number(&json_obj, "scale", None, None, Some(1.))?;
    let rotation = get_number(&json_obj, "rotation", None, None, Some(0.))?;
    let file = get_string(&json_obj, "file", None)?;

    let mut obj = Obj::new(pos, dir, rotation, scale, file.clone());
    if let Err(err) = obj.parse_file() {
        return Err(format!("{}: {}", file, err));
    }

    let shape = Box::new(obj);
    let material = get_material(&json_obj)?;
    let element = ComposedElement::new(shape, material);
    Ok(element)
}