use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    vec3::{Vec3, dot, random_unit_vector, reflect, refract, unit_vector},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            // If normal is almost fully opposite the random unit vector
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let f = if fuzz < 1.0 { fuzz } else { 1.0 };

        Self { albedo, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(r_in.direction(), rec.normal);
        reflected = unit_vector(reflected) + (self.fuzz * random_unit_vector());

        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        return dot(scattered.direction(), rec.normal) > 0.0;
    }
}

pub struct Dielectric {
    refractive_idx: f64,
}

impl Dielectric {
    pub fn new(refractive_idx: f64) -> Self {
        Self { refractive_idx }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refractive_idx
        } else {
            self.refractive_idx
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = dot(-unit_direction, rec.normal).min(1.0); // angle between incident ray and normal
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction: Vec3;

        if ri * sin_theta > 1.0 {
            // reflects instead of refracting when angle greater than pi/2 rad
            direction = reflect(unit_direction, rec.normal);
        } else {
            direction = refract(unit_direction, rec.normal, ri);
        }

        // p -> point which the ray hit the surface
        *scattered = Ray::new(rec.p, direction);
        return true;
    }
}
