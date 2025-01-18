use super::{shape::Shape, utils::get_cross_axis};
use std::{
    sync::{Arc, RwLock},
    vec
};
use crate::{
    model::{
        materials::material::Projection,
        maths::{hit::Hit, ray::Ray, vec3::Vec3},
        scene::Scene,
        shapes::plane::Plane,
        element::Element
    },
    ui::{
        prefabs::vector_ui::get_vector_ui,
        uielement::{Category, UIElement},
        utils::misc::{ElemType, Property, Value},
        ui::UI
    }
};

#[derive(Debug)]
pub struct Cone {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
    height: f64,
    cos_powed: f64,
    plane: Plane,
    aabb: super::aabb::Aabb,
}

unsafe impl Send for Cone {}

impl Shape for Cone {
    fn distance(&self, _vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        //d:    direction du rayon
        //co:   vecteur entre la postion du cone et le point d'origine du rayon
        //v:    vecteur directeur du cone
        //abc:  les coefficients
        let dv = r.get_dir().dot(&self.dir);
        let co = r.get_pos() - &self.pos;
        let cov = co.dot(&self.dir);
        let a = dv.powi(2) - &self.cos_powed;
        let b = 2.0 * ((dv * cov) - co.dot(&r.get_dir()) * &self.cos_powed);
        let c = cov.powi(2) - co.dot(&(co)) * &self.cos_powed;

        let mut delta = b.powi(2) - 4.0 * a * c;

        if delta < 0.0 {
            return None;
        }
        delta = delta.sqrt();

        //On calcule la distance avec les deux intersections
        let mut intersections = Vec::from([(-b - delta) / (2.0 * a), (-b + delta) / (2.0 * a)]);

        //On vérifie si les intersections sont bien sur le cone (delimité par la hauteur)
        let projection1 = (intersections[0] * r.get_dir() + r.get_pos() - &self.pos).dot(&self.dir);
        let projection2 = (intersections[1] * r.get_dir() + r.get_pos() - &self.pos).dot(&self.dir);

        if (projection2 < 0.0 || projection2 > self.height) || intersections[1] < 0. || delta == 0.
        {
            intersections.remove(1);
        }
        if (projection1 < 0.0 || projection1 > self.height) || intersections[0] < 0. {
            intersections.remove(0);
        }

        //On vérifie si le rayon intersecte le plan du cone
        match self.plane.intersect(r) {
            Some(intersection) => {
                let mut tmin = -1.;
                for t in intersection {
                    tmin = t.min(tmin);
                }
                if tmin >= 0.0 {
                    let position = tmin * r.get_dir() + r.get_pos();
                    let distance = (position - (&self.pos + &self.dir * &self.height)).length();
                    if distance < self.radius {
                        intersections.push(tmin);
                    }
                }
            }
            _ => {
                // Ce bloc sera exécuté pour tous les autres cas, y compris None
            }
        }
        if intersections.len() == 0 {
            return None;
        }

        //On trie et on retourne les intersections
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        return Some(intersections);
    }

    fn outer_intersect(&self, r: &Ray, _displaced_factor: f64) -> Option<Vec<f64>> {
        self.intersect(r)
    }

    fn intersect_displacement(&self, ray: &Ray, _element: &Element, _scene: &Scene) -> Option<Vec<f64>> {
        self.intersect(ray)
    }

    fn projection(&self, hit: &Hit) -> Projection {
        let mut projection: Projection = Projection::default();
        let constant_axis: Vec3;
        if *hit.norm() == Vec3::new(0., 0., 1.) {
            constant_axis = Vec3::new(0., 1., 0.);
        } else {
            constant_axis = Vec3::new(0., 0., 1.);
        }
        projection.i = hit.norm().cross(&constant_axis).normalize();
        projection.j = hit.norm().cross(&projection.i).normalize();
        projection.k = hit.norm().clone();

        let point_to_hit = hit.pos() - &self.pos;
        let level = point_to_hit.dot(&self.dir);

        let slope_lenght = (self.height.powi(2) + self.radius.powi(2)).sqrt();
        let total_height = slope_lenght + self.radius;

        let constant_axis: Vec3;
        if self.dir == Vec3::new(0., 0., 1.) {
            constant_axis = Vec3::new(0., 1., 0.);
        } else {
            constant_axis = Vec3::new(0., 0., 1.);
        }
        let cylinder_i = self.dir.cross(&constant_axis).normalize();
        let cylinder_j = self.dir.cross(&cylinder_i).normalize();

        let i_component: f64 = point_to_hit.dot(&cylinder_i);
        let j_component: f64 = point_to_hit.dot(&cylinder_j);
        let ij_hit: Vec3 = (i_component * &cylinder_i + j_component * &cylinder_j).normalize();
        let is_front: bool = ij_hit.dot(&cylinder_j) > 0.;
        if is_front {
            projection.u = (ij_hit.dot(&cylinder_i) + 1.) / 4.;
        } else {
            projection.u = 1. - (ij_hit.dot(&cylinder_i) + 1.) / 4.;
        }
        if level > self.height - 0.000001 && level < self.height + 0.000001 {
            // Cap
            projection.v =
                (total_height - (point_to_hit - &self.dir * &self.height).length()) / total_height;
        } else {
            // Cone
            projection.v = point_to_hit.length() / total_height;
        }
        projection
    }

