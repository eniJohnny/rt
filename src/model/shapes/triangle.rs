use super::{aabb::Aabb, plane::Plane, shape::Shape};
use std::sync::{Arc, RwLock};
use crate::{
    model::{
        materials::material::Projection,
		maths::{hit::Hit, ray::Ray, vec2::Vec2, vec3::Vec3},
		scene::Scene,
		element::Element
    },
    ui::{
        prefabs::vector_ui::get_vector_ui,
        ui::UI,
        uielement::{Category, UIElement},
        utils::misc::{ElemType, Value}
    }
};

#[derive(Debug)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    a_uv: Vec2,
    b_uv: Vec2,
    c_uv: Vec2,
    dir: Vec3,
    plane: Plane,
    aabb: Aabb,
    is_obj: bool,
}

impl Shape for Triangle {
    fn distance(&self, _vec: &Vec3) -> f64 {
        unimplemented!()
    }
    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        let intersection: f64;
        match self.plane.intersect(r) {
            Some(intersections) => {
                intersection = intersections[0];
            },
            _ => {
                return None;
            }
        }

        let p = intersection * r.get_dir() + r.get_pos();
        if Triangle::inside_triangle(&p, &self.a, &self.b, &self.c, &self.dir) {
            return Some(Vec::from([intersection]));
        }
        None
    }

	fn outer_intersect(&self, r: &Ray, _displaced_factor: f64) -> Option<Vec<f64>> {
		self.intersect(r)
	}

    fn intersect_displacement(&self, ray: &Ray, _element: &Element, _scene: &Scene) -> Option<Vec<f64>> {
		self.intersect(ray)
	}

    fn projection(&self, hit: &Hit) -> Projection {
        if self.is_obj == false {
            return self.plane.projection(hit);
        } else {
            let other_vector;
            if *hit.norm() == Vec3::new(0., 1., 0.) || *hit.norm() == Vec3::new(0., -1., 0.) {
                other_vector = Vec3::new(0., 0., 1.);
            } else {
                other_vector = Vec3::new(0., 1., 0.);
            }
            let uv = self.barycentric_coords(hit.pos());
            let uv = self.interpolate_uv(uv.1, uv.2, uv.3);
            let (u, v) = (*uv.x(), *uv.y());
            let i = self.dir.cross(&other_vector).normalize();
            let j = i.cross(&other_vector).normalize();
            let k = hit.norm().clone();

            let mut projection = Projection::default();
            projection.i = i;
            projection.j = j;
            projection.k = k;
            projection.u = u;
            projection.v = v;

            if projection.u < 0. {
                projection.u = 1. + projection.u;
            }
            if projection.v < 0. {
                projection.v = 1. + projection.v;
            }

            return projection;
        }
    }
    fn norm(&self, hit: &Vec3) -> Vec3 {
        self.plane.norm(hit)
    }
    fn pos(&self) -> &Vec3 { &self.a }
    fn as_triangle(&self) -> Option<&Triangle> { Some(self) }
    fn as_triangle_mut(&mut self) -> Option<&mut Triangle> { Some(self) }
    fn aabb(&self) -> Option<&Aabb> {
        Some(&self.aabb)
    }

    fn get_ui(&self, element: &Element, ui: &mut UI, _scene: &Arc<RwLock<Scene>>) -> UIElement {
        let mut category = UIElement::new("Triangle", "triangle", ElemType::Category(Category::default()), ui.uisettings());

        if let Some(triangle) = element.shape().as_triangle() {
            let id = element.id().clone();
            category.add_element(get_vector_ui(triangle.a.clone(), "Point A", "pA", &ui.uisettings_mut(), 
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.a.set_x(value);
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
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.a.set_y(value);
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
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.a.set_z(value);
                    }
                }
            }),
            true, None, None));
            category.add_element(get_vector_ui(triangle.b.clone(), "Point B", "pB", &ui.uisettings_mut(), 
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.b.set_x(value);
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
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.b.set_y(value);
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
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.b.set_z(value);
                    }
                }
            }),
            true, Some(-1.), Some(1.)));
            category.add_element(get_vector_ui(triangle.c.clone(), "Point C", "pC", &ui.uisettings_mut(), 
            Box::new(move |_, value, context, _| {
                    let scene = match context.active_scene {
                        Some(active_scene_index) => context.scene_list.get(&active_scene_index).unwrap(),
                        None => return,
                    };
                let mut scene = scene.write().unwrap();
                let elem = scene.element_mut_by_id(id.clone()).unwrap();
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.c.set_x(value);
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
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.c.set_y(value);
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
                if let Some(triangle) = elem.shape_mut().as_triangle_mut() {
                    if let Value::Float(value) = value {
                        triangle.c.set_z(value);
                    }
                }
            }),
            true, Some(-1.), Some(1.)));
        }
        category
    }
}

