use eqsolver::single_variable::FDNewton;
use meval::{self, eval_str_with_context};

use super::Shape;
use crate::model::materials::material::Projection;
use crate::model::maths::{hit::Hit, ray::Ray, vec3::Vec3};

#[derive(Debug)]
pub struct Any {
    equation: String,
    pos: Vec3,
}

impl Shape for Any {
    fn distance(&self, _vec: &Vec3) -> f64 {
        unimplemented!()
    }

    fn intersect(&self, r: &Ray) -> Option<Vec<f64>> {
        // Ray/Any intersection
        let f = |t: f64| -> f64 {
            let p = r.get_pos() + r.get_dir() * t;
            let mut ctx = meval::Context::new();
            ctx.var("x", *p.x())
                .var("z", *p.y())
                .var("y", *p.z());

            eval_str_with_context(&self.equation, ctx).unwrap_or(-1.0)
        };

        let solution = FDNewton::new(f).solve(0.5);
        let mut t_array: Vec<f64> = Vec::new();
        
        match solution.ok() {
            Some(t) => 
                if t > 0.0 {
                    t_array.push(t);
                },
            None => {},
        }


        if t_array.len() > 0 {
            t_array.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(t_array)
        } else {
            None
        }
    }

    fn projection(&self, hit: &Hit) -> Projection {
        let mut projection: Projection = Projection::default();

        let constant_axis: Vec3;
        if *hit.norm() == Vec3::new(0., 1., 0.) {
            constant_axis = Vec3::new(1., 0., 0.);
        } else {
            constant_axis = Vec3::new(0., 1., 0.);
        }
        projection.i = hit.norm().cross(&constant_axis).normalize();
        projection.j = hit.norm().cross(&projection.i).normalize();
        projection.k = hit.norm().clone();
        
        let normalized_pos = hit.pos().normalize();
        let (x, y, z) = (*normalized_pos.x(), *normalized_pos.y(), *normalized_pos.z());

        if x.abs() > y.abs() && x.abs() > z.abs() {
            projection.u = z;
            projection.v = y;
        } else if y.abs() > x.abs() && y.abs() > z.abs() {
            projection.u = x;
            projection.v = z;
        } else {
            projection.u = x;
            projection.v = y;
        }

        if projection.u < 0. {
            projection.u += 1.;
        }
        if projection.v < 0. {
            projection.v += 1.;
        }

        projection
    }

    fn norm(&self, hit_position: &Vec3, _ray_dir: &Vec3) -> Vec3 {
        let f = |x: f64, y: f64, z: f64| -> f64 {
            let mut ctx = meval::Context::new();
            ctx.var("x", x)
                .var("z", y)
                .var("y", z);

            eval_str_with_context(&self.equation, ctx).unwrap_or(-1.0)
        };

        let gradient = Any::gradient(&f, *hit_position);
        gradient.normalize()
    }

    fn as_any(&self) -> Option<&Any> {
        Some(self)
    }

    fn pos(&self) -> &Vec3 {
        &self.pos
    }

    fn outer_intersect(&self, ray: &Ray, _displaced_factor: f64) -> Option<Vec<f64>> {
        self.intersect(ray)
    }

    fn intersect_displacement(&self, ray: &Ray, _element: &crate::model::Element,_scenee: &crate::model::scene::Scene) -> Option<Vec<f64>> {
        self.intersect(ray)
    }

    fn get_ui(&self, _element: &crate::model::Element, _ui: &mut crate::ui::ui::UI, _scene: &std::sync::Arc<std::sync::RwLock<crate::model::scene::Scene>>) -> crate::ui::uielement::UIElement {
        todo!()
    }
}

impl Any {
    // Accessors
    pub fn equation(&self) -> &String {
        &self.equation
    }

    // Mutators
    pub fn set_equation(&mut self, equation: String) {
        self.equation = equation;
    }

    // Constructor
    pub fn new(equation: String) -> Any {
        self::Any { equation, pos: Vec3::new(0.0, 0.0, 0.0) }
    }

    // Methods
    pub fn clone(&self) -> Any {
        self::Any {
            equation: self.equation.clone(),
            pos: self.pos.clone(),
        }
    }

    fn gradient(f: &dyn Fn(f64, f64, f64) -> f64, hit: Vec3) -> Vec3 {
        let delta = 1e-6;
        let x = *hit.x();
        let y = *hit.y();
        let z = *hit.z();


        let df_dx = (f(x + delta, y, z) - f(x - delta, y, z)) / (2.0 * delta);
        let df_dy = (f(x, y + delta, z) - f(x, y - delta, z)) / (2.0 * delta);
        let df_dz = (f(x, y, z + delta) - f(x, y, z - delta)) / (2.0 * delta);
    
        Vec3::new(df_dx, df_dy, df_dz)
    }
}