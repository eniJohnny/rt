use std::collections::HashMap;

use crate::{
    bvh::{self},
    model::objects::light::AmbientLight,
    parsing::obj::Obj,
    render::settings::Settings
};
use super::{
    materials::{
        diffuse::Diffuse,
        material::Material,
        texture::{Texture, TextureType}
    },
    maths::vec3::Vec3,
    objects::{camera::Camera, light::AnyLight},
    shapes::{self, aabb::Aabb, triangle::Triangle},
    ComposedElement, Element
};

#[derive(Debug)]
pub struct Scene {
    elements: Vec<Element>,
    non_bvh_elements_index: Vec<usize>,
    non_bvh_composed_elements_index: Vec<usize>,
    composed_elements: Vec<ComposedElement>,
    camera: Camera,
    lights: Vec<AnyLight>,
    ambient_light: AmbientLight,
    settings: Settings,
    textures: HashMap<String, image::RgbaImage>,
    dirty: bool,
    bvh: Option<bvh::node::Node>,
    next_element_id: u32,
    next_composed_element_id: u32
}

impl Scene {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            non_bvh_elements_index: Vec::new(),
            non_bvh_composed_elements_index: Vec::new(),
            composed_elements: Vec::new(),
            camera: Camera::default(),
            lights: Vec::new(),
            ambient_light: AmbientLight::default(),
            settings: Settings::default(),
            textures: HashMap::new(),
            dirty: true,
            bvh: None,
            next_element_id: 0,
            next_composed_element_id: 0
        }
    }

    // Adders
    pub fn add_element(&mut self, mut element: Element) {
        element.set_id(self.next_element_id);
        self.elements.push(element);
        self.next_element_id += 1;
    }

    pub fn add_composed_element(&mut self, mut composed_element: ComposedElement) {
        for element in composed_element.composed_shape_mut().elements_as_mut() {
            element.set_id(self.next_element_id);
            element.set_composed_id(self.next_composed_element_id);
            self.next_element_id += 1;
        }

        composed_element.set_id(self. next_composed_element_id);
        self.composed_elements.push(composed_element);
        self.next_composed_element_id += 1;
    }

    pub fn add_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn add_light(&mut self, mut light: AnyLight) {
        light.set_id(self.next_element_id);
        self.lights.push(light);
        self.next_element_id += 1;
    }

    pub fn add_ambient_light(&mut self, ambient_light: AmbientLight) {
        self.ambient_light = ambient_light;
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut Settings {
        &mut self.settings
    }

    pub fn load_texture(&mut self, path: &str) {
        if path == "" || path.contains("..") {
            panic!("Textures should only be stored in the textures folder.");
        }
        if !self.textures.contains_key(path) {
            let path_str = String::from("./textures/") + path;
            self.textures.insert(
                path.to_string(),
                match image::open(&path_str) {
                    Ok(img) => img.to_rgba8(),
                    Err(_) => panic!("Error opening texture file {}", path),
                },
            );
        }
    }

    pub fn load_material_textures(&mut self, material: &Box<dyn Material + Sync + Send>) {
        let textures = [
            material.color(),
            material.roughness(),
            material.metalness(),
            material.norm(),
            material.emissive(),
            material.opacity(),
			material.displacement(),
        ];
        for texture in textures.iter() {
            match texture {
                Texture::Value(..) => {}
                Texture::Texture(path, _) => {
                    self.load_texture(path);
                }
            }
        }
    }

    pub fn add_obj(&mut self, obj: &mut Obj) {
        // let mut obj = Obj::new();
        let path = obj.filepath().clone();
        let result = obj.parse_file(path.clone());

        match result {
            Ok(_) => {
                let len = obj.faces.len();

                for i in 0..len {
                    let face = &obj.faces[i];
                    let mut vertices: Vec<Vec3> = vec![];
                    let mut normals: Vec<Vec3> = vec![];
                    let mut textures: Vec<Vec3> = vec![];
                    let modulo = obj.params_number();
        
                    for j in 0..face.len() {
                        match j % modulo {
                            0 => vertices.push(face[j]),
                            1 => textures.push(face[j]),
                            2 => normals.push(face[j]),
                            _ => {}
                        }
                    }        

                    for k in 0..obj.triangle_count()[i] {
                        let material = obj.material.clone();
                        let (a, b, c) = (vertices[0], vertices[k + 1], vertices[k + 2]);
                        let (a_uv, b_uv, c_uv) = (textures[0].xy(), textures[k + 1].xy(), textures[k + 2].xy());

                        let mut triangle = Triangle::new(a, b, c);
                        triangle.set_is_obj(true);
                        triangle.set_a_uv(a_uv);
                        triangle.set_b_uv(b_uv);
                        triangle.set_c_uv(c_uv);

                        let elem = Element::new(Box::new(triangle), material);
                        self.add_element(elem);
                    }

                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
    
    pub fn add_wireframes(&mut self) {
        let aabbs = self.all_aabb();
        let mut new_elements = vec![];
        for aabb in aabbs {
            let mut new_material = Diffuse::default();
            new_material.set_emissive(Texture::Value(Vec3::from_value(0.0), TextureType::Float));

            let new_shape = shapes::wireframe::Wireframe::from_aabb(aabb);
            let new_element = Element::new(Box::new(new_shape), new_material);

            new_elements.push(new_element);
        }
        self.elements.append(&mut new_elements);
    }

    pub fn update_bvh(&mut self) {
        let aabbs = self.all_aabb();
        let biggest_aabb = Aabb::from_aabbs(&aabbs);
        let mut node = bvh::node::Node::new(&biggest_aabb);
        node.build_tree(self);

        self.non_bvh_elements_index.clear();
        self.non_bvh_composed_elements_index.clear();
        let mut nb_elements = 0;
        for element in &self.elements {
            if element.shape().aabb().is_none() {
                self.non_bvh_elements_index.push(nb_elements);
            }
            nb_elements += 1;
        }
        let mut nb_elements = 0;
        for composed_element in &self.composed_elements {
            if composed_element.composed_shape().elements().iter().all(|element| element.shape().aabb().is_none()) {
                self.non_bvh_composed_elements_index.push(nb_elements);
            }
            nb_elements += 1;
        }

        self.bvh = Some(node);
    }

    // Accessors
    pub fn elements(&self) -> &Vec<Element> {
        &self.elements
    }

    pub fn elements_as_mut(&mut self) -> &mut Vec<Element> {
        &mut self.elements
    }

    pub fn composed_elements(&self) -> &Vec<ComposedElement> {
        &self.composed_elements
    }

    pub fn composed_elements_as_mut(&mut self) -> &mut Vec<ComposedElement> {
        &mut self.composed_elements
    }

    pub fn element_by_id(&self, id: u32) -> Option<&Element> {
        for element in &self.elements {
            if element.id == id {
                return Some(element);
            }
        }
        None
    }
    pub fn element_mut_by_id(&mut self, id: u32) -> Option<&mut Element> {
        for element in &mut self.elements {
            if element.id == id {
                return Some(element);
            }
        }
        None
    }

    pub fn composed_element_by_id(&self, id: u32) -> Option<&ComposedElement> {
        for element in &self.composed_elements {
            if element.id == id {
                return Some(element);
            }
        }
        None
    }
    pub fn composed_element_mut_by_id(&mut self, id: u32) -> Option<&mut ComposedElement> {
        for element in &mut self.composed_elements {
            if element.id == id {
                return Some(element);
            }
        }
        None
    }
    
    pub fn camera(&self) -> &Camera {
        &self.camera
    }
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn lights(&self) -> &Vec<AnyLight> {
        &self.lights
    }

    pub fn ambient_light(&self) -> &AmbientLight {
        &self.ambient_light
    }

    pub fn textures(&self) -> &HashMap<String, image::RgbaImage> {
        &self.textures
    }

    pub fn get_texture(&self, name: &str) -> Option<&image::RgbaImage> {
        self.textures.get(name)
    }
    
    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }
    
    pub fn get_element(&self, index: usize) -> &Element {
        &self.elements[index]
    }

    pub fn all_aabb(&self) -> Vec<&crate::model::shapes::aabb::Aabb> {
        self.elements
            .iter()
            .filter_map(|element| element.shape().as_aabb().or_else(|| element.shape.aabb()) )
            .collect()
    }

    pub fn bvh(&self) -> &Option<bvh::node::Node> {
        &self.bvh
    }

    pub fn non_bvh_elements(&self) -> &Vec<usize> {
        &self.non_bvh_elements_index
    }

    pub fn non_bvh_composed_elements(&self) -> &Vec<usize> {
        &self.non_bvh_composed_elements_index
    }

    pub fn non_bvh_element_ids(&self) -> &Vec<usize> {
        &self.non_bvh_elements_index
    }

    pub fn test_all_elements(&self) -> Vec<&crate::model::Element> {
        self.elements
            .iter()
            .collect()
    }

    pub fn get_next_element_id(&self) -> u32 {
        self.next_element_id
    }
    pub fn get_next_composed_element_id(&self) -> u32 {
        self.next_composed_element_id
    }

    // Mutators

    pub fn set_elements(&mut self, elements: Vec<Element>) {
        self.elements = elements;
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn set_lights(&mut self, lights: Vec<AnyLight>) {
        self.lights = lights;
    }

    pub fn set_ambient_light(&mut self, ambient_light: AmbientLight) {
        self.ambient_light = ambient_light;
    }

    pub fn set_bvh(&mut self, bvh: Option<bvh::node::Node>) {
        self.bvh = bvh;
    }

    pub fn set_next_element_id(&mut self, id: u32) {
        self.next_element_id = id;
    }
    pub fn set_next_composed_element_id(&mut self, id: u32) {
        self.next_composed_element_id = id;
    }

    // TESTING - GET COMPOSED ELEMENTS UI
    pub fn is_composed_element(&self, id: u32) -> Option<u32> {
        self.composed_elements().iter().find(|composed_element| composed_element.composed_shape().elements().iter().any(|element| element.id() == id)).map(|composed_element| composed_element.id())
    }
}
