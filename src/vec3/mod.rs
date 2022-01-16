use std::{
  fmt::Display,
  ops::{Index, IndexMut},
};

use crate::utility::*;

pub mod color;
pub mod ray;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
  pub fn create() -> Self {
      Self(0., 0., 0.)
  }
  pub fn new<T, U, V>(e0: T, e1: U, e2: V) -> Self
  where
      f64: From<T>,
      f64: From<U>,
      f64: From<V>,
  {
      Self(e0.into(), e1.into(), e2.into())
  }

  pub fn random() -> Self {
      Self(random_float(), random_float(), random_float())
  }
  pub fn random_rng(min: f64, max: f64) -> Self {
      Self(random_float_rng(min, max), random_float_rng(min, max), random_float_rng(min, max))
  }
  pub fn random_in_unit_sphere() -> Self {
      loop {
          let gen = Self::random_rng(-1., 1.);
          if gen.length_squared() >= 1. {continue};
          return gen
      }
  }
  pub fn random_unit_vector() -> Self {
      Self::random_in_unit_sphere().unit_vec()
  }
  pub fn random_in_hemisphere(&self) -> Self {
      let gen = Self::random_unit_vector();
      if Self::dot(&gen, self) > 0. {gen} else {-gen}
  }
  pub fn random_in_unit_disk() -> Self {
      loop {
        let gen = Self::new(random_float_rng(-1., 1.), random_float_rng(-1., 1.), 0);
        if gen.length_squared() >= 1. {continue};
        return gen
      }
  }

  pub fn x(&self) -> f64 {
      self.0
  }
  pub fn y(&self) -> f64 {
      self.1
  }
  pub fn z(&self) -> f64 {
      self.2
  }

  pub fn length(&self) -> f64 {
      self.length_squared().sqrt()
  }

  pub fn length_squared(&self) -> f64 {
      self.0 * self.0 + self.1 * self.1 + self.2 * self.2
  }

  pub fn dot(&self, v2: &Self) -> f64 {
      self.0 * v2.0 + self.1 * v2.1 + self.2 * v2.2
  }

  pub fn cross(&self, v2: &Self) -> Self {
      Self(
          self.1 * v2.2 - self.2 * v2.1,
          self.2 * v2.0 - self.0 * v2.2,
          self.0 * v2.1 - self.1 * v2.0,
      )
  }

  pub fn unit_vec(&self) -> Self {
      self / self.length()
  }

  pub fn near_zero(&self) -> bool {
      self.0.abs() < 1e-8 && self.1.abs() < 1e-8 && self.2.abs() < 1e-8
  }

  pub fn reflect(&self, n: &Self) -> Self {
    self - 2.*Self::dot(self, n)*n
  }

  pub fn refract(uv : &Self, n : &Self, etai_over_etat : f64) -> Self {
    let cos_theta = Vec3::dot(&-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    r_out_perp + r_out_parallel
  }
}

impl Index<usize> for Vec3 {
  type Output = f64;

  fn index(&self, index: usize) -> &Self::Output {
      match index {
          0 => &self.0,
          1 => &self.1,
          2 => &self.2,
          _ => panic!("{} is not a valid index!", index),
      }
  }
}

impl IndexMut<usize> for Vec3 {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
      match index {
          0 => &mut self.0,
          1 => &mut self.1,
          2 => &mut self.2,
          _ => panic!("{} is not a valid index!", index),
      }
  }
}

impl Display for Vec3 {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{} {} {}", self[0], self[1], self[2])
  }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

use super::impl_ops;
use std::ops;

// https://docs.rs/impl_ops/0.1.1/impl_ops/index.html

// Binary operators
impl_ops::impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {Vec3(a[0] + b[0], a[1] + b[1], a[2] + b[2])});
impl_ops::impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 {
  Vec3(a[0] - b[0], a[1] - b[1], a[2] - b[2])
});
impl_ops::impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 {
  Vec3(a[0] * b[0], a[1] * b[1], a[2] * b[2])
});
impl_ops::impl_op_ex_commutative!(*|a: &Vec3, b: f64| -> Vec3 {
  Vec3(a[0] * b, a[1] * b, a[2] * b)
});
impl_ops::impl_op_ex!(/ |a: &Vec3, b: f64| -> Vec3 {a * (1./b)});

// Assignment operators
impl_ops::impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| {a[0] += b[0]; a[1] += b[1]; a[2] += b[2]});
impl_ops::impl_op_ex!(*= |a: &mut Vec3, b: f64| {a[0] *= b; a[1] *= b; a[2] *= b});
impl_ops::impl_op_ex!(/= |a: &mut Vec3, b: f64| {*a *= 1./b});

// Unary operator
impl_ops::impl_op_ex!(-|a: &Vec3| -> Vec3 { Vec3(-a[0], -a[1], -a[2]) });

// TODO: Look into other ways of reducing code duplication for impl of references

// impl Neg for Vec3 {
//   type Output = Self;

//   fn neg(self) -> Self::Output {
//       Self (-self.0, -self.1, -self.2)
//   }
// }

// impl AddAssign<Vec3> for Vec3 {
//   fn add_assign(&mut self, rhs: Vec3) {
//       self[0] += rhs[0];
//       self[1] += rhs[1];
//       self[2] += rhs[2];
//   }
// }

// impl MulAssign<f64> for Vec3 {
//   fn mul_assign(&mut self, rhs: f64) {
//       self[0] *= rhs;
//       self[1] *= rhs;
//       self[2] *= rhs;
//   }
// }

// impl DivAssign<f64> for Vec3 {
//   fn div_assign(&mut self, rhs: f64) {
//     *self *= 1./rhs
//   }
// }

// impl Add for Vec3 {
//   type Output = Vec3;
//   fn add(self, rhs: Self) -> Self::Output {
//     Vec3(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
//   }
// }

// impl Sub for Vec3 {
//   type Output = Vec3;
//   fn sub(self, rhs: Self) -> Self::Output {
//       Vec3(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
//   }
// }

// impl Mul for Vec3 {
//   type Output = Vec3;
//   fn mul(self, rhs: Self) -> Self::Output {
//       Vec3(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
//   }
// }

// impl Mul<f64> for Vec3 {
//   type Output = Vec3;
//   fn mul(self, rhs: f64) -> Self::Output {
//       Vec3(self[0] * rhs, self[1] * rhs, self[2] * rhs)
//   }
// }

// impl Div<f64> for Vec3 {
//   type Output = Vec3;
//   fn div(self, rhs: f64) -> Self::Output {
//       self * (1./rhs)
//   }
// }

#[cfg(test)]
mod test {
  use super::Vec3;

  #[test]
  fn add_vec3() {
      assert_eq!(
          Vec3::new(10, 12, 13),
          Vec3::new(5, 1, 13) + Vec3::new(5, 11, 0)
      )
  }

  #[test]
  fn vec3_len() {
      assert_eq!(
          ((3. * 3. + 4.2 * 4.2 + 1. * 1.) as f64).sqrt(),
          Vec3::new(3, 4.2, 1).length()
      )
  }
}
