pub mod mt19937;

pub trait PRNG {
  fn next_f32(&mut self) -> f32;
}