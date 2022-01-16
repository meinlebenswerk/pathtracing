use std::fmt::Display;
use std::ops;
use std::fmt;
use std::ops::Add;
use std::ops::Mul;

use num::Signed;
use num::Zero;

use crate::config::RaytracerFloat;
use crate::config::RaytracerInt;

use super::utils::MinMax;
use super::vector::Vector3;

#[derive(Copy, Clone)]
pub struct Point3<T: Signed + Copy + Clone + MinMax> {
  pub x: T,
  pub y: T,
  pub z: T
}

impl<T> Point3<T>
  where T: Signed + Copy + Clone + MinMax {
  pub fn new(x: T, y: T, z: T) -> Self {
    Self {
      x,
      y,
      z
    }
  }

  pub fn permute(&self, x: usize, y: usize, z: usize) -> Self {
    Self::new(
      self[x],
      self[y],
      self[z]
    )
  }

  pub fn abs(&self) -> Self {
    Self::new(
      self.x.abs(), 
      self.y.abs(), 
      self.z.abs()
    )
  }

  pub fn min(&self, b: &Self) -> Self {
    let x = MinMax::min(self.x, b.x);
    let y = MinMax::min(self.y, b.y);
    let z = MinMax::min(self.z, b.z);
    Self::new(x, y, z)
  }

  pub fn max(&self, b: &Self) -> Self {
    let x = MinMax::max(self.x, b.x);
    let y = MinMax::max(self.y, b.y);
    let z = MinMax::max(self.z, b.z);
    Self::new(x, y, z)
  }

  pub fn as_Vector3(&self) -> Vector3<T> {
    Vector3::new(self.x, self.y, self.z)
  }
}

// Point3 Addition
impl<T: Add<Output=T> + Copy + Clone + Signed + MinMax> ops::Add for Point3<T> {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
      Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl<T: Add<Output=T> + Copy + Clone + Signed + MinMax> ops::Add<Vector3<T>> for Point3<T> {
  type Output = Self;
  fn add(self, rhs: Vector3<T>) -> Self::Output {
      Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}


// Point3 Subtraction
impl<T: Copy + Clone + Signed + MinMax> ops::Sub for Point3<T> {
  type Output = Vector3<T>;
  fn sub(self, _rhs: Self) -> Self::Output {
    Self::Output::new(
      self.x - _rhs.x, 
      self.y - _rhs.y, 
      self.z - _rhs.z
    )
  }
}

impl<T: Copy + Clone + Signed + MinMax> ops::Sub for &Point3<T> {
  type Output = Vector3<T>;
  fn sub(self, _rhs: Self) -> Self::Output {
    Self::Output::new(
      self.x - _rhs.x, 
      self.y - _rhs.y, 
      self.z - _rhs.z
    )
  }
}

impl<T: Copy + Clone + Signed + MinMax> ops::Sub<Vector3<T>> for Point3<T> {
  type Output = Vector3<T>;
  fn sub(self, _rhs: Vector3<T>) -> Self::Output {
    Self::Output::new(
      self.x - _rhs.x, 
      self.y - _rhs.y, 
      self.z - _rhs.z
    )
  }
}


// Point3 Multiplication
impl<T: Mul<Output = T> + Copy + Clone + Signed + MinMax> ops::Mul<T> for Point3<T> {
  type Output = Self;
  fn mul(self, rhs: T) -> Self::Output {
      Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs)
  }
}

impl<T: Mul<Output=T> + Copy + Clone + Signed + MinMax + Into<RaytracerFloat>> ops::Mul<Point3<T>> for RaytracerFloat {
  type Output = Point3<RaytracerFloat>;
  fn mul(self, rhs: Point3<T>) -> Self::Output {
    Self::Output::new(self * rhs.x.into(), self * rhs.y.into(), self * rhs.z.into())
  }
}

impl<T: Copy + Clone + Signed + MinMax + Into<RaytracerFloat>> ops::Mul<&Point3<T>> for RaytracerFloat {
  type Output = Point3<RaytracerFloat>;
  fn mul(self, rhs: &Point3<T>) -> Self::Output {
    Self::Output::new(self * rhs.x.into(), self * rhs.y.into(), self * rhs.z.into())
  }
}


// Point3 Element access
impl<T: Signed + Copy + MinMax + Zero> ops::Index<usize> for Point3<T> {
  type Output = T;
  fn index(&self, index: usize) -> &Self::Output {
    assert!(index <= 2);
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => &self.x
    }
  }
}


// Point3 Printing
impl<T: Copy + Clone + Signed + MinMax + Display> fmt::Display for Point3<T> {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "Point3({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
  }
}


// Point3 Default
impl<T: Copy + Clone + Signed + MinMax + Zero> Default for Point3<T> {
  fn default() -> Self {
    let val = T::zero();
    Point3::new(val, val, val)
  }
}


#[derive(Clone, Copy)]
pub struct Point2<T: Copy + Clone + Signed + MinMax> {
  x: T,
  y: T
}

impl<T: Copy + Clone + Signed + MinMax> Point2<T> {
  #[allow(dead_code)]
  pub fn new(x: T, y: T) -> Self {
    Self {
      x,
      y,
    }
  }
  #[allow(dead_code)]
  pub fn from_point3(p: &Point3<T>) -> Self {
    Self::new(p.x, p.y)
  }
}

#[allow(dead_code)]
pub type Point3f = Point3<RaytracerFloat>;
#[allow(dead_code)]
pub type Point3i = Point3<RaytracerInt>;

#[allow(dead_code)]
pub type Point2f = Point2<RaytracerFloat>;
#[allow(dead_code)]
pub type Point2i = Point2<RaytracerInt>;
