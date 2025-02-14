use super::shape::Shape;
use super::utils::get_cross_axis;
use std::f64::consts::PI;
use std::sync::{Arc, RwLock};
use crate::{
    model::{
        materials::material::Projection,
        maths::{hit::Hit, ray::Ray, vec3::Vec3},
        scene::Scene,
        element::Element
    },
    ui::{
        prefabs::vector_ui::get_vector_ui,
        ui::UI,
        uielement::{Category, UIElement},
        utils::misc::{ElemType, Property, Value}
    },
    render::raycasting::get_sorted_hit_from_t
};

#[derive(Debug)]
pub struct Sphere {
    pos: Vec3,
    dir: Vec3,
    radius: f64,
    aabb: super::aabb::Aabb,
}

impl Shape for Sphere {
    fn distance(&self, _vec: &Vec3) -> f64 {
        unimplemented!()
    }

    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        // intersection rayon/sphere
        let dist = &self.pos - r.get_pos();
        let dot_product = r.get_dir().dot(&dist);
        let discriminant =
            &dot_product * &dot_product - &dist.dot(&dist) + &self.radius * &self.radius;
        if discriminant < 0.0 {
            return None;
        }
        let intersection1 = &dot_product - &discriminant.sqrt();
        let intersection2 = &dot_product + &discriminant.sqrt();
        return Some(Vec::from([intersection1, intersection2]));
    }

    fn outer_intersect(&self, r: &Ray, displaced_factor: f64) -> Option<Vec<f64>> {
        let mut outer_sphere = self.clone();
        outer_sphere.set_radius(outer_sphere.radius() + outer_sphere.radius() * displaced_factor);
        outer_sphere.intersect(r)
    }

    fn intersect_displacement(&self, ray: &Ray, element: &Element, scene: &Scene) -> Option<Vec<f64>> {
        // Size of the displacement proportional to the radius
        let displaced_factor: f64 = scene.settings().sphere_displaced_distance;
        let step_size: f64 = scene.settings().sphere_displacement_step; // step number ~ 1 / step_size 

        let biggest_sphere_size: f64 = self.radius * displaced_factor;
        let t: Option<Vec<f64>> = self.outer_intersect(ray, displaced_factor);
        if let Some(mut hits) = get_sorted_hit_from_t(scene, ray, &t, element) {
            if hits.len() == 1 {
                return None; // Inside the sphere
            }

            let mut hit = hits.remove(0);
            let second_hit = hits.remove(0);

            let mut old_t = *hit.dist();
            while hit.dist() < second_hit.dist() {
                let sphere_to_hit = hit.pos() - self.pos();
                let hit_distance = sphere_to_hit.length() - self.radius;
                let hit_ratio: f64 = hit_distance / biggest_sphere_size;

                let displaced_ratio = hit.map_texture(element.material().displacement(), scene.textures(), Vec3::from_value(0.)).to_value();
                if (displaced_ratio - hit_ratio).abs() < 0.01 {
                    return Some(vec![*hit.dist()]); // Almost perfect match
                }
                if displaced_ratio >= hit_ratio {
                    return Some(vec![(*hit.dist() + old_t) / 2.]); // Passed the displacement
                }

                old_t = *hit.dist();
                let mut displaced_dist = (hit_ratio - displaced_ratio) * biggest_sphere_size;
                if displaced_dist > step_size * biggest_sphere_size {
                    displaced_dist = step_size * biggest_sphere_size;
                }
                hit = Hit::new(
                    element,
                    hit.dist() + displaced_dist,
                    hit.pos() + ray.get_dir() * displaced_dist,
                    ray.get_dir(),
                    scene.textures(),
                    vec![hit.dist() + displaced_dist]
                );
            }
        }
        None
    }

    fn projection(&self, hit: &Hit) -> Projection {
        let mut projection = Projection::default();
        let constant_axis = get_cross_axis(&self.dir());
        let i = self.dir().cross(&constant_axis).normalize();
        let j = self.dir().cross(&i).normalize();
        projection.k = hit.norm().clone();
        let i_component: f64 = hit.norm().dot(&i);
        let j_component: f64 = hit.norm().dot(&j);
        let k_component: f64 = hit.norm().dot(&self.dir);
        projection.u = (f64::atan2(i_component, j_component) + PI) / (2. * PI);
        projection.v = f64::acos(k_component) / PI;
        projection.i = hit.norm().cross(&self.dir).normalize();
        projection.j = -hit.norm().cross(&projection.i).normalize();
        projection
    }

    fn norm(&self, hit_position: &Vec3) -> Vec3 {
        let norm = (hit_position - self.pos()).normalize();
        norm
    }

    fn as_sphere(&self) -> Option<&Sphere> {
        Some(self)
    }
    fn as_sphere_mut(&mut self) -> Option<&mut Sphere> {
        Some(self)
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn aabb(&self) -> Option<&super::aabb::Aabb> {
        Some(&self.aabb)
    }

    fn get_ui(&self, element: &Element, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Sphere", "sphere", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(sphere) = element.shape().as_sphere() {
            let id = element.id().clone();
            category.add_element(get_vector_ui(sphere.pos.clone(), "Position", "pos", &ui.uisettings_mut(), 
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(sphere) = elem.shape_mut().as_sphere_mut() {
                        if let Value::Float(value) = value {
                            sphere.pos.set_x(value);
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
                    if let Some(sphere) = elem.shape_mut().as_sphere_mut() {
                        if let Value::Float(value) = value {
                            sphere.pos.set_y(value);
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
                    if let Some(sphere) = elem.shape_mut().as_sphere_mut() {
                        if let Value::Float(value) = value {
                            sphere.pos.set_z(value);
                        }
                    }
                }),
                false, None, None));
            category.add_element(get_vector_ui(sphere.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
                Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(sphere) = elem.shape_mut().as_sphere_mut() {
                        if let Value::Float(value) = value {
                            sphere.dir.set_x(value);
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
                    if let Some(sphere) = elem.shape_mut().as_sphere_mut() {
                        if let Value::Float(value) = value {
                            sphere.dir.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, context, ui| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(sphere) = elem.shape_mut().as_sphere_mut() {
                        if let Value::Float(value) = value {
                            sphere.dir.set_z(value);
                        }
                        sphere.set_dir(sphere.dir.normalize());
                        ui.set_dirty();
                    }
                }),
                false, Some(-1.), Some(1.)));

            category.add_element(UIElement::new(
                "Radius",
                "radius", 
                ElemType::Property(Property::new(
                    Value::Float(sphere.radius), 
                    Box::new(move |_, value, context, _| {
                        let scene = match context.active_scene {
                            Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                            None => return,
                        };
                        let mut scene = scene.write().unwrap();
                        let elem = scene.element_mut_by_id(id.clone()).unwrap();
                        if let Some(sphere) = elem.shape_mut().as_sphere_mut() {
                            if let Value::Float(value) = value {
                                sphere.set_radius(value);
                            }
                        }
                        scene.set_dirty(true);
                    }),
                    Box::new(|_, _, _| Ok(())),
                    ui.uisettings())),
                ui.uisettings()));
        }
        category
    }
}

impl Sphere {
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
    pub fn aabb(&self) -> &super::aabb::Aabb {
        &self.aabb
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos;
        self.update_aabb();
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir
    }
    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
        self.update_aabb();
    }
    pub fn set_aabb(&mut self, aabb: super::aabb::Aabb) {
        self.aabb = aabb
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3, radius: f64) -> Sphere {
        let aabb = self::Sphere::compute_aabb(&pos, radius);
        self::Sphere { pos, dir, radius, aabb }
    }

    // Methods
    pub fn clone(&self) -> Sphere {
        let pos = Vec3::new(*self.pos.x(), *self.pos.y(), *self.pos.z());
        let dir = Vec3::new(*self.dir.x(), *self.dir.y(), *self.dir.z());
        self::Sphere {
            pos: pos,
            dir: dir,
            radius: self.radius,
            aabb: self.aabb.clone(),
        }
    }

    fn update_aabb(&mut self) {
        self.aabb.set_x_min(self.pos.x() - self.radius);
        self.aabb.set_x_max(self.pos.x() + self.radius);
        self.aabb.set_y_min(self.pos.y() - self.radius);
        self.aabb.set_y_max(self.pos.y() + self.radius);
        self.aabb.set_z_min(self.pos.z() - self.radius);
        self.aabb.set_z_max(self.pos.z() + self.radius);
    }

    pub fn compute_aabb(pos: &Vec3, radius: f64) -> super::aabb::Aabb {
        super::aabb::Aabb::new(
            pos.x() - radius,
            pos.x() + radius,
            pos.y() - radius,
            pos.y() + radius,
            pos.z() - radius,
            pos.z() + radius,
        )
    }

}

#[cfg(test)]
mod tests {
    use crate::model::maths::ray::Ray;
    use crate::model::maths::vec3::Vec3;
    use crate::model::shapes::sphere::Sphere;
    use crate::model::shapes::shape::Shape;

    #[test]
    fn test_intersect() {
        let s1: Sphere = Sphere::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 0.), 1.);
        let r1: Ray = Ray::new(Vec3::new(-5., 0., 0.), Vec3::new(1., 0., 0.), 5);
        assert_eq!(s1.intersect(&r1), Some(Vec::from([4., 6.])));
    }

    #[test]
    fn test_intersect2() {
        let s1: Sphere = Sphere::new(Vec3::new(0., 0., 2.), Vec3::new(0., 0., 0.), 1.);
        let r1: Ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.), 5);
        assert_eq!(s1.intersect(&r1), Some(Vec::from([1., 3.])));
    }

    #[test]
    fn test_intersect3() {
        let s1: Sphere = Sphere::new(Vec3::new(0., 0., 2.), Vec3::new(0., 0., 0.), 1.);
        let r1: Ray = Ray::new(Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), 5);
        assert_eq!(s1.intersect(&r1), None);
    }
}