impl Triangle {
    // // Accessors
    pub fn get_a(&self) -> &Vec3 { &self.a }
    pub fn get_b(&self) -> &Vec3 { &self.b }
    pub fn get_c(&self) -> &Vec3 { &self.c }
    pub fn get_a_uv(&self) -> &Vec2 { &self.a_uv }
    pub fn get_b_uv(&self) -> &Vec2 { &self.b_uv }
    pub fn get_c_uv(&self) -> &Vec2 { &self.c_uv }
    pub fn aabb(&self) -> &Aabb { &self.aabb }
    pub fn is_obj(&self) -> bool { self.is_obj }

    // Mutators
    pub fn set_a(&mut self, a: Vec3) {
        self.a = a;
        self.update_aabb();
    }

    pub fn set_b(&mut self, b: Vec3) {
        self.b = b;
        self.update_aabb();
    }

    pub fn set_c(&mut self, c: Vec3) {
        self.c = c;
        self.update_aabb();
    }

    pub fn set_a_uv(&mut self, a_uv: Vec2) {
        self.a_uv = a_uv;
    }

    pub fn set_b_uv(&mut self, b_uv: Vec2) {
        self.b_uv = b_uv;
    }

    pub fn set_c_uv(&mut self, c_uv: Vec2) {
        self.c_uv = c_uv;
    }

    pub fn set_aabb(&mut self, aabb: super::aabb::Aabb) {
        self.aabb = aabb;
    }

    pub fn set_is_obj(&mut self, is_obj: bool) {
        self.is_obj = is_obj;
    }

    pub fn inside_triangle(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3, dir: &Vec3) -> bool {
        let pa = (b - a).cross(&(p - a)).dot(&dir);
        let pb = (c - b).cross(&(p - b)).dot(&dir);
        let pc = (a - c).cross(&(p - c)).dot(&dir);
        if (pa >= 0. && pb >= 0. && pc >= 0.) || (pa <= 0. && pb <= 0. && pc <= 0.) {
            return true;
        }
        false
    }

    // Constructor
    pub fn new(a: Vec3, b: Vec3, c: Vec3 ) -> Triangle {

        let dir =(b.clone() - &a).cross(&(c.clone() - &a)).normalize();
        let plane = Plane::new(a.clone(), dir.clone());
        let aabb = self::Triangle::compute_aabb(&a, &b, &c);
        Triangle {
            a,
            b,
            c,
            dir,
            plane,
            aabb,
            is_obj: false,
            a_uv: Vec2::new(0., 0.),
            b_uv: Vec2::new(0., 0.),
            c_uv: Vec2::new(0., 0.),
        }
    }

    pub fn update_aabb(&mut self) {
        self.set_aabb(self::Triangle::compute_aabb(&self.a, &self.b, &self.c));
    }

    pub fn compute_aabb(a: &Vec3, b: &Vec3, c: &Vec3) -> Aabb {
        let x_min = a.x().min(*b.x()).min(*c.x());
        let x_max = a.x().max(*b.x()).max(*c.x());
        let y_min = a.y().min(*b.y()).min(*c.y());
        let y_max = a.y().max(*b.y()).max(*c.y());
        let z_min = a.z().min(*b.z()).min(*c.z());
        let z_max = a.z().max(*b.z()).max(*c.z());

        super::aabb::Aabb::new(x_min, x_max, y_min, y_max, z_min, z_max)
    }

    pub fn barycentric_coords(&self, p: &Vec3) -> (f64, f64, f64, f64) {
        let (area, alpha, beta, gamma);
        let (a, b, c) = (&self.a, &self.b, &self.c);

        // Compute the area of the triangle
        area = (b - a).cross(&(c - a)).length() / 2.;

        // Compute the barycentric coordinates
        alpha = (b - p).cross(&(c - p)).length() / 2. / area;
        beta = (c - p).cross(&(a - p)).length() / 2. / area;
        gamma = 1. - alpha - beta;

        (area, alpha, beta, gamma)
    }
    
    pub fn interpolate_uv(&self, alpha: f64, beta: f64, gamma: f64) -> Vec2 {
        let (a_uv, b_uv, c_uv) = (&self.a_uv, &self.b_uv, &self.c_uv);
        let u = alpha * a_uv.x() + beta * b_uv.x() + gamma * c_uv.x();
        let v = alpha * a_uv.y() + beta * b_uv.y() + gamma * c_uv.y();
    
        Vec2::new(u, v)
    }

}
