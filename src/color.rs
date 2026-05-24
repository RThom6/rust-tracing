use crate::{interval::Interval, vec3::Vec3};
use std::io::Write;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity = Interval::of(0.000, 0.999);
    let rbyte: u32 = (256.0 * intensity.clamp(r)) as u32;
    let gbyte: u32 = (256.0 * intensity.clamp(g)) as u32;
    let bbyte: u32 = (256.0 * intensity.clamp(b)) as u32;

    write!(out, "{} {} {}\n", rbyte, gbyte, bbyte).expect("writing color");
}
