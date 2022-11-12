use core::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg, Index};

use num::{Zero, Float, One};

use crate::config::{RaytracerFloat, RaytracerInt};

use super::vector3::Vector3;

#[derive(Copy, Clone)]
pub struct Point3<T> {
  pub x: T,
  pub y: T,
  pub z: T
}

impl<T> Point3<T> {
  pub fn new(x: T, y: T, z: T) -> Self {
    return Self {
      x,
      y,
      z
    }
  }
}

impl<T> Point3<T>
where T: Float {
  pub fn has_nans(&self) -> bool {
    self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
  }
}

// Conversions
impl<T> From<Vector3<T>> for Point3<T> {
  fn from(vec: Vector3<T>) -> Self {
    Self {
      x: vec.x,
      y: vec.y,
      z: vec.z,
    }
  }
}

// Operators
impl<T> Add<Vector3<T>> for Point3<T>
where T: Add<T, Output = T> {
  type Output = Point3<T>;
  fn add(self, rhs: Vector3<T>) -> Self::Output {
   Self::Output {
    x: self.x + rhs.x,
    y: self.y + rhs.y,
    z: self.z + rhs.z
   }   
  }
}

impl<T> Add<Point3<T>> for Point3<T>
where T: Add<T, Output = T> {
  type Output = Point3<T>;
  fn add(self, rhs: Point3<T>) -> Self::Output {
   Self::Output {
    x: self.x + rhs.x,
    y: self.y + rhs.y,
    z: self.z + rhs.z
   }   
  }
}

impl<T> AddAssign<Vector3<T>> for Point3<T>
where T: AddAssign<T> {
  fn add_assign(&mut self, rhs: Vector3<T>) {
      self.x += rhs.x;
      self.y += rhs.y;
      self.z += rhs.z;
  }
}

impl<T> Sub<Point3<T>> for Vector3<T>
where T: Sub<T, Output = T> {
  type Output = Vector3<T>;
  fn sub(self, rhs: Point3<T>) -> Self::Output {
   Self::Output {
    x: self.x - rhs.x,
    y: self.y - rhs.y,
    z: self.z - rhs.z
   }   
  }
}

impl<T> Sub<Vector3<T>> for Point3<T>
where T: Sub<T, Output = T> {
  type Output = Point3<T>;
  fn sub(self, rhs: Vector3<T>) -> Self::Output {
   Self::Output {
    x: self.x - rhs.x,
    y: self.y - rhs.y,
    z: self.z - rhs.z
   }   
  }
}

impl<T> Sub<Point3<T>> for Point3<T>
where T: Sub<T, Output = T> {
  type Output = Vector3<T>;
  fn sub(self, rhs: Point3<T>) -> Self::Output {
   Self::Output {
    x: self.x - rhs.x,
    y: self.y - rhs.y,
    z: self.z - rhs.z
   }   
  }
}


impl<T> SubAssign<Vector3<T>> for Point3<T> 
where T: SubAssign<T> {
  fn sub_assign(&mut self, rhs: Vector3<T>) {
      self.x -= rhs.x;
      self.y -= rhs.y;
      self.z -= rhs.z;
  }
}

impl<T> AddAssign<Point3<T>> for Point3<T> 
where T: AddAssign<T> {
  fn add_assign(&mut self, rhs: Point3<T>) {
      self.x += rhs.x;
      self.y += rhs.y;
      self.z += rhs.z;
  }
}

impl<T> Mul<T> for Point3<T>
where T: Mul<T, Output = T> + Copy {
  type Output = Point3<T>;
  fn mul(self, rhs: T) -> Self::Output {
      Self::Output {
        x: self.x * rhs,
        y: self.y * rhs,
        z: self.z * rhs,
      }
  }
}

impl<T> MulAssign<T> for Point3<T>
where T: MulAssign<T> + Copy {
  fn mul_assign(&mut self, rhs: T) {
      self.x *= rhs;
      self.y *= rhs;
      self.z *= rhs;
  }
}


impl<T> Div<T> for Point3<T>
where T: Mul<T, Output = T> + Div<T, Output = T> + One + Copy {
  type Output = Point3<T>;
  fn div(self, rhs: T) -> Self::Output {
    let inv = T::one() / rhs;
    Self::Output {
      x: self.x * inv,
      y: self.y * inv,
      z: self.z * inv,
    }
  }
}

impl<T> DivAssign<T> for Point3<T>
where T: MulAssign<T> + Div<T, Output = T> + One + Copy {
  fn div_assign(&mut self, rhs: T) {
    let inv = T::one() / rhs;
    self.x *= inv;
    self.y *= inv;
    self.z *= inv;
  }
}

impl<T> Neg for Point3<T>
where T: Neg<Output = T> {
  type Output = Point3<T>;
  fn neg(self) -> Self::Output {
      Self::Output {
        x: -self.x,
        y: -self.y,
        z: -self.z,
      }
  }
}

impl<T> PartialEq<Point3<T>> for Point3<T>
where T: PartialEq<T> {
  fn eq(&self, other: &Point3<T>) -> bool {
      self.x == other.x && self.y == other.y && self.z == other.z
  }

  fn ne(&self, other: &Point3<T>) -> bool {
    self.x != other.x || self.y != other.y || self.z != other.z
  }
}

impl<T> Index<usize> for Point3<T> {
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


// Default 
impl<T> Default for Point3<T>
where T: Zero {
  fn default() -> Self {
      Self {
        x: T::zero(),
        y: T::zero(),
        z: T::zero()
      }
  }
}

// Printing
impl<T> fmt::Display for Point3<T>
where T: fmt::Display {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Point3 ({}, {}, {})", self.x, self.y, self.z)
  }
}


#[allow(dead_code)]
pub type Point3f = Point3<RaytracerFloat>;
#[allow(dead_code)]
pub type Point3i = Point3<RaytracerInt>;