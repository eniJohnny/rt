use super::shape::Shape;
use std::sync::{Arc, RwLock};
use crate::{
    model::{
        materials::material::Projection,
        maths::{hit::Hit, ray::Ray, vec3::Vec3},
        scene::Scene,
        element::Element
    },
    render::raycasting::get_sorted_hit_from_t,
    ui::{
        prefabs::vector_ui::get_vector_ui,
        ui::UI,
        uielement::{Category, UIElement},
        utils::misc::{ElemType, Value}
    }
};

#[derive(Debug, Clone)]
pub struct Plane {
    pos: Vec3,
    dir: Vec3,
}

impl Shape for Plane {
    fn distance(&self, _vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        let dist = &self.pos - r.get_pos();
        let mut dir = self.dir.clone();
        let mut dot_product = r.get_dir().dot(&self.dir);
        if dot_product > 0. {
            dir = -dir;
            dot_product = -dot_product;
        }
        let t = dist.dot(&dir) / dot_product;
        return Some(Vec::from([t]));
    }

	fn outer_intersect(&self, r: &Ray, displaced_factor: f64) -> Option<Vec<f64>> {
		let mut upper_plane = self.clone();
		upper_plane.set_pos(upper_plane.pos() + upper_plane.dir() * displaced_factor);
		let mut t_list: Vec<f64> = Vec::new();
		let t_up = upper_plane.intersect(r);
		if !t_up.is_none() {
			t_list.push(t_up.unwrap()[0]);
		}
		let t_down = self.intersect(r);
		if !t_down.is_none() {
			t_list.push(t_down.unwrap()[0]);
		}
		if t_list.len() == 0 {
			return None;
		}
		Some(t_list)
	}

    fn intersect_displacement(&self, ray: &Ray, element: &Element, scene: &Scene) -> Option<Vec<f64>> {
		let displaced_factor = scene.settings().plane_displaced_distance;
		let total_displacement = displaced_factor;
		let step = scene.settings().plane_displacement_step;
		let t: Option<Vec<f64>> = self.outer_intersect(ray, displaced_factor);
		if let Some(mut hits) = get_sorted_hit_from_t(scene, ray, &t, element) {
			if hits.len() == 1 {
				return None;
			}
			let mut hit = hits.remove(0);
			let sec_hit = hits.remove(0);
			let mut old_t = *hit.dist();
			let mut current_step = 1.;
			while hit.dist() < sec_hit.dist() {
				let hit_ratio: f64 = current_step;

				let displaced_ratio = hit.map_texture(element.material().displacement(), scene.textures(), Vec3::from_value(0.)).to_value();
				if (displaced_ratio - hit_ratio).abs() < 0.01 {
					return Some(vec![*hit.dist()]);
				}
				if displaced_ratio >= hit_ratio {
					return Some(vec![(*hit.dist() + old_t) / 2.]);
				}
				old_t = *hit.dist();
				let mut displaced_dist = (hit_ratio - displaced_ratio) * total_displacement;
				if displaced_dist > step * total_displacement {
					displaced_dist = step * total_displacement ;
				}
				hit = Hit::new(
					element,
					hit.dist() + displaced_dist,
					hit.pos() + ray.get_dir() * displaced_dist,
					ray.get_dir(),
					scene.textures(),
					vec![hit.dist() + displaced_dist]
				);
				current_step += (displaced_dist * ray.get_dir()).dot(&self.dir) / total_displacement;
			}
		}
		None
	}

    fn projection(&self, hit: &Hit) -> Projection {
        let mut projection: Projection = Projection::default();
        let scale = 4.;

        let constant_axis: Vec3;
        if *hit.norm() == Vec3::new(0., 1., 0.) || *hit.norm() == Vec3::new(0., -1., 0.)  {
            constant_axis = Vec3::new(0., 0., 1.);
        } else {
            constant_axis = Vec3::new(0., 1., 0.);
        }
        projection.i = self.dir.cross(&constant_axis).normalize();
        projection.j = - self.dir.cross(&projection.i).normalize();
        projection.k = hit.norm().clone();
        let dist = hit.pos() - self.pos();
        let i_component = dist.dot(&projection.i) / &scale;
        let j_component = - dist.dot(&projection.j) / &scale;
        projection.u = &i_component - (i_component as i32) as f64;
        if projection.u < 0. {
            projection.u += 1.;
        }
        projection.v = &j_component - (j_component as i32) as f64;
        if projection.v < 0. {
            projection.v += 1.;
        }
        projection
    }

    fn norm(&self, _hit_pos: &Vec3) -> Vec3 {
        self.dir.clone()
    }
    fn as_plane(&self) -> Option<&Plane> {
        Some(self)
    }
    fn as_plane_mut(&mut self) -> Option<&mut Plane> {
        Some(self)
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn get_ui(&self, element: &Element, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Plane", "Plane", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(plane) = element.shape().as_plane() {
            let id = element.id().clone();
            category.add_element(get_vector_ui(plane.pos.clone(), "Position", "pos", &ui.uisettings_mut(), 
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(plane) = elem.shape_mut().as_plane_mut() {
                        if let Value::Float(value) = value {
                            plane.pos.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(plane) = elem.shape_mut().as_plane_mut() {
                        if let Value::Float(value) = value {
                            plane.pos.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(plane) = elem.shape_mut().as_plane_mut() {
                        if let Value::Float(value) = value {
                            plane.pos.set_z(value);
                        }
                    }
                }),
                true, None, None));
            category.add_element(get_vector_ui(plane.dir.clone(), "Direction", "dir", &ui.uisettings_mut(),
                Box::new(move |_, value, scene, _ui| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(plane) = elem.shape_mut().as_plane_mut() {
                        if let Value::Float(value) = value {
                            plane.dir.set_x(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(plane) = elem.shape_mut().as_plane_mut() {
                        if let Value::Float(value) = value {
                            plane.dir.set_y(value);
                        }
                    }
                }),
                Box::new(move |_, value, scene, _| {
                    let mut scene = scene.write().unwrap();
                    let elem = scene.element_mut_by_id(id.clone()).unwrap();
                    if let Some(plane) = elem.shape_mut().as_plane_mut() {
                        if let Value::Float(value) = value {
                            plane.dir.set_z(value);
                        }
                    }
                }),
                true, Some(-1.), Some(1.)));
        }
        category
    }
}

impl Plane {
    // Accessors
    pub fn pos(&self) -> &Vec3 {
        &self.pos
    }
    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    // Mutators
    pub fn set_pos(&mut self, pos: Vec3) {
        self.pos = pos
    }
    pub fn set_dir(&mut self, dir: Vec3) {
        self.dir = dir
    }

    // Constructor
    pub fn new(pos: Vec3, dir: Vec3) -> Plane {
        self::Plane { pos, dir }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::maths::ray::Ray;
    use crate::model::maths::vec3::Vec3;
    use crate::model::shapes::shape::Shape;

    #[test]
    fn test_plane_intersect1() {
        let plane = super::Plane::new(Vec3::new(0., 0., 1.), Vec3::new(0., 0., 1.));
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 5);
        assert_eq!(plane.intersect(&ray), Some(Vec::from([1.0])));
    }

    #[test]
    fn test_plane_intersect2() {
        let plane = super::Plane::new(Vec3::new(0., 0., 1.), Vec3::new(0., 0., -1.));
        let ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 5);
        assert_eq!(plane.intersect(&ray), Some(Vec::from([1.0])));
    }
}
