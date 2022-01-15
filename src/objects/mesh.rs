use crate::{material::{ RTXMaterial }, geometry::{point::Point3}};

use super::triangle::Triangle;

pub struct Mesh<'material> {
  pub center: Point3,
  pub material: &'material dyn RTXMaterial,
  pub triangles: Vec<Triangle<'material>>
}

impl<'material> Mesh<'material> {
  pub fn new(center: Point3, triangles: Vec<Triangle<'material>>, material: &'material dyn RTXMaterial) -> Self {

    // calculate current center
    // I think just adding up all elements should work.
    let world_center = Point3::default();
    let offset = &center - &world_center;

    Self {
      center,
      material,
      triangles: triangles.iter().map(|triangle| {
        let mut translated_triangle = triangle.translate(&offset);
        translated_triangle.set_material(material);
        translated_triangle
      }).collect()
    }
  }

  pub fn get_triangles(&self) -> Vec<&Triangle<'material>> {
    self.triangles.iter().collect()
  }
}