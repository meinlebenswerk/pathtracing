use std::{fmt, sync::Arc};
use crate::{material::{ RTXMaterial }, geometry::vector3::Vector3f};

use super::point::Point3f;

#[derive(Copy, Clone)]
pub struct Ray {
  pub origin: Point3f,
  pub direction: Vector3f,
  pub current_ior: f32,
  pub inv_direction: Vector3f

  // Medium
  // time
  // tMax
}

impl Ray {
  pub fn new(origin: Point3f, direction: Vector3f) -> Self {

    let dir = direction.normalize();

    Self {
      origin,
      direction: dir,
      inv_direction: 1.0 / dir,
      current_ior: 1.0
    }
  }

  pub fn at(self, t: f32) -> Point3f {
    let pos = self.origin + (self.direction * t);
    Point3f::new(pos.x, pos.y, pos.z)
  }
}

impl fmt::Display for Ray {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "Ray( o: {}, dir: {})", self.origin, self.direction )
  }
}

#[derive(Clone)]
pub struct HitRecord {
  pub t: f32,
  pub point: Point3f,
  pub normal: Vector3f,
  pub front_face: bool,
  pub material: Option<Arc<Box<dyn RTXMaterial + Send + Sync>>>
}

impl HitRecord {
  pub fn new() -> Self {
    Self {
      t: 0.0,
      point: Point3f::default(),
      normal: Vector3f::default(),
      front_face: false,
      material: None
    }
  }

  pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3f) {
    self.front_face = ray.direction.dot(outward_normal) < 0.0;
    self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
  }

  pub fn copy_from<'function>(&mut self, record: &'function HitRecord) {
    self.t = record.t;
    self.point = record.point;
    self.normal = record.normal;
    self.front_face = record.front_face;
    self.material = if let Some(mat) = &record.material { Some(Arc::clone(mat))} else { None };
  }
}