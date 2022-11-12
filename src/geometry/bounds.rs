use super::point::{ Point2f, Point3f, Point2i, Point3i };

#[allow(dead_code)]
pub struct Bounds<T> {
  pub min: T,
  pub max: T
}

#[allow(dead_code)]
impl<T> Bounds<T> {
  pub fn new(min: T, max: T) -> Self {
    Self {
      min,
      max
    }
  }
}

pub type Bounds2f = Bounds<Point2f>;
pub type Bounds3f = Bounds<Point3f>;
pub type Bounds2i = Bounds<Point2i>;
pub type Bounds3i = Bounds<Point3i>;