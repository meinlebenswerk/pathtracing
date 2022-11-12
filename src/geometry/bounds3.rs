use std::ops::{Mul, Sub, Add, Index, DivAssign};

use num::One;

use crate::config::{RaytracerFloat, RaytracerInt};

use super::{point3::Point3, vector3::Vector3, utils::{Lerp, MinMax}};


pub struct Bounds3<T> {
  p_min: Point3<T>,
  p_max: Point3<T>
}

impl<T> Bounds3<T>
where T: MinMax{
  pub fn new(point1: &Point3<T>, point2: &Point3<T>) -> Self {
    Self {
      p_min: Point3::new(point1.x.min(&point2.x), point1.y.min(&point2.y), point1.z.min(&point2.z)),
      p_max: Point3::new(point1.x.max(&point2.x), point1.y.max(&point2.y), point1.z.max(&point2.z))
    }
  }
}

impl<T> Bounds3<T>
where T: Sub<T, Output = T> + Add<T, Output = T> + Mul<T, Output = T> + DivAssign<T> + One + PartialOrd + Lerp<T> + MinMax + Copy {
  pub fn diagonal(&self) -> Vector3<T> {
    self.p_max - self.p_min
  }

  pub fn surface_area(&self) -> T {
    let diag = self.diagonal();
    return (T::one() + T::one()) * (diag.x*diag.y + diag.x*diag.z + diag.y*diag.z)
  }
  
  pub fn volume(&self) -> T {
    let diag = self.diagonal();
    diag.x * diag.y * diag.z
  }

  pub fn maximum_extent(&self) -> usize {
    let diag = self.diagonal();
    if diag.x > diag.y { if diag.x > diag.z { 0 } else { 2 }} else { if diag.y > diag.z { 1 } else { 2 }}
  }

  pub fn offset(&self, point: &Point3<T>) -> Vector3<T> {
    let mut offset = *point - self.p_min;
    if self.p_max.x > self.p_min.x { offset.x /= self.p_max.x - self.p_min.x }
    if self.p_max.y > self.p_min.y { offset.y /= self.p_max.y - self.p_min.y }
    if self.p_max.z > self.p_min.z { offset.z /= self.p_max.z - self.p_min.z }
    offset
  }

  pub fn lerp(&self, p: &Point3<T>) -> Point3<T>{
    Point3 {
      x: T::lerp(p.x, &self.p_min.x, &self.p_max.x),
      y: T::lerp(p.y, &self.p_min.y, &self.p_max.y),
      z: T::lerp(p.z, &self.p_min.z, &self.p_max.z)
    }
  }

  pub fn union(&self, point: &Point3<T>) -> Self {
    Self::new(
      &Point3::new(
        self.p_min.x.min(&point.x),
        self.p_min.y.min(&point.y),
        self.p_min.z.min(&point.z),
      ),
      &Point3::new(
        self.p_max.x.max(&point.x),
        self.p_max.y.max(&point.y),
        self.p_max.z.max(&point.z),
      )
    )
  }

  pub fn union_bounds(&self, other: &Bounds3<T>) -> Self {
    Self::new(
      &Point3::new(
        self.p_min.x.min(&other.p_min.x),
        self.p_min.y.min(&other.p_min.y),
        self.p_min.z.min(&other.p_min.z),
      ),
      &Point3::new(
        self.p_max.x.max(&other.p_max.x),
        self.p_max.y.max(&other.p_max.y),
        self.p_max.z.max(&other.p_max.z),
      )
    )
  }

  pub fn intersection(&self, other: &Bounds3<T>) -> Self {
    Self::new(
      &Point3::new(
        self.p_min.x.max(&other.p_min.x),
        self.p_min.y.max(&other.p_min.y),
        self.p_min.z.max(&other.p_min.z),
      ),
      &Point3::new(
        self.p_max.x.min(&other.p_max.x),
        self.p_max.y.min(&other.p_max.y),
        self.p_max.z.min(&other.p_max.z),
      )
    )
  }

  pub fn overlaps(&self, other: &Bounds3<T>) -> bool {
    let x = (self.p_max.x >= other.p_min.x) && (self.p_min.x <= other.p_max.x);
    let y = (self.p_max.y >= other.p_min.y) && (self.p_min.y <= other.p_max.y);
    let z = (self.p_max.z >= other.p_min.z) && (self.p_min.z <= other.p_max.z);
    x && y && z
  }

  pub fn point_inside(&self, point: &Point3<T>) -> bool{
    point.x >= self.p_min.x && point.x <= self.p_max.x &&
    point.y >= self.p_min.y && point.y <= self.p_max.y &&
    point.z >= self.p_min.z && point.z <= self.p_max.z
  }

  pub fn expand(&self, factor: T) -> Self {
    Self::new(
      &(self.p_min - Vector3::new(factor, factor, factor)),
      &(self.p_max + Vector3::new(factor, factor, factor))
    )
  }
}

impl<T> Bounds3<T>
where T: Add<T, Output = T> + Copy {
  pub fn corner(&self, index: usize) -> Point3<T> {
    Point3{
      x: self[ if index & 1 == 0 { 0 } else { 1 } ].x,
      y: self[ if index & 2 == 0 { 0 } else { 1 } ].y,
      z: self[ if index & 4 == 0 { 0 } else { 1 } ].z,
    }
  }
}

// Operators
impl<T> Index<usize> for Bounds3<T> {
  type Output = Point3<T>;
  fn index(&self, index: usize) -> &Self::Output {
    assert!(index <= 1);
    match index {
      0 => &self.p_min,
      1 => &self.p_max,
      _ => &self.p_max
    }
  }
}

impl<T> PartialEq<Bounds3<T>> for Bounds3<T>
where T: PartialEq {
  fn eq(&self, other: &Bounds3<T>) -> bool {
      self.p_min == other.p_min && self.p_max == other.p_max
  }

  fn ne(&self, other: &Bounds3<T>) -> bool {
      self.p_min != other.p_min || self.p_max != other.p_max
  }
}

// void BoundingSphere(Point3<T> *center, Float *radius) const {
//   *center = (pMin + pMax) / 2;
//   *radius = Inside(*center, *this) ? Distance(*center, pMax) : 0;
// }
// bool IntersectP(const Ray &ray, Float *hitt0 = nullptr, Float *hitt1 = nullptr) const;
// inline bool IntersectP(const Ray &ray, const Vector3f &invDir,
//                      const int dirIsNeg[3]) const;

pub type Bounds3f = Bounds3<RaytracerFloat>;
pub type Bounds3i = Bounds3<RaytracerInt>;