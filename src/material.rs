use crate::{Ray, HitRecord, Color, Vec3};

pub trait Material {
  fn scatter(&self, r_in : &Ray, rec : &HitRecord, attenuation : &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
  albedo : Color
}

impl Lambertian {
  pub fn new(albedo: Color) -> Self {
    Self {albedo}
  }
}

impl Material for Lambertian {
  fn scatter(&self, r_in : &Ray, rec : &HitRecord, attenuation : &mut Color, scattered: &mut Ray) -> bool {
      let mut scatter_direction = &rec.normal + Vec3::random_unit_vector();
      
      if scatter_direction.near_zero() { scatter_direction = rec.normal.clone() }
      
      *scattered = Ray::new(rec.p.clone(), scatter_direction);
      *attenuation = self.albedo.clone();
      true
  }
}

pub struct Metal {
  albedo : Color,
  fuzz : f64
}

impl Metal {
  pub fn new(albedo: Color, fuzz : f64) -> Self {
    Self {albedo, fuzz}
  }
}

impl Material for Metal {
  fn scatter(&self, r_in : &Ray, rec : &HitRecord, attenuation : &mut Color, scattered: &mut Ray) -> bool {
      let reflected = Vec3::reflect(&r_in.direction().unit_vec(), &rec.normal);
      
      *scattered = Ray::new(rec.p.clone(), reflected + self.fuzz * Vec3::random_in_unit_sphere());
      *attenuation = self.albedo.clone();
      
      return Vec3::dot(scattered.direction(), &rec.normal) > 0.
  }
}