use std::rc::Rc;

use super::{Point3, Ray, Vec3, material::*};

pub mod sphere;
pub mod hittable_list;

pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn get_face_normal(r: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = Vec3::dot(r.direction(), &outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (front_face, normal)
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
