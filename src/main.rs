use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::*;

mod camera;
mod color;
mod common;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // World
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -1.5, -1.0), 1.0)));

    let mut cam: Camera = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400.0;

    cam.render(Box::new(world));
}
