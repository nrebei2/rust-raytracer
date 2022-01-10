use super::{Point3, Vec3, Ray};

pub struct HitRecord {
  pub p : Point3,
  pub normal : Vec3,
  pub t : f64,
  pub front_face : bool,
}

impl HitRecord {
  pub fn new() -> Self {
    Self {p : Point3::new(), normal : Vec3::new(), t : 0., front_face : false}
  }
  pub fn set_face_normal(&mut self, r : &Ray, outward_normal : Vec3) {
    let front_face = Vec3::dot(r.direction(), &outward_normal) < 0.;
    self.normal = if front_face {outward_normal} else {-outward_normal}
  }
}

pub trait Hittable {
  fn hit(&self, r : &Ray, t_min : f64, t_max : f64, rec : &mut HitRecord) -> bool;
}