use std::f32;

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * f32::consts::PI / 180.0;
}

#[inline]
pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { return min; }
    if x > max { return max; }
    x
}
