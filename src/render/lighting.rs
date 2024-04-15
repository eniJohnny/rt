use crate::model::{materials::Color, maths::hit::Hit, scene::Scene};

pub fn apply_lighting(hit: Hit, scene: &Scene) -> Color {
    let material = hit.element().material();
    let color =match material.needs_projection() {
        false => material.color(0, 0),
        true => {
            let point = hit.element().shape().projection(&hit);
            material.color(point.0, point.1)
        }
    };
    color
}