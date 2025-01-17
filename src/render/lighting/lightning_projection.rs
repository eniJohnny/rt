use crate::model::{materials::color::Color, maths::hit::Hit};

pub fn projection_lighting_from_hit(hit: &mut Hit) -> Color {
    let projection = hit.projection();
    Color::new(projection.u, projection.v, 0.)
}