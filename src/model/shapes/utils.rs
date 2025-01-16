use crate::model::maths::vec3::Vec3;

pub fn get_cross_axis(dir: &Vec3) -> Vec3 {
    if *dir == Vec3::new(0.0, 1.0, 0.0) || *dir == Vec3::new(0.0, -1.0, 0.0) {
        return Vec3::new(1.0, 0.0, 0.0);
    } else {
        return Vec3::new(0.0, 1.0, 0.0);
    }
}

pub fn get_u_v_from_normal(normal: &Vec3) -> (Vec3, Vec3) {
    let normal = normal.normalize();
    let axis = get_cross_axis(&normal).normalize();
    let v = normal.cross(&axis).normalize();
    let u = v.cross(&normal).normalize();
    (u, v)
}

pub fn get_min_max_multiple_vec3(vec: &Vec<Vec3>) -> (Vec3, Vec3) {
    let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for v in vec {
        min = get_min_two_vec3(&min, v);
        max = get_max_two_vec3(&max, v);
    }
    (min, max)
}

pub fn get_min_two_vec3(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.x().min(*v2.x()),
        v1.y().min(*v2.y()),
        v1.z().min(*v2.z())
    )
}

pub fn get_max_two_vec3(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(
        v1.x().max(*v2.x()),
        v1.y().max(*v2.y()),
        v1.z().max(*v2.z())
    )
}