    fn norm(&self, hit_position: &Vec3) -> Vec3 {
        let pc = hit_position - &self.pos;
        let coef = pc.dot(&self.dir) / pc.dot(&pc);
        let projection = &pc * coef;

        if pc.dot(&self.dir) == self.height {
            return self.plane.norm(hit_position);
        }

        return ((&self.pos + &projection) - (&self.pos + &self.dir * &self.height)).normalize();
    }
    fn as_cone(&self) -> Option<&Cone> {
        Some(self)
    }
    fn as_cone_mut(&mut self) -> Option<&mut Cone> {
        Some(self)
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn aabb(&self) -> Option<&super::aabb::Aabb> {
        Some(&self.aabb)
    }
    
    fn get_ui(&self, element: &Element, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Cone", "cone", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(cone) = element.shape().as_cone() {
            let id = element.id().clone();
            category.add_element(get_vector_ui(cone.pos.clone(), "Position", "pos", &ui.uisettings_mut(), 
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cone) = elem.shape_mut().as_cone_mut() {
                        if let Value::Float(value) = value {
                            cone.pos.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cone) = elem.shape_mut().as_cone_mut() {
                        if let Value::Float(value) = value {
                            cone.pos.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cone) = elem.shape_mut().as_cone_mut() {
                        if let Value::Float(value) = value {
                            cone.pos.set_z(value);
                        }
                    }
                }),
                true, None, None));
            category.add_element(get_vector_ui(cone.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cone) = elem.shape_mut().as_cone_mut() {
                        if let Value::Float(value) = value {
                            cone.dir.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cone) = elem.shape_mut().as_cone_mut() {
                        if let Value::Float(value) = value {
                            cone.dir.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(cone) = elem.shape_mut().as_cone_mut() {
                        if let Value::Float(value) = value {
                            cone.dir.set_z(value);
                        }
                    }
                }),
                true, Some(-1.), Some(1.)));
            category.add_element(UIElement::new(
                "Radius",
                "radius", 
                ElemType::Property(Property::new(
                    Value::Float(cone.radius), 
                    Box::new(move |_, value, context, _| {
                        let scene = match context.active_scene {
                            Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                            None => return,
                        };
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(cone) = elem.shape_mut().as_cone_mut() {
                            if let Value::Float(value) = value {
                                cone.set_radius(value);
                            }
                        }
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));

            category.add_element(UIElement::new(
                "Height",
                "height", 
                ElemType::Property(Property::new(
                    Value::Float(cone.height), 
                    Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(cone) = elem.shape_mut().as_cone_mut() {
                            if let Value::Float(value) = value {
                                cone.set_height(value);
                            }
                        }
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));
        }

        category
    }
}

impl Cone {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
    pub fn height(&self) -> f64 {
        self.height
    }
    pub fn aabb(&self) -> &super::aabb::Aabb {
        &self.aabb
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.update_aabb();
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir;
        self.update_aabb();
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
        self.update_aabb();
    }
    pub fn set_height(&mut self, height: f64) {
        self.height = height;
        self.update_aabb();
    }
    pub fn set_aabb(&mut self, aabb: super::aabb::Aabb) {
        self.aabb = aabb;
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64, height: f64) -> Cone {
        let cos_powed = (radius / height).atan().cos().powi(2);
        let plane = Plane::new(&pos + &dir * height, dir.clone());
        let aabb = Cone::compute_aabb(pos.clone(), dir.clone(), height, radius);
        self::Cone {
            pos,
            dir,
            radius,
            height,
            cos_powed,
            plane,
            aabb,
        }
    }

    fn update_aabb(&mut self) {
        self.aabb = Cone::compute_aabb(self.pos.clone(), self.dir.clone(), self.height, self.radius);
    }

    fn get_min(array: &Vec<f64>) -> f64 {
        let mut min = array[0];
        for i in 1..array.len() {
            if array[i] < min {
                min = array[i];
            }
        }
        min
    }

    fn get_max(array: &Vec<f64>) -> f64 {
        let mut max = array[0];
        for i in 1..array.len() {
            if array[i] > max {
                max = array[i];
            }
        }
        max
    }

    pub fn compute_aabb(pos:Vec3, dir: Vec3, height: f64, radius: f64) -> super::aabb::Aabb {
        let dir = dir.normalize();
        let i = get_cross_axis(&dir);
        let j = dir.cross(&i).normalize();

        let apex = &pos;
        let base_center = &pos + &dir * height;

        let sample_nb = 16;
        let mut x_array = vec![*apex.x()];
        let mut y_array = vec![*apex.y()];
        let mut z_array = vec![*apex.z()];

        for k in 0..sample_nb {
            let angle = 2.0 * std::f64::consts::PI * (k as f64) / (sample_nb as f64);
            let x_offset = radius * angle.cos();
            let y_offset = radius * angle.sin();
            let point = &base_center + &i * x_offset + &j * y_offset;

            x_array.push(*point.x());
            y_array.push(*point.y());
            z_array.push(*point.z());
        }

        super::aabb::Aabb::new(
            Cone::get_min(&x_array),
            Cone::get_max(&x_array),
            Cone::get_min(&y_array),
            Cone::get_max(&y_array),
            Cone::get_min(&z_array),
            Cone::get_max(&z_array),
        )
    }
}