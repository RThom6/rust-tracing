use std::io::{self, Write};

use crate::color::{Color, write_color};

pub mod color;
pub mod ray;
pub mod vec3;

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in 0..IMAGE_HEIGHT {
        eprintln!("\rScanlines remaining: {}", IMAGE_HEIGHT - i);
        io::stderr().flush().unwrap();

        for j in 0..IMAGE_WIDTH {
            let r: f64 = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = i as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0;

            let pixel_color = Color::new(r, g, b);
            write_color(&mut io::stdout(), pixel_color);
        }
    }

    eprintln!("\rDone.                  \n");
}
