use rand::Rng;

use super::vec3::Vec3;


pub fn reflect_dir(dir: &Vec3, normal: &Vec3) -> Vec3 {
    (dir - 2. * dir.dot(normal) * normal).normalize()
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let mut rng = rand::thread_rng();
        let vec = Vec3::new(
            rng.gen_range((-1.)..(1.)),
            rng.gen_range((-1.)..(1.)),
            rng.gen_range((-1.)..(1.)),
        );
        if vec.length() <= 1. {
            return vec.normalize();
        }
    }
}