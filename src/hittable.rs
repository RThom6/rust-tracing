use crate::Point3;
use crate::Ray;
use crate::interval::Interval;
use crate::{Vec3, dot};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        Default::default()
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // sets the hit record normal vector
        // outward_normal is assumed to have unit length
        let front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
