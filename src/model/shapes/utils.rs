use crate::model::maths::vec3::Vec3;

pub fn get_cross_axis(dir: &Vec3) -> Vec3 {
    if *dir == Vec3::new(1.0, 0.0, 0.0) || *dir == Vec3::new(-1.0, 0.0, 0.0) {
        return Vec3::new(0.0, 1.0, 0.0);
    } else if *dir == Vec3::new(0.0, 1.0, 0.0) || *dir == Vec3::new(0.0, -1.0, 0.0) {
        return Vec3::new(1.0, 0.0, 0.0);
    } else if *dir == Vec3::new(0.0, 0.0, 1.0) || *dir == Vec3::new(0.0, 0.0, -1.0) {
        return Vec3::new(0.0, 1.0, 0.0);
    } else {
        return *dir;
    }
}