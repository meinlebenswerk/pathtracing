use std::fmt;
use crate::{material::{ RTXMaterial }, geometry::vector3::Vector3};

use super::point::Point3;

#[derive(Copy, Clone)]
pub struct Ray {
  pub origin: Point3,
  pub direction: Vector3,
  pub current_ior: f32,
  pub inv_direction: Vector3

  // Medium
  // time
  // tMax
}

impl Ray {
  pub fn new(origin: Point3, direction: Vector3) -> Self {

    let dir = direction.normalize();

    Self {
      origin,
      direction: dir,
      inv_direction: 1.0 / dir,
      current_ior: 1.0
    }
  }

  pub fn at(self, t: f32) -> Point3 {
    let pos = self.origin + (self.direction * t);
    Point3::new(pos.x, pos.y, pos.z)
  }
}

impl fmt::Display for Ray {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "Ray( o: {}, dir: {})", self.origin, self.direction )
  }
}

#[derive(Copy, Clone)]
pub struct HitRecord<'material> {
  pub t: f32,
  pub point: Point3,
  pub normal: Vector3,
  pub front_face: bool,
  pub material: Option<&'material dyn RTXMaterial>
}

impl<'material> HitRecord<'material> {
  pub fn new() -> Self {
    Self {
      t: 0.0,
      point: Point3::default(),
      normal: Vector3::default(),
      front_face: false,
      material: None
    }
  }

  pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
    self.front_face = ray.direction.dot(outward_normal) < 0.0;
    self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
  }

  pub fn copy_from<'function>(&mut self, record: &'function HitRecord<'material>) {
    self.t = record.t;
    self.point = record.point;
    self.normal = record.normal;
    self.front_face = record.front_face;
    self.material = record.material;
  }
}