use crate::color::{Color, write_color};
use crate::ray::Ray;
use crate::vec3::*;
use std::io::{self, Write};

pub mod color;
pub mod hittable;
pub mod ray;
pub mod sphere;
pub mod vec3;

fn ray_color(r: &Ray) -> Color {
    // p = center of sphere
    let p = Point3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(&p, 0.5, r);

    if t > 0.0 {
        let n = unit_vector(r.at(t) - p);
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    // multiplying by distance t would give exact 3D point at that distance
    let unit_direction = unit_vector(*r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);

    // Start color -> end color
    return (1.0 - a) * Color::new(0.9, 1.0, 0.3) + a * Color::new(0.6, 0.0, 1.0);
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - *r.origin();

    // quadratic equation
    let a = r.direction().length_squared();
    let h = dot(*r.direction(), oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        -1.0 // didn't hit
    } else {
        (h - discriminant.sqrt()) / a // distance t
    }
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
            // how far over and down the pixel is from pixel(0,0)
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);

            // vector of the pixel from the lens(camera)
            let ray_direction = pixel_center - camera_center;

            // start point and the direction vector
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&mut io::stdout(), pixel_color);
        }
    }

    eprintln!("\rDone.                  \n");
}
