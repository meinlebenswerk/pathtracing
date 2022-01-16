use crate::{material::RTXMaterial, geometry::vector::Vector3f};

#[allow(dead_code)]
pub struct Plane<'material> {
  pub normal: Vector3f,
  pub u: Vector3f,
  pub v: Vector3f, 
  material: Option<&'material dyn RTXMaterial>,
}

impl<'material> Plane<'material> {
  #[allow(dead_code)]
  pub fn new(normal: Vector3f, u: Vector3f, v: Vector3f) -> Self {
    Self {
      normal,
      u,
      v,
      material: None
    }
  }

  #[allow(dead_code)]
  pub fn set_material(&mut self, material: &'material dyn RTXMaterial) {
    self.material = Some(material);
  }
}