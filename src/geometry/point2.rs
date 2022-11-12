use num::{Float, Zero, One};
use std::{fmt, ops::{Add, AddAssign, Sub, Neg, SubAssign, Mul, MulAssign, Div, DivAssign}};
use crate::config::{RaytracerFloat, RaytracerInt};

use super::vector2::Vector2;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Point2<T> {
  pub x: T,
  pub y: T
}

impl<T> Point2<T> {
  pub fn new(x: T, y: T) -> Self {
    Self {
      x,
      y
    }
  }
}

impl<T> Point2<T>
where T: Float {
  pub fn has_nans(&self) -> bool {
    self.x.is_nan() || self.y.is_nan()
  }
}

// Conversions
impl<T> From<Vector2<T>> for Point2<T>
  where T: Copy + Clone {
  fn from(vector: Vector2<T>) -> Self {
      Self::new(vector.x, vector.y)
  }
}

// Operators
impl<T> Add<Vector2<T>> for Point2<T>
where T: Add<T, Output = T> {
  type Output = Point2<T>;

  fn add(self, rhs: Vector2<T>) -> Self::Output {
      Self::Output {
        x: self.x + rhs.x,
        y: self.y + rhs.y
      }
  }
}

impl<T> Add<Point2<T>> for Point2<T>
where T: Add<T, Output = T> {
  type Output = Point2<T>;

  fn add(self, rhs: Point2<T>) -> Self::Output {
      Self::Output {
        x: self.x + rhs.x,
        y: self.y + rhs.y
      }
  }
}

impl<T> AddAssign<Vector2<T>> for Point2<T>
where T: AddAssign<T> {
  fn add_assign(&mut self, rhs: Vector2<T>) {
      self.x += rhs.x;
      self.y += rhs.y;
  }
}

impl<T> Sub<Point2<T>> for Vector2<T>
where T: Sub<T, Output = T> {
  type Output = Vector2<T>;
  fn sub(self, rhs: Point2<T>) -> Self::Output {
      Self::Output {
        x: self.x - rhs.x,
        y: self.y - rhs.y,
      }
  }
}

impl<T> Sub<Point2<T>> for Point2<T>
where T: Sub<T, Output = T> {
  type Output = Vector2<T>;
  fn sub(self, rhs: Point2<T>) -> Self::Output {
      Self::Output {
        x: self.x - rhs.x,
        y: self.y - rhs.y,
      }
  }
}

impl<T> Sub<&Point2<T>> for &Point2<T>
where T: Sub<T, Output = T> + Copy + Clone {
  type Output = Vector2<T>;
  fn sub(self, rhs: &Point2<T>) -> Self::Output {
      Self::Output {
        x: self.x - rhs.x,
        y: self.y - rhs.y,
      }
  }
}


impl<T> Sub<Vector2<T>> for Point2<T>
where T: Sub<T, Output = T> {
  type Output = Point2<T>;
  fn sub(self, rhs: Vector2<T>) -> Self::Output {
      Self::Output {
        x: self.x - rhs.x,
        y: self.y - rhs.y
      }
  }
}

impl<T> Neg for Point2<T>
where T: Neg<Output = T> {
  type Output = Point2<T>;
  fn neg(self) -> Self::Output {
      Self::Output {
        x: -self.x,
        y: -self.y,
      }
  }
}

impl<T> PartialEq<Point2<T>> for Point2<T>
where T: PartialEq<T> {
  fn eq(&self, other: &Point2<T>) -> bool {
      self.x == other.x && self.y == other.y
  }

  fn ne(&self, other: &Point2<T>) -> bool {
      return self.x != other.x || self.x != other.y
  }
}

impl<T> SubAssign<Vector2<T>> for Point2<T>
where T: SubAssign<T> {
  fn sub_assign(&mut self, rhs: Vector2<T>) {
      self.x -= rhs.x;
      self.y -= rhs.y;
  }
}

impl<T> AddAssign<Point2<T>> for Point2<T>
where T: AddAssign<T> {
  fn add_assign(&mut self, rhs: Point2<T>) {
      self.x += rhs.x;
      self.y += rhs.y;
  }
}

impl<T> Mul<T> for Point2<T>
where T: Mul<T, Output = T> + Copy {
  type Output = Point2<T>;
  fn mul(self, rhs: T) -> Self::Output {
      Self::Output {
        x: self.x * rhs,
        y: self.y * rhs
      }
  }
} 

impl<T> MulAssign<T> for Point2<T>
where T: MulAssign<T> + Copy {
  fn mul_assign(&mut self, rhs: T) {
      self.x *= rhs;
      self.y *= rhs;
  }
} 

impl<T> Div<T> for Point2<T>
where T: Div<T, Output = T> + Mul<T, Output = T> + One + Copy {
  type Output = Point2<T>;
  fn div(self, rhs: T) -> Self::Output {
    let inv = T::one() / rhs;
    Self::Output {
      x: self.x * inv,
      y: self.y * inv
    }
  }
} 

impl<T> DivAssign<T> for Point2<T>
where T: MulAssign<T> + Div<T, Output = T> + One + Copy {
  fn div_assign(&mut self, rhs: T) {
    let inv = T::one() / rhs;
    self.x *= inv;
    self.y *= inv;
  }
} 

// Default 
impl<T> Default for Point2<T> 
  where T: Zero {
    fn default() -> Self {
        Self {
          x: T::zero(),
          y: T::zero(),
        }
    }
}

// Printing
impl<T> fmt::Display for Point2<T>
where T: fmt::Display {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Point2 ({}, {})", self.x, self.y)
  }
}

#[allow(dead_code)]
pub type Point2f = Point2<RaytracerFloat>;
#[allow(dead_code)]
pub type Point2i = Point2<RaytracerInt>;
