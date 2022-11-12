use std::sync::Arc;

use crate::geometry::ray::{Ray, HitRecord};
use crate::rtx_traits::{ RTXIntersectable };
use crate::objects::mesh::Mesh;
use crate::bvh::BVHNode;
use crate::prng::{ PRNG };

pub struct Scene<'material> {
  pub objects: Vec<Arc<dyn RTXIntersectable<'material>  + Send + Sync>>,
  pub lights: Vec<Arc<dyn RTXIntersectable<'material>  + Send + Sync>>,
  bvh: Option<BVHNode<'material>>
}



impl<'material> Scene<'material> {
  pub fn new() -> Self {
    Self {
      objects: Vec::new(),
      lights: Vec::new(),
      bvh: None
    }
  }

  pub fn add(&mut self, object: Arc<Mesh>) {
    for triangle in object.get_triangles() {
      let is_light = triangle.get_material().unwrap().counts_as_light();
      if is_light {
        self.lights.push(triangle);
      } else {
        self.objects.push(triangle);
      }
    }
  }

  pub fn add_generic(&mut self, object: Arc<dyn RTXIntersectable<'material>  + Send + Sync>) {
    let is_light = object.get_material().unwrap().counts_as_light();
    if is_light {
      self.lights.push(Arc::clone(&object));
    } else {
      self.objects.push(Arc::clone(&object));
    }
  }

  #[allow(dead_code)]
  pub fn get_random_light(&self, _context: &mut RTXContext, rng: &mut dyn PRNG) -> Arc<dyn RTXIntersectable<'material>  + Send + Sync> {
    let index = ((self.lights.len() - 1) as f32 * rng.next_f32()) as usize;
    Arc::clone(&self.lights[index])
  }

  fn intersect_no_bvh(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
    let mut tmp_record = HitRecord::new();
    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for object in &self.objects {
      let hit = object.intersect(ray, t_min, closest_so_far, &mut tmp_record);
      if hit {
        hit_anything = true;
        closest_so_far = tmp_record.t;
        record.copy_from(&tmp_record);
      }
    }

    for object in &self.lights {
      let hit = object.intersect(ray, t_min, closest_so_far, &mut tmp_record);
      if hit {
        hit_anything = true;
        closest_so_far = tmp_record.t;
        record.copy_from(&tmp_record);
      }
    }

    // println!("closest so far = {}", closest_so_far);

    hit_anything
  }

  fn intersect_objects(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord, objects: &[Arc<dyn RTXIntersectable<'material>  + Send + Sync>]) -> bool {
    let mut tmp_record = HitRecord::new();
    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for object in objects {
      let hit = object.intersect(ray, t_min, closest_so_far, &mut tmp_record);
      if hit {
        hit_anything = true;
        closest_so_far = tmp_record.t;
        record.copy_from(&tmp_record);
      }
    }

    hit_anything
  }

  fn intersect_bvh(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord, bvh: &BVHNode<'material>) -> bool {
    // recursive intersect with the sub-bvh's
    // But first, check if we hit the BV at all.
    // let (_, root_hit) = bvh.bounds.intersect(ray, t_min, t_max);
    let root_hit = bvh.bounds.intersect_mod(ray, t_min, t_max);
    if !root_hit { return false; }

    // Handle BVH nodes with children
    if !bvh.is_inner {
      return self.intersect_objects(ray, t_min, t_max, record, &bvh.children);
    }

    // check child BVHs
    let mut tmp_record = HitRecord::new();
    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for child in &bvh.sub_volumes {
      let hit = self.intersect_bvh(ray, t_min, closest_so_far, &mut tmp_record, child);
      if hit {
        hit_anything = true;
        closest_so_far = tmp_record.t;
        record.copy_from(&tmp_record);
      }
    }
    
    hit_anything
  }

  pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
    if self.bvh.is_none() { self.intersect_no_bvh(ray, t_min, t_max, record) } else { self.intersect_bvh(ray, t_min, t_max, record, self.bvh.as_ref().unwrap()) }
  }

  pub fn all_elements(&self) -> Vec<Arc<dyn RTXIntersectable<'material>  + Send + Sync>> {
    self.lights.iter().chain(self.objects.iter()).map(| o | Arc::clone(o) ).collect()
  }

  pub fn use_bvh(&mut self, bvh: Option<BVHNode<'material>>) {
    self.bvh = bvh;
  }
}



// RTXContext
pub struct RTXContext<'materials> {
  pub scene: Arc<Scene<'materials>>
}

impl<'materials> RTXContext<'materials> {
  pub fn new(scene: Scene<'materials>) -> Self {
    Self {
      scene: Arc::new(scene)
    }
  }
}