use crate::prelude::*;

pub fn log_interpolate(current: f32, step_length: f32, max: f32, acceleration: f32) -> f32 {
    let x = max / ( 1. + acceleration * std::f32::consts::E.powf(-acceleration * (current + step_length)));
    x.max(0.)
}

pub fn lerp(value: f32, start: f32, end: f32, step: f32) -> f32 {
    start + (end - start) * (((value - start) / (end - start)) + step)
}

pub fn lerp_vec3(vector: Vec3, start: f32, end: f32, step: f32) -> Vec3 {
    Vec3::new(
        lerp(vector.x, start, end, step),
        lerp(vector.y, start, end, step),
        lerp(vector.z, start, end, step),
    )
}
