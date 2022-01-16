use crate::geometry::ray::{Ray, HitRecord};
use crate::rtx_traits::{ RTXIntersectable };
use crate::objects::mesh::Mesh;
use crate::bvh::BVHNode;
use crate::prng::{ PRNG };


type IntersectableRef<'object, 'material> = &'object dyn RTXIntersectable<'material>;

pub struct Scene<'object, 'material> {
  pub objects: Vec<IntersectableRef<'object, 'material>>,
  pub lights: Vec<IntersectableRef<'object, 'material>>,
  bvh: Option<BVHNode<'object, 'material>>
}



impl<'object, 'material> Scene<'object, 'material> {
  pub fn new() -> Self {
    Self {
      objects: Vec::new(),
      lights: Vec::new(),
      bvh: None
    }
  }

  pub fn add(&mut self, object: &'object Mesh<'material>) {
    for triangle in object.get_triangles() {
      let is_light = triangle.get_material().unwrap().counts_as_light();
      if is_light {
        self.lights.push(triangle);
      } else {
        self.objects.push(triangle);
      }
    }
  }

  pub fn add_generic(&mut self, object: IntersectableRef<'object, 'material>) {
    let is_light = object.get_material().unwrap().counts_as_light();
    if is_light {
      self.lights.push(object);
    } else {
      self.objects.push(object);
    }
  }

  #[allow(dead_code)]
  pub fn get_random_light(&self, context: &mut RTXContext) -> &'object dyn RTXIntersectable<'material> {
    let index = ((self.lights.len() - 1) as f32 * context.rng.next_f32()) as usize;
    self.lights[index]
  }

  fn intersect_no_bvh(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord<'object>) -> bool {
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

  fn intersect_objects(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord<'object>, objects: &[IntersectableRef<'object, 'material>]) -> bool {
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

  fn intersect_bvh(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord<'object>, bvh: &BVHNode<'object, 'material>) -> bool {
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

  pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord<'object>) -> bool {
    if self.bvh.is_none() { self.intersect_no_bvh(ray, t_min, t_max, record) } else { self.intersect_bvh(ray, t_min, t_max, record, self.bvh.as_ref().unwrap()) }
  }

  pub fn all_elements(&self) -> Vec<&'object dyn RTXIntersectable<'material>> {
    self.lights.iter().chain(self.objects.iter()).copied().collect()
  }

  pub fn use_bvh(&mut self, bvh: Option<BVHNode<'object, 'material>>) {
    self.bvh = bvh;
  }
}



// RTXContext
pub struct RTXContext<'objects, 'materials> {
  pub rng: &'objects mut dyn PRNG,
  pub scene: &'objects Scene<'objects, 'materials>
}

impl<'objects, 'materials> RTXContext<'objects, 'materials> {
  pub fn new(rng: &'objects mut dyn PRNG, scene: &'objects Scene<'objects, 'materials>) -> Self {
    Self {
      rng,
      scene
    }
  }
}