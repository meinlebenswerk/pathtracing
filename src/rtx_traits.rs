use crate::geometry::point::Point3;
use crate::geometry::ray::{Ray, HitRecord};
use crate::geometry::vector3::Vector3;
use crate::material::RTXMaterial;
use crate::bvh::BoundingVolume;
use crate::scene::RTXContext;

pub trait RTXIntersectable<'material> {
  fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord<'material>) -> bool;
  fn get_material(&self) -> Option<&'material dyn RTXMaterial>;
  fn get_position(&self) -> Point3;
  fn get_bounding_volume(&self) -> BoundingVolume;
  fn random_point_on_surface(&self, context: &mut RTXContext) -> Point3;
}