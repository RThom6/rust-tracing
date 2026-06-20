use crate::{interval::Interval, vec3::Vec3};
use std::io::Write;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Transform for gamma 2
    // Commented out thanks to zed ppm viewer using linear not gamma space
    // r = linear_to_gamma(r);
    // g = linear_to_gamma(g);
    // b = linear_to_gamma(b);

    let intensity = Interval::of(0.000, 0.999);
    let rbyte: u32 = (256.0 * intensity.clamp(r)) as u32;
    let gbyte: u32 = (256.0 * intensity.clamp(g)) as u32;
    let bbyte: u32 = (256.0 * intensity.clamp(b)) as u32;

    write!(out, "{} {} {}\n", rbyte, gbyte, bbyte).expect("writing color");
}

// This is required to transform our image from linear space to gamma space
// Zeditor's ppm viewer uses the linear space on MacOS so I won't be plugging this in
// But I will leave the code in and comment it out
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
