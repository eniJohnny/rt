use std::{collections::HashMap, ops::SubAssign};

use crate::{
    bvh::{self}, model::objects::light::AmbientLight, render::settings::Settings
};
use super::{
    composed_element::ComposedElement, element::Element, materials::{
        diffuse::Diffuse,
        material::Material,
        texture::{Texture, TextureType}
    }, maths::vec3::Vec3, objects::{camera::Camera, light::AnyLight}, shapes::{self, aabb::Aabb}
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
    next_element_id: usize,
    next_composed_element_id: usize,
    paused: bool
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
            paused: false,
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
        composed_element.set_id(self.next_composed_element_id);
        let elements = composed_element.composed_shape().generate_elements((*composed_element.material()).clone());
        let mut elements_index = composed_element.elements_index_mut().clone();
        elements_index.clear();
        for mut element in elements {
            element.set_composed_id(composed_element.id());
            element.set_id(self.next_element_id);
            elements_index.push(self.next_element_id);
            self.elements.push(element);
            self.next_element_id += 1;
        }
        composed_element.set_elements_index(elements_index);
        self.composed_elements.push(composed_element);
        self.next_composed_element_id += 1;
    }

    pub fn update_composed_element_shape(&mut self, composed_id: usize) {
        let material = (*self.composed_elements[composed_id].material()).clone();
        let mut new_elements = self.composed_elements[composed_id].composed_shape().generate_elements(material);
        let new_nb_elem = new_elements.len();
        let mut composed_ids: Vec<usize> = vec![];
        let ids = self.composed_elements[composed_id].elements_index().clone();

        // Remove old elements
        for id in ids {
            for element in &self.elements {
                if element.id() == id {
                    let index = self.elements.iter().position(|e| e.id() == id).unwrap();
                    self.elements.remove(index);
                    break;
                }
            }
        }

        // Defrag (remove gaps so that they are all consecutive) the ids
        self.defrag_next_element_id();

        // Add new elements with new ids
        for _ in 0..new_nb_elem {
            // Compute next element id (first available id)
            self.compute_next_element_id();

            let id = self.next_element_id;
            let mut element = new_elements.remove(0);
            element.set_composed_id(composed_id);
            element.set_id(id);
            composed_ids.push(id);
            self.elements.push(element);
        }

        // Update composed element with new ids
        self.composed_elements[composed_id].set_elements_index(composed_ids);
    }

    pub fn defrag_next_element_id(&mut self) {
        let mut prev_id = 0;
        for i in 0..self.elements.len() {
            let expected_id = (prev_id + 1) * (i != 0) as usize;
            if self.elements[i].id() != expected_id {
                if let Some(composed_id) = self.elements[i].composed_id() {
                    let composed_element = &mut self.composed_elements[composed_id];
                    let elements_index = composed_element.elements_index_mut();
                    for index in elements_index {
                        if *index == self.elements[i].id() {
                            *index = expected_id;
                        }
                    }
                }
                self.elements[i].set_id(expected_id);
            }
            prev_id = expected_id;
        }
    }
    pub fn compute_next_element_id(&mut self) {
        let ids = self.elements.iter().map(|e| e.id()).collect::<Vec<usize>>();
        let mut id = 0;
        while ids.contains(&id) {
            id += 1;
        }
        self.next_element_id = id;
    }

    pub fn update_composed_element_material(&mut self, composed_id: usize) {
        if let Some(composed_element) = self.composed_elements.get(composed_id) {
            let material = (*composed_element.material()).clone();

            for index in composed_element.elements_index() {
                let element = &mut self.elements[*index];
                element.set_material(material.clone());
            }
        }
    }

    pub fn remove_element(&mut self, index_to_remove: usize) {
        for i in (index_to_remove + 1)..self.elements.len() {
            let element = &mut self.elements[i];
            element.set_id(i - 1);
        }
        for composed_element in &mut self.composed_elements {
            let elements_index = composed_element.elements_index_mut();
            for index in elements_index {
                if *index > index_to_remove {
                    index.sub_assign(1);
                }
            }
        }
        self.next_element_id -= 1;
    }

    pub fn add_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn add_light(&mut self, light: AnyLight) {
        self.lights.push(light);
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
        if path == "" {
            return;
        }
        if path.contains("..") {
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
    pub fn remove_texture(&mut self, name: String) {
        self.textures.remove(&name);
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

    /**
     * If we have a single composed objects with refraction enabled, we need to do a full bvh traversal to get every intersection.
     * This is to ensure that we have the good refraction indices, otherwise the bvh will stop at the first found intersection, and we
     * might not now if we are inside or outside an object.
     */
    pub fn determine_full_bvh_traversal(&mut self) {
        let mut has_transparent_composed_objects = false;
        for composed_element in &self.composed_elements {
            match composed_element.material().transparency() {
                Texture::Texture(_, _) => has_transparent_composed_objects = true,
                Texture::Value(vector_value, _) => has_transparent_composed_objects = vector_value.to_value() > f64::EPSILON
            }
            if has_transparent_composed_objects {
                break;
            }
        }
        self.settings.bvh_full_traversal = has_transparent_composed_objects;
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
    
    pub fn remove_wireframes(&mut self) {
        let mut to_remove = vec![];
        for (i, element) in self.elements.iter().enumerate() {
            if element.shape().as_wireframe().is_some() {
                to_remove.push(i);
            }
        }
        for i in to_remove.iter().rev() {
            self.elements.remove(*i);
        }
    }

    pub fn update_bvh(&mut self) {
        let aabbs = self.all_aabb();
        let biggest_aabb = Aabb::from_aabbs(&aabbs);
        let mut node = bvh::node::Node::new(&biggest_aabb);
        node.build_tree(self);

        self.non_bvh_elements_index.clear();
        let mut nb_elements = 0;
        for element in &self.elements {
            if element.shape().aabb().is_none() {
                self.non_bvh_elements_index.push(nb_elements);
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

    pub fn element_by_id(&self, id: usize) -> Option<&Element> {
        for element in &self.elements {
            if element.id() == id {
                return Some(element);
            }
        }
        None
    }
    pub fn element_mut_by_id(&mut self, id: usize) -> Option<&mut Element> {
        for element in &mut self.elements {
            if element.id() == id {
                return Some(element);
            }
        }
        None
    }

    pub fn composed_element_by_element_id(&self, id: usize) -> Option<&ComposedElement> {
        if let Some(element) = self.element_by_id(id) {
            if let Some(composed_id) = element.composed_id() {
                return self.composed_element_by_id(composed_id);
            }
        }
        None
    }

    pub fn composed_element_mut_by_element_id(&mut self, id:usize) -> Option<&mut ComposedElement> {
        if let Some(element) = self.element_by_id(id) {
            if let Some(composed_id) = element.composed_id() {
                return self.composed_element_mut_by_id(composed_id);
            }
        }
        None
    }

    pub fn composed_element_by_id(&self, id: usize) -> Option<&ComposedElement> {
        for element in &self.composed_elements {
            if element.id() == id {
                return Some(element);
            }
        }
        None
    }
    pub fn composed_element_mut_by_id(&mut self, id: usize) -> Option<&mut ComposedElement> {
        for element in &mut self.composed_elements {
            if element.id() == id {
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

    pub fn paused(&self) -> bool {
        self.paused
    }
    
    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }
    
    pub fn get_element(&self, index: usize) -> &Element {
        &self.elements[index]
    }

    pub fn all_aabb(&self) -> Vec<&crate::model::shapes::aabb::Aabb> {
        self.elements
            .iter()
            .filter_map(|element| element.shape().as_aabb().or_else(|| element.shape().aabb()) )
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
}
