use crate::{material::RTXMaterial, geometry::vector3::Vector3};


pub struct Plane<'material> {
  pub normal: Vector3,
  pub u: Vector3,
  pub v: Vector3, 
  material: Option<&'material dyn RTXMaterial>,
}

impl<'material> Plane<'material> {
  pub fn new(normal: Vector3, u: Vector3, v: Vector3) -> Self {
    Self {
      normal,
      u,
      v,
      material: None
    }
  }

  pub fn set_material(&mut self, material: &'material dyn RTXMaterial) {
    self.material = Some(material);
  }
}