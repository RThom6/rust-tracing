use crate::vec3::Vec3;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let rbyte: u32 = (255.999 * r) as u32;
    let gbyte: u32 = (255.999 * g) as u32;
    let bbyte: u32 = (255.999 * b) as u32;

    write!(out, "{} {} {}\n", rbyte, gbyte, bbyte).expect("writing color");
}
