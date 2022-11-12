use std::ops::{Mul, Sub};

use num::Float;
use crate::config::{RaytracerFloat, RaytracerInt};

use super::{ point2::Point2, vector2::Vector2, utils::{Lerp, MinMax} };

pub struct Bounds2<T> {
  p_min: Point2<T>,
  p_max: Point2<T>
}

impl<T> Bounds2<T>
where T: Sub<T, Output = T> + Mul<T, Output = T> + MinMax + PartialOrd + Copy {
  pub fn new(point1: &Point2<T>, point2: &Point2<T>) -> Self {
    Self {
      p_min: Point2::new(point1.x.min(&point2.x), point1.y.min(&point2.y)),
      p_max: Point2::new(point1.x.max(&point2.x), point1.y.max(&point2.y))
    }
  }

  pub fn diagonal(&self) -> Vector2<T> {
    self.p_max - self.p_min
  }

  pub fn maximum_extent(&self) -> usize {
    let diag = self.diagonal();
    if diag.x > diag.y { 0 } else { 1 }
  }

  pub fn area(&self) -> T {
    let d = self.diagonal();
    d.x * d.y
  }
}

impl<T> Bounds2<T>
where T: Float + Lerp<T> + std::ops::DivAssign<T> {
  pub fn lerp(&self, p: &Point2<T>) -> Point2<T>{
    Point2::<T>::new(
      T::lerp(p.x, &self.p_min.x, &self.p_max.x),
      T::lerp(p.y, &self.p_min.y, &self.p_max.y)
    )
  }

  pub fn offset(&self, point: &Point2<T>) -> Vector2<T> {
    let mut offset = *point - self.p_min;
    if self.p_max.x > self.p_min.x { offset.x /= self.p_max.x - self.p_min.x; }
    if self.p_max.y > self.p_min.y { offset.y /= self.p_max.y - self.p_min.y; }
    offset
  }
}

// Operators
impl<T> PartialEq<Bounds2<T>> for Bounds2<T>
where T: PartialEq {
  fn eq(&self, other: &Bounds2<T>) -> bool {
    self.p_min == other.p_min && self.p_max == other.p_max
  }
  fn ne(&self, other: &Bounds2<T>) -> bool {
    self.p_min != other.p_min || self.p_max != other.p_max
  }
}

// void BoundingSphere(Point2<T> *c, Float *rad) const {
//   *c = (pMin + pMax) / 2;
//   *rad = Inside(*c, *this) ? Distance(*c, pMax) : 0;
// }


pub type Bounds2f = Bounds2<RaytracerFloat>;
pub type Bounds2i = Bounds2<RaytracerInt>;