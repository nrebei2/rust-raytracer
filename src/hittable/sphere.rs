use crate::{hittable::{Hittable, HitRecord, self}, Point3, Vec3, material::Material};
use std::rc::Rc;

pub struct Sphere<M : Material> {
    center: Point3,
    radius: f64,
    mat_ptr: M,
}

impl<M : Material> Sphere<M> {
    pub fn new(center: Point3, radius: f64, mat_ptr: M) -> Self {
        Self { center, radius, mat_ptr }
    }
}

impl<M : Material> hittable::Hittable for Sphere<M> {
    fn hit(
        &self,
        r: &crate::Ray,
        t_min: f64,
        t_max: f64
    ) -> Option<HitRecord> {
        let oc = r.origin() - &self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return None;
        };

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);

        let outward_normal = (&p - &self.center) / self.radius;
        let (front_face, normal) = HitRecord::get_face_normal(r, outward_normal);
        let mat_ptr = &self.mat_ptr;

        Some (HitRecord {t, p, normal, front_face, mat_ptr })
    }
}
