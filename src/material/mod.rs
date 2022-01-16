use crate::{Ray, HitRecord, Color, Vec3, utility::random_float};

pub trait Material: Sync {
  fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Color, Ray)>;
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
  fn scatter(&self, _r_in : &Ray, rec : &HitRecord) -> Option<(Color, Ray)> {
      let mut scatter_direction = &rec.normal + Vec3::random_unit_vector();
      
      if scatter_direction.near_zero() { scatter_direction = rec.normal }
      
      Some ((self.albedo, Ray::new(rec.p, scatter_direction)))
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
  fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Color, Ray)> {
      let mut reflected = Vec3::reflect(&r_in.direction().unit_vec(), &rec.normal);

      if self.fuzz > 0.0 {reflected += self.fuzz * Vec3::random_in_unit_sphere()}

      if Vec3::dot(&reflected, &rec.normal) > 0. {
        Some ((self.albedo, Ray::new(rec.p, reflected)))
      } else {
        None
      }

  }
}

pub struct Dielectic {
  ir : f64 // Index of Refraction
}

impl Dielectic {
  pub fn new(ir : f64) -> Self {
    Self {ir}
  }
  fn reflectance(cosine : f64, ref_idx : f64) -> f64 {
    // Schlick's approximation for reflectance
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    return r0 + (1. - r0) * (1. - cosine).powi(5);
  }
}

impl Material for Dielectic {
  fn scatter(&self, r_in : &Ray, rec : &HitRecord) -> Option<(Color, Ray)> {
      let refraction_ratio = if rec.front_face {1.0/self.ir} else {self.ir};

      let unit_direction = r_in.direction().unit_vec();
      let cos_theta = (-&unit_direction).dot(&rec.normal).min(1.0);
      let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

      let cannot_refract = refraction_ratio * sin_theta > 1.0;
      let direction : Vec3;

      if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_float() {
        direction = Vec3::reflect(&unit_direction, &rec.normal)
      } else {
        direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
      }

      Some ((Color::new(1, 1, 1), Ray::new(rec.p, direction)))

  }
}