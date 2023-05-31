use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{ Color, Point3, Vec3 };

use rand::Rng;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = r_in.direction.clone().unit_vector().reflect(&rec.normal);
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        *attenuation = self.albedo;

        scattered.direction.dot(&rec.normal) > 0.
    }
}

pub struct Dielectric {
    pub ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Dielectric {
        Dielectric { ir: ir }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;

        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1., 1., 1.);
        
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = r_in.direction.clone().unit_vector();

        let cos_theta = if -unit_direction.dot(&rec.normal) < 1.0 { -unit_direction.dot(&rec.normal) } else { 1.0 };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;
        let mut rng = rand::thread_rng();

        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>()  {
            direction = unit_direction.reflect(&rec.normal);
        } else {
            direction = unit_direction.refract(&rec.normal, refraction_ratio);
        }
        *scattered = Ray::new(rec.p, direction);

        true
    }
}
