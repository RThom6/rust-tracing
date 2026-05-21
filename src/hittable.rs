use crate::Point3;
use crate::Ray;
use crate::Vec3;

#[derive(Clone, Default)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        Default::default()
    }
}

pub trait Hittable {
    fn hit(r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &HitRecord) -> bool;
}
