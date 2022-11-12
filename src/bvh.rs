use crate::geometry::point3::Point3f;
use crate::geometry::ray::Ray;
use crate::geometry::utils::MinMax;
use crate::geometry::vector3::Vector3f;
use crate::rtx_traits::{ RTXIntersectable };
use crate::scene::{ Scene };
use std::fmt;
use std::sync::Arc;
// A simple BVH Implementation

#[derive(Copy, Clone)]
pub struct BoundingVolume {
  min: Point3f,
  max: Point3f
}

impl BoundingVolume {
  pub fn new(min: Point3f, max: Point3f) -> Self {
    Self {
      min,
      max
    }
  }

  pub fn from(volumes: &Vec<&BoundingVolume>) -> Self {
    let (min, max) = volumes.iter().fold((volumes[0].min, volumes[0].max), | (min, max), e| {
      (MinMax::min(&min, &e.min), MinMax::max(&max, &e.max))
    });
    Self::new(min, max)
  }
  
  #[allow(dead_code)]
  pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> (f32, bool) {


    // calculate intersection point with the box's x-coord points
    // TODO this can be optimized, a lot.

    //  I think min and max-values should be swapped around based on the ray-direction

    let mut min_t;
    let mut max_t;

    let min_t_y;
    let max_t_y;

    let min_t_z;
    let max_t_z;

    if ray.direction.x >= 0.0 {
      min_t = (self.min.x - ray.origin.x) * ray.inv_direction.x;
      max_t = (self.max.x - ray.origin.x) * ray.inv_direction.x;
    } else {
      min_t = (self.max.x - ray.origin.x) * ray.inv_direction.x;
      max_t = (self.min.x - ray.origin.x) * ray.inv_direction.x;
    }

    if max_t < t_min || min_t > t_max { return (-1.0, false) }

    min_t = f32::max(min_t, t_min);
    max_t = f32::min(max_t, t_max);

    // There must be a way to do this better.

    if ray.direction.y >= 0.0 {
      min_t_y = (self.min.y - ray.origin.y) * ray.inv_direction.y;
      max_t_y = (self.max.y - ray.origin.y) * ray.inv_direction.y;
    } else {
      min_t_y = (self.max.y - ray.origin.y) * ray.inv_direction.y;
      max_t_y = (self.min.y - ray.origin.y) * ray.inv_direction.y;
    }

    // Quick-return on inplausible results
    if max_t_y < min_t || min_t_y > max_t { return (-1.0, false); }

    min_t = f32::max(min_t, min_t_y);
    max_t = f32::min(max_t, max_t_y);

    if ray.direction.z >= 0.0 {
      min_t_z = (self.min.z - ray.origin.z) * ray.inv_direction.z;
      max_t_z = (self.max.z - ray.origin.z) * ray.inv_direction.z;
    } else {
      min_t_z = (self.max.z - ray.origin.z) * ray.inv_direction.z;
      max_t_z = (self.min.z - ray.origin.z) * ray.inv_direction.z;
    }

    if max_t_z < min_t || min_t_z > max_t { return (-1.0, false); }
    
    min_t = f32::max(min_t, min_t_z);
    max_t = f32::min(max_t, max_t_z);

    // println!("min={}, max={}", min_t, max_t);
    let mut t = min_t;
    if t < t_min || t > t_max {
      t = max_t;
      if t < t_min || t > t_max { return (-1.0, false); }
    }

    (t, true)
  }

  pub fn intersect_mod(&self, ray: &Ray, t_bound_min: f32, t_bound_max: f32) -> bool {
    // calculate intersection point with the box's x-coord points

    let x_dir_negative = ray.direction.x < 0.0;
    let y_dir_negative = ray.direction.y < 0.0;
    let z_dir_negative = ray.direction.z < 0.0;

    let mut t_min = ((if x_dir_negative { self.max.x } else { self.min.x }) - ray.origin.x) * ray.inv_direction.x;
    let mut t_max = ((if x_dir_negative { self.min.x } else { self.max.x }) - ray.origin.x) * ray.inv_direction.x;

    let t_min_y = ((if y_dir_negative { self.max.y } else { self.min.y }) - ray.origin.y) * ray.inv_direction.y;
    let t_max_y = ((if y_dir_negative { self.min.y } else { self.max.y }) - ray.origin.y) * ray.inv_direction.y;

    // Check for no overlap between x- and y intersection ranges
    if  t_min > t_max_y || t_min_y > t_max {
      return false
    }

    if t_min_y > t_min { t_min = t_min_y; }
    if t_max_y < t_max { t_max = t_max_y; }

    let t_min_z = ((if z_dir_negative { self.max.z } else { self.min.z }) - ray.origin.z) * ray.inv_direction.z;
    let t_max_z = ((if z_dir_negative { self.min.z } else { self.max.z }) - ray.origin.z) * ray.inv_direction.z;

    // Check for no overlap between current and z intersection ranges
    if  t_min > t_max_z || t_min_z > t_max {
      return false
    }

    if t_min_z > t_min { t_min = t_min_z; }
    if t_max_z < t_max { t_max = t_max_z; }

    t_min < t_bound_max && t_max > t_bound_min
  }

