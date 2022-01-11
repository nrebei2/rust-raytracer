use rand::Rng;

pub fn random_float() -> f64 {
  random_float_rng(0., 1.)
}

pub fn random_float_rng(min: f64, max: f64) -> f64 {
  rand::thread_rng().gen_range(min..max)
}

