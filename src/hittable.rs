use crate::material::{ Lambertian, Material };
use crate::ray::Ray;
use crate::vec3::{ Color, Point3, Vec3 };

use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new_empty() -> HitRecord {
        HitRecord {
            p: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            mat_ptr: Rc::new(Lambertian::new(Color::new(0., 0., 0.))),
            t: 0.,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.;
        self.normal = if self.front_face { outward_normal.clone() } else { -outward_normal.clone() };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
