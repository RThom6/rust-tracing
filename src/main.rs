use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, unit_vector};
use std::io::{self, Write};

pub mod color;
pub mod ray;
pub mod vec3;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = unit_vector(*r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);

    return (1.0 - a) * Color::new(0.3, 1.0, 0.3) + a * Color::new(0.6, 0.0, 1.0);
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_width: f64 = 400.0;

    let mut image_height: f64 = (image_width / ASPECT_RATIO).floor();
    image_height = if image_height < 1.0 {
        1.0
    } else {
        image_height
    };

    // Camera
    let focal_length = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = (viewport_height * (image_width / image_height)).floor();
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate horizontal and vetical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // Calculate location of upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2 - viewport_v / 2;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    println!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height as u32 {
        eprintln!("\rScanlines remaining: {}", image_height - j as f64);
        io::stderr().flush().unwrap();

        for i in 0..image_width as u32 {
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;

            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&mut io::stdout(), pixel_color);
        }
    }

    eprintln!("\rDone.                  \n");
}
