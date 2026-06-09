use std::io::{self, Write, stdout};

use crate::{
    color::{Color, write_color},
    common::{self, random_double},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3, unit_vector},
};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: f64,
    pub samples_per_pixel: u32,
    image_height: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new() -> Camera {
        Default::default()
    }

    pub fn render(&mut self, world: Box<dyn Hittable>) {
        self.initialize();

        // Render
        println!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height as u32 {
            eprintln!("\rScanlines remaining: {}", self.image_height - j as f64);
            io::stderr().flush().unwrap();

            for i in 0..self.image_width as u32 {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, &world);
                }

                write_color(&mut io::stdout(), self.pixel_samples_scale * pixel_color);
            }
        }

        eprintln!("\rDone.                  \n");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width / self.aspect_ratio).floor();
        self.image_height = if self.image_height < 1.0 {
            1.0
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 =
            (viewport_height * (self.image_width / self.image_height)).floor();

        // Calculate vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate horizontal and vetical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;

        // Calculate location of upper left pixel
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2 - viewport_v / 2;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, r: &Ray, world: &Box<dyn Hittable>) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(r, Interval::of(0.0, common::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        // multiplying by distance t would give exact 3D point at that distance
        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);

        // Start color -> end color
        return (1.0 - a) * Color::new(0.9, 1.0, 0.3) + a * Color::new(0.6, 0.0, 1.0);
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
