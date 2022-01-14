use crate::{hittable, Point3, Vec3, material::Material};
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Rc<dyn Material>) -> Self {
        Self { center, radius, mat_ptr }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        let oc = r.origin() - &self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return false;
        };

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = &rec.p - &self.center;
        rec.set_face_normal(r, outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();

        true
    }
}