  #[allow(dead_code)]
  pub fn diagonal(&self) -> Vector3f {
    self.max - self.min
  }

  #[allow(dead_code)]
  pub fn maximum_extent(&self) -> usize {
    let diag = self.diagonal();
    if diag.x > diag.y && diag.x > diag.z {
      return 0;
    } else if diag.y > diag.z {
      return 1;
    }
    2
  }

  #[allow(dead_code)]
  pub fn lerp(&self, t: &Vector3f) -> Vector3f {
    Vector3f::new(
      (1.0 - t.x) * self.min.x + t.x * self.max.x,
      (1.0 - t.y) * self.min.y + t.y * self.max.y,
      (1.0 - t.z) * self.min.z + t.z * self.max.z
    )
  }
}

impl fmt::Display for BoundingVolume {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "BoundingBox (min={}, max={})", self.min, self.max)
  }
}


type Intersectable<'material> = Arc<(dyn RTXIntersectable<'material> + Send + Sync)>;
type IVolPair<'material> = (Intersectable<'material>, BoundingVolume);

pub struct BVHNode<'material> {
  pub is_inner: bool,
  pub sub_volumes: Vec<Box<BVHNode<'material>>>,
  pub children: Vec<Arc<(dyn RTXIntersectable<'material> + Send + Sync)>>,
  pub bounds: BoundingVolume
}

impl<'material> BVHNode<'material> {

  fn new_inner( left: BVHNode<'material>, right: BVHNode<'material>, bounds: BoundingVolume) -> Self {
    Self {
      is_inner: true,
      sub_volumes: vec![Box::new(left), Box::new(right)],
      children: Vec::new(),
      bounds
    }
  }

  fn new_leaf(children: Vec<Arc<(dyn RTXIntersectable<'material> + Send + Sync)>>, bounds: BoundingVolume) -> Self {
    Self {
      is_inner: false,
      sub_volumes: Vec::new(),
      children,
      bounds
    }
  }

  #[allow(dead_code)]
  fn get_depth(&self, depth: usize) -> usize {
    if self.is_inner { return depth + 1 };
    let left_depth = self.sub_volumes[0].get_depth(depth + 1);
    let right_depth = self.sub_volumes[1].get_depth(depth + 1);
    depth.max(left_depth.max(right_depth))
  }

  #[allow(dead_code)]
  pub fn print_tree(&self) {
    println!("BVH Tree w/ depth={}", self.get_depth(0));
  }
}


// Pretty shitty - it loops ad absurdum
// It needs a depth-limit
// But the object-median is much better
fn generate_bvh_node_spatial_median<'material>(elements: &Vec<IVolPair<'material>>, max: usize) -> BVHNode<'material> {
  let bounds = BoundingVolume::from(&(elements.iter().map(|(_, bv)| bv).collect()));
  if elements.len() <= max {
    let children: Vec<Arc<(dyn RTXIntersectable<'material> + Send + Sync)>> = elements.iter().map(|(e, _)| Arc::clone(e)).collect();
    return BVHNode::new_leaf(children, bounds);
  }

  // Split @ spatial median
  let axis_sizes = bounds.max - bounds.min;
  if axis_sizes.x > axis_sizes.y && axis_sizes.x > axis_sizes.z {
    // println!("Splitting along X-Axis");
    // split along x-axis
    let x_split = axis_sizes.x / 2.0 + bounds.min.x;
    let children_smaller: Vec<(Arc<(dyn RTXIntersectable<'material> + Send + Sync)>, BoundingVolume)> = elements.iter()
      .filter(|(e, _)| e.get_position().x <= x_split)
      .map(|(e, bv)| (Arc::clone(e), *bv))
      .collect();
    let children_larger: Vec<(Arc<(dyn RTXIntersectable<'material> + Send + Sync)>, BoundingVolume)> = elements.iter()
      .filter(|(e, _)| e.get_position().x > x_split)
      .map(|(e, bv)| (Arc::clone(e), *bv))
      .collect();
    
    // println!("Got {} smaller and {} larger elements", children_smaller.len(), children_larger.len());
    println!();
    return BVHNode::new_inner(
      generate_bvh_node_spatial_median(&children_smaller, max), 
      generate_bvh_node_spatial_median(&children_larger, max), 
      bounds);
  } else if axis_sizes.y > axis_sizes.x && axis_sizes.y > axis_sizes.z {
    // println!("Splitting along Y-Axis");
    // split along y-axis
    let y_split = axis_sizes.y / 2.0 + bounds.min.y;
    let children_smaller: Vec<(Arc<(dyn RTXIntersectable<'material> + Send + Sync)>, BoundingVolume)> = elements.iter()
      .filter(|(e, _)| e.get_position().y <= y_split)
      .map(|(e, bv)| (Arc::clone(e), *bv))
      .collect();

    let children_larger: Vec<(Arc<(dyn RTXIntersectable<'material> + Send + Sync)>, BoundingVolume)> = elements.iter()
      .filter(|(e, _)| e.get_position().y > y_split)
      .map(|(e, bv)| (Arc::clone(e), *bv))
      .collect();

    // println!("Got {} smaller and {} larger elements", children_smaller.len(), children_larger.len());
    println!();
    return BVHNode::new_inner(
      generate_bvh_node_spatial_median(&children_smaller, max), 
      generate_bvh_node_spatial_median(&children_larger, max), 
      bounds);
  } else  {
    // println!("Splitting along Z-Axis");
    // split along Z-axis
    let z_split = axis_sizes.z / 2.0 + bounds.min.z;
    let children_smaller: Vec<(Arc<(dyn RTXIntersectable<'material> + Send + Sync)>, BoundingVolume)> = elements.iter()
      .filter(|(e, _)| e.get_position().z <= z_split)
      .map(|(e, bv)| (Arc::clone(e), *bv))
      .collect();
    let children_larger: Vec<(Arc<(dyn RTXIntersectable<'material> + Send + Sync)>, BoundingVolume)> = elements.iter()
      .filter(|(e, _)| e.get_position().z > z_split)
      .map(|(e, bv)| (Arc::clone(e), *bv))
      .collect();

    // println!("Got {} smaller and {} larger elements", children_smaller.len(), children_larger.len());
    println!();
    return BVHNode::new_inner(
      generate_bvh_node_spatial_median(&children_smaller, max), 
      generate_bvh_node_spatial_median(&children_larger, max), 
      bounds);
  }
}


fn split_by_axis<'material>(elements: &[IVolPair<'material>], axis_index: usize) -> (Vec<IVolPair<'material>>, Vec<IVolPair<'material>>) {
  assert!(axis_index < 3);
  
  let mut sorted_elements: Vec<(Arc<(dyn RTXIntersectable<'material> + Send + Sync)>, BoundingVolume)> = elements.iter().map(| (e, bv) | (Arc::clone(e), bv.clone()) ).collect();
  sorted_elements.sort_by(|(a, _), (b, _)| {
    a.get_position()[axis_index].partial_cmp(&(b.get_position()[axis_index])).unwrap()
  });

  let n = elements.len() / 2;
  let subarray_a: Vec<IVolPair<'material>> = sorted_elements[0..n]
    .iter()
    .map(| (e, bv) | (Arc::clone(e), bv.clone()) )
    .collect();
  let subarray_b: Vec<IVolPair<'material>> = sorted_elements[n..]
    .iter()
    .map(| (e, bv) | (Arc::clone(e), bv.clone()) )
    .collect();

  // println!("subarray sizes: {},{} | {}", subarray_a.len(), subarray_b.len(), elements.len());
  (subarray_a, subarray_b)
}

fn generate_bvh_node_object_median<'material>(elements: &[IVolPair<'material>], max: usize) -> BVHNode<'material> {
  let bounds = BoundingVolume::from(&(elements.iter().map(|(_, bv)| bv).collect()));
  if elements.len() <= max {
    let children: Vec<Intersectable<'material>> = elements.iter().map(|(e, _)| Arc::clone(e)).collect();
    return BVHNode::new_leaf(children, bounds);
  }

  let mut best_split = split_by_axis(elements, 0);
  let mut best_ratio = f32::abs(1.0 - best_split.0.len() as f32 / best_split.1.len() as f32);
  for i in 1..2 {
    let tmp_split = split_by_axis(elements, i);
    let ratio = f32::abs(1.0 - tmp_split.0.len() as f32 / tmp_split.1.len() as f32);

    if ratio < best_ratio {
      best_ratio = ratio;
      best_split = tmp_split;
    }
  }

  return BVHNode::new_inner(
    generate_bvh_node_object_median(&best_split.0, max), 
    generate_bvh_node_object_median(&best_split.1, max), 
    bounds
  );
}


pub fn generate_bvh<'object, 'material>(scene: &Scene<'material>, use_object_median: bool) -> BVHNode<'material> {
  // For each object in the scene, generate and temporarily store a bounding volume:
  let pairs: Vec<IVolPair<'material>> = scene
    .all_elements()
    .iter()
    .map(|e| { (Arc::clone(e), e.get_bounding_volume()) })
    .collect();
    if use_object_median {
      generate_bvh_node_object_median(&pairs, 3)
    } else {
      generate_bvh_node_spatial_median(&pairs, 19)
    }
}