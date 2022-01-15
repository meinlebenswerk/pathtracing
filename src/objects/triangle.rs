use std::fmt;

use crate::geometry::point::Point3;
use crate::geometry::ray::{HitRecord, Ray};
use crate::geometry::vector3::Vector3;
use crate::prng::PRNG;
use crate::material::RTXMaterial;
use crate::rtx_traits::RTXIntersectable;
use crate::bvh::BoundingVolume;
use crate::scene::RTXContext;


pub struct Triangle<'material> {
  points: [Vector3; 3],
  normal: Vector3,
  material: Option<&'material dyn RTXMaterial>,
  center: Point3,

  edge10: Vector3,
  edge20: Vector3
}

impl<'material> Triangle<'material> {
  pub fn new(a: Vector3, b: Vector3, c: Vector3) -> Self {
    let v0v1 = b - a;
    let v0v2 = c - a;

    let normal = v0v1.cross(&v0v2).normalize();
    let center = (a+b+c) / 3.0;

    Self {
      points: [ a, b, c ],
      normal,
      material: None,
      center: center.as_point3(),
      edge10: v0v1,
      edge20: v0v2
    }
  }

  pub fn intersect_mt(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord<'material>) -> bool {
    let eps = 1e-10;
    let vertex0 = self.points[0];
    let vertex1 = self.points[1];
    let vertex2 = self.points[2];

    let edge1 = vertex1 - vertex0;
    let edge2 = vertex2 - vertex0;
    let h = ray.direction.cross(&edge2);
    let a = edge1.dot(&h);

    //  check for parallel rays
    if f32::abs(a) < eps { return false; }

    let f = 1.0 / a;
    let s = ray.origin.as_vector3() - vertex0;
    let u = f * s.dot(&h);
    if u < 0.0 || u > 1.0 { return false; }

    let q = s.cross(&edge1);
    let v = f * ray.direction.dot(&q);
    if v < 0.0 || u + v > 1.0 { return false; }

    // Compute the intersection point
    let t = f * edge2.dot(&q);
    // 
    if t < t_min || t > t_max { return false; }
    // println!("t={}, {}", t, ray.at(t));

    record.t = t;
    record.point = ray.at(t);
    record.set_face_normal(ray, &self.normal);
    record.material = if self.material.is_none() { None } else { Some(self.material.unwrap() )};

    true
  }

  pub fn translate(&self, offset: &Vector3) -> Self {
    let a = self.points[0] + *offset;
    let b = self.points[1] + *offset;
    let c = self.points[2] + *offset;
    Self::new(a, b, c)
  }

  pub fn set_material(&mut self, material: &'material dyn RTXMaterial) {
    self.material = Some(material);
  }
}

// Printing

impl<'material> fmt::Display for Triangle<'material> {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "{}\t{}\t{}\t->\tNormal: {}", self.points[0], self.points[1], self.points[2], self.normal)
  }
}


impl<'material> RTXIntersectable<'material> for Triangle<'material> {
  fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord<'material>) -> bool {
      self.intersect_mt(ray, t_min, t_max, record)
  }

  fn get_material(&self) -> Option<&'material dyn RTXMaterial> {
      self.material
  }

  fn get_position(&self) -> Point3 {
      self.center
  }

  fn get_bounding_volume(&self) -> BoundingVolume {
      let min = Vector3::min_elementwise(&Vector3::min_elementwise(&self.points[0], &self.points[1]), &self.points[2]);
      let max = Vector3::max_elementwise(&Vector3::max_elementwise(&self.points[0], &self.points[1]), &self.points[2]);
      BoundingVolume::new(min.as_point3(), max.as_point3())
  }

  fn random_point_on_surface(&self, context: &mut RTXContext) -> Point3 {
    let mut a = context.rng.next_f32();
    let mut b = context.rng.next_f32();
    if a+b >= 1.0 {
      a = 1.0 - a;
      b = 1.0 - b;
    }

    (self.points[0] + a * self.edge10 + b * self.edge20).as_point3()
  }
}