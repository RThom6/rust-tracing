use crate::vec3::{Point3, Vec3};

#[derive(Copy, Clone, Default, Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    sign: [usize; 3],
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
            sign: [
                // sign of ray dir
                (1.0 / direction.x() < 0.0) as usize,
                (1.0 / direction.y() < 0.0) as usize,
                (1.0 / direction.z() < 0.0) as usize,
            ],
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin() + t * self.direction()
    }

    pub fn sign(&self, i: usize) -> usize {
        self.sign[i]
    }
}
