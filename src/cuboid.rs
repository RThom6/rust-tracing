use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Cuboid {
    center: Point3,
    bounds: [Vec3; 2],
    mat: Rc<dyn Material>,
}

impl Cuboid {
    pub fn new(center: Point3, vmin: Vec3, vmax: Vec3, mat: Rc<dyn Material>) -> Cuboid {
        Cuboid {
            center,
            bounds: [vmin, vmax],
            mat,
        }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;
        let mut hit_axis = 0;

        let ray_origin = r.origin() - self.center;
        let ray_dir = r.direction();

        // x axis
        let tx1 = (self.bounds[r.sign(0)].x() - ray_origin.x()) * (1.0 / ray_dir.x());
        let tx2 = (self.bounds[1 - r.sign(0)].x() - ray_origin.x()) * (1.0 / ray_dir.x());

        if tx1 > t_min {
            t_min = tx1;
            hit_axis = 0;
        }
        if tx2 < t_max {
            t_max = tx2;
        }
        if t_min >= t_max {
            return false;
        }

        // y axis
        let ty1 = (self.bounds[r.sign(1)].y() - ray_origin.y()) * (1.0 / ray_dir.y());
        let ty2 = (self.bounds[1 - r.sign(1)].y() - ray_origin.y()) * (1.0 / ray_dir.y());

        if ty1 > t_min {
            t_min = ty1;
            hit_axis = 1;
        }
        if ty2 < t_max {
            t_max = ty2;
        }
        if t_min >= t_max {
            return false;
        }

        // z axis
        let tz1 = (self.bounds[r.sign(2)].z() - ray_origin.z()) * (1.0 / ray_dir.z());
        let tz2 = (self.bounds[1 - r.sign(2)].z() - ray_origin.z()) * (1.0 / ray_dir.z());

        if tz1 > t_min {
            t_min = tz1;
            hit_axis = 2;
        }
        if tz2 < t_max {
            t_max = tz2;
        }
        if t_min >= t_max {
            return false;
        }

        if !ray_t.surrounds(t_min) {
            return false;
        }

        rec.t = t_min;
        rec.p = r.at(rec.t);

        // Calculate outward normal based on hit axis and ray direction sign
        let outward_normal = match hit_axis {
            0 => Vec3::new(if ray_dir.x() > 0.0 { -1.0 } else { 1.0 }, 0.0, 0.0),
            1 => Vec3::new(0.0, if ray_dir.y() > 0.0 { -1.0 } else { 1.0 }, 0.0),
            _ => Vec3::new(0.0, 0.0, if ray_dir.z() > 0.0 { -1.0 } else { 1.0 }),
        };
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());

        true
    }
}
