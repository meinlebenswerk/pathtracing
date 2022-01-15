use super::vector3::Vector3;

pub struct Bounds3 {
  pub min: Vector3,
  pub max: Vector3
}

impl Bounds3 {
  pub fn new(min: Vector3, max: Vector3) -> Self {
    Self {
      min,
      max
    }
  }
}