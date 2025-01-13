use std::f64::consts::PI;
use crate::model::{materials::{color::Color, material::Projection, texture::{self, Texture}}, maths::{ray::Ray, vec3::Vec3}, scene::Scene, shapes::{shape::Shape, sphere::{self, Sphere}}};

pub fn get_skybox_color(scene: &Scene, ray: &Ray) -> Color {
    match scene.skybox() {
        Texture::Texture(file, _) => {
            let sphere = sphere::Sphere::new(Vec3::new(0., 0., 0.), Vec3::new(0., 1., 0.), 1.);
            let hit_norm = sphere.norm(ray.get_dir());
            let projection = skybox_projection(&hit_norm, &sphere);
            if let Some(img) = scene.get_texture(file) {
                let color = texture::Texture::get(&projection, &img);
                &color * &color
            } else {
                Color::new(0., 0., 0.)
            }
        }
        Texture::Value(vector, _) => {
            Color::from_vec3(vector)
        }
    }
}

fn skybox_projection(hit_norm: &Vec3, sphere: &Sphere) -> Projection {
    let mut projection: Projection = Projection::default();

    let constant_axis: Vec3;
    if *hit_norm == Vec3::new(0., 0., 1.) {
        constant_axis = Vec3::new(0., 1., 0.);
    } else {
        constant_axis = Vec3::new(0., 0., 1.);
    }
    projection.v = (sphere.dir().dot(&hit_norm) + 1.) / 2.;
    projection.i = hit_norm.cross(&constant_axis).normalize();
    projection.j = hit_norm.cross(&projection.i).normalize();
    projection.k = hit_norm.clone();

    let constant_axis: Vec3;
    if *sphere.dir() == Vec3::new(0., 0., 1.) {
        constant_axis = Vec3::new(0., 1., 0.);
    } else {
        constant_axis = Vec3::new(0., 0., 1.);
    }
    projection.v = ((sphere.dir().dot(&hit_norm) + 1.) / 2.).clamp(0., 1.);
    projection.i = sphere.dir().cross(&constant_axis).normalize();
    projection.j = sphere.dir().cross(&projection.i).normalize();
    projection.k = hit_norm.clone();
    
    let i_component: f64 = hit_norm.dot(&projection.i);
    let j_component: f64 = hit_norm.dot(&projection.j);
    let k_component: f64 = hit_norm.dot(&sphere.dir());
    projection.u = (f64::atan2(i_component, j_component) + PI) / (2. * PI);
    projection.v = f64::acos(k_component) / PI;
    projection.i = hit_norm.cross(&sphere.dir()).normalize();
    projection.j = hit_norm.cross(&projection.i).normalize();
    projection
}