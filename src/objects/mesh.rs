use std::sync::Arc;

use crate::{material::{ RTXMaterial }, geometry::{point::Point3f}};

use super::triangle::Triangle;

pub struct Mesh {
  pub center: Point3f,
  pub triangles: Vec<Arc<Triangle>>
}

impl Mesh {
  pub fn new(center: Point3f, triangles: Vec<Triangle>, material: &Arc<Box<dyn RTXMaterial + Send + Sync>>) -> Self {

    // calculate current center
    // I think just adding up all elements should work.
    let world_center = Point3f::default();
    let offset = &center - &world_center;

    Self {
      center,
      triangles: triangles.iter().map(|triangle| {
        let mut translated_triangle = triangle.translate(&offset);
        translated_triangle.set_material(Arc::clone(material));
        Arc::new(translated_triangle)
      }).collect()
    }
  }

  pub fn get_triangles(&self) -> Vec<Arc<Triangle>> {
    self.triangles.iter().map(| ta | Arc::clone(ta) ).collect()
  }
}