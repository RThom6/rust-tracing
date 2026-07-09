use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3, dot},
};

pub struct Cuboid {
    center: Point3,
    half_extents: Vec3,
    rotation: [Vec3; 3],
    mat: Rc<dyn Material>,
}

impl Cuboid {
    pub fn new(
        center: Point3,
        half_extents: Vec3,
        rotation: [Vec3; 3],
        mat: Rc<dyn Material>,
    ) -> Cuboid {
        Cuboid {
            center,
            half_extents,
            rotation,
            mat,
        }
    }
}

pub fn rotation_from_degrees(pitch: f64, yaw: f64, roll: f64) -> [Vec3; 3] {
    let p = pitch.to_radians();
    let y = yaw.to_radians();
    let r = roll.to_radians();

    let (sin_p, cos_p) = p.sin_cos();
    let (sin_y, cos_y) = y.sin_cos();
    let (sin_r, cos_r) = r.sin_cos();

    // RPY (ZYX) Tait-Bryan rotation matrix column construction
    // https://www.ce.unipr.it/~medici/geometry_en/node193.html
    let axis_x = Vec3::new(
        cos_y * cos_r,
        sin_p * sin_y * cos_r + cos_p * sin_r,
        -cos_p * sin_y * cos_r + sin_p * sin_r,
    );

    let axis_y = Vec3::new(
        -cos_y * sin_r,
        -sin_p * sin_y * sin_r + cos_p * cos_r,
        cos_p * sin_y * sin_r + sin_p * cos_r,
    );

    let axis_z = Vec3::new(sin_y, -sin_p * cos_y, cos_p * cos_y);

    [axis_x, axis_y, axis_z]
}

impl Hittable for Cuboid {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Transforming ray to OBB's (Oriented Bounding Box) local space
        let delta = r.origin() - self.center;

        let local_origin = Vec3::new(
            dot(delta, self.rotation[0]),
            dot(delta, self.rotation[1]),
            dot(delta, self.rotation[2]),
        );

        let local_direction = Vec3::new(
            dot(r.direction(), self.rotation[0]),
            dot(r.direction(), self.rotation[1]),
            dot(r.direction(), self.rotation[2]),
        );

        let mut t_min = ray_t.min;
        let mut t_max = ray_t.max;
        let mut hit_axis = 0;

        // x axis
        let inv_dx = 1.0 / local_direction.x();
        let mut tx1 = (-self.half_extents.x() - local_origin.x()) * inv_dx;
        let mut tx2 = (self.half_extents.x() - local_origin.x()) * inv_dx;
        if tx1 > tx2 {
            std::mem::swap(&mut tx1, &mut tx2);
        }

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
        let inv_dy = 1.0 / local_direction.y();
        let mut ty1 = (-self.half_extents.y() - local_origin.y()) * inv_dy;
        let mut ty2 = (self.half_extents.y() - local_origin.y()) * inv_dy;
        if ty1 > ty2 {
            std::mem::swap(&mut ty1, &mut ty2);
        }

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
        let inv_dz = 1.0 / local_direction.z();
        let mut tz1 = (-self.half_extents.z() - local_origin.z()) * inv_dz;
        let mut tz2 = (self.half_extents.z() - local_origin.z()) * inv_dz;
        if tz1 > tz2 {
            std::mem::swap(&mut tz1, &mut tz2);
        }

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
        let local_normal = match hit_axis {
            0 => Vec3::new(if local_direction.x() > 0.0 { -1.0 } else { 1.0 }, 0.0, 0.0),
            1 => Vec3::new(0.0, if local_direction.y() > 0.0 { -1.0 } else { 1.0 }, 0.0),
            _ => Vec3::new(0.0, 0.0, if local_direction.z() > 0.0 { -1.0 } else { 1.0 }),
        };

        let outward_normal = self.rotation[0] * local_normal.x()
            + self.rotation[1] * local_normal.y()
            + self.rotation[2] * local_normal.z();

        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());

        true
    }
}
