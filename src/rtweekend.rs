use std::f32;

#[inline]
fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * f32::consts::PI / 180.0;
}
