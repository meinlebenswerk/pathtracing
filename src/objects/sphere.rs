use std::sync::Arc;

use crate::geometry::point3::Point3f;
use crate::geometry::ray::{Ray, HitRecord};
use crate::geometry::utils::random_vector_on_sphere;
use crate::geometry::vector3::Vector3f;
use crate::prng::PRNG;
use crate::rtx_traits::{ RTXIntersectable };
use crate::material::{ RTXMaterial };
use crate::utils::in_range_f32;
use crate::bvh::BoundingVolume;
use crate::scene::RTXContext;

pub struct Sphere {
  pub center: Point3f,
  pub radius: f32,
  inverse_radius: f32,
  pub material: Arc<Box<dyn RTXMaterial + Send + Sync>>
}

impl Sphere {
  pub fn new(center: Point3f, radius: f32, material: &Arc<Box<dyn RTXMaterial + Send + Sync>>) -> Self {
    Self {
      center,
      radius,
      inverse_radius: 1.0 / radius,
      material: Arc::clone(material)
    }
  }
}

impl<'material> RTXIntersectable<'material> for Sphere {
  fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
    let oc = ray.origin - self.center;

    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.length_squared() - self.radius*self.radius;
    let mut disc = b*b - 4.0*c;
    
    if disc < 0.0 {
        return false
    };

    disc = disc.sqrt();
    let solution_1 = -b + disc;
    let solution_2 = -b - disc;

    let mut t = solution_2 / 2.0;

    // ensure one of the solutions is wihtin the acceptable range:
    if !in_range_f32(t, t_min, t_max){
      t = solution_1 / 2.0;
      if !in_range_f32(t, t_min, t_max) {
        return false;
      }
    }


    record.t = t;
    record.point = ray.at(record.t);

    // Using inverse radius looks to be tiny a bit faster (tested via multitime 0.355 vs 0.362 @ mean. of 10 runs)
    let outward_normal = (record.point - self.center) * self.inverse_radius;

    // println!("Sphere(c={}) normal @p={} => {} ", self.center, record.point, outward_normal);
    // if (outward_normal.len2() - 1.0) > 10.0*std::f32::EPSILON {
    //   println!("Someting weird happened in Sphere intersection, normal is longer than expected, @ {}, r={}, t={}, point={}", outward_normal.len(), self.radius, t, record.point);
    // }
    record.set_face_normal(ray, &outward_normal);
    record.material = Some(Arc::clone(&self.material));

    true
  }

  fn get_material(&self) -> Option<Arc<Box<dyn RTXMaterial + Send + Sync>>> {
      Some(Arc::clone(&self.material))
  }

  fn get_position(&self) -> Point3f {
      self.center
  }

  fn get_bounding_volume(&self) -> BoundingVolume {
    let offset = Vector3f::new(self.radius, self.radius, self.radius);
    let min = self.center - offset;
    let max = self.center + offset;
    BoundingVolume::new(min, max)
  }

  fn random_point_on_surface(&self, _context: &mut RTXContext, rng: &mut dyn PRNG) -> Point3f {
    // Project the sphere onto the Plane
    // But that's complicated.

    random_vector_on_sphere(rng, self.center, self.radius)
  }
}