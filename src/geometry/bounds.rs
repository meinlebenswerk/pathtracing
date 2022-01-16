use super::vector::Vector3f;

#[allow(dead_code)]
pub struct Bounds3 {
  pub min: Vector3f,
  pub max: Vector3f
}

#[allow(dead_code)]
impl Bounds3 {
  pub fn new(min: Vector3f, max: Vector3f) -> Self {
    Self {
      min,
      max
    }
  }
}