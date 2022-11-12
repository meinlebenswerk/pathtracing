use std::sync::Arc;

use crate::geometry::point3::Point3f;
use crate::geometry::ray::{ Ray, HitRecord };
use crate::material::RTXMaterial;
use crate::bvh::BoundingVolume;
use crate::prng::PRNG;
use crate::scene::RTXContext;

pub trait RTXIntersectable<'material> {
  fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
  fn get_material(&self) -> Option<Arc<Box<dyn RTXMaterial + Send + Sync>>>;
  fn get_position(&self) -> Point3f;
  fn get_bounding_volume(&self) -> BoundingVolume;
  fn random_point_on_surface(&self, context: &mut RTXContext, rng: &mut dyn PRNG) -> Point3f;
}