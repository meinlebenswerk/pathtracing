use crate::geometry::vector3::Vector3;

pub mod reinhard_devlin;


pub struct QuantisedColor {
  pub r: u8,
  pub g: u8,
  pub b: u8
}

impl QuantisedColor {
  pub fn new(r: f32, g: f32, b: f32) -> Self {

    let rc = (0.0f32).max(r.min(1.0)) * 255.0;
    let gc = (0.0f32).max(g.min(1.0)) * 255.0;
    let bc = (0.0f32).max(b.min(1.0)) * 255.0;

    Self {
      r: rc as u8,
      g: gc as u8,
      b: bc as u8
    }
  }
}


fn rgb_to_luminance(color: &Vector3) -> f32 {
  let scaling = Vector3::new(0.212671, 0.71516, 0.072169);
  scaling.dot(color)
}