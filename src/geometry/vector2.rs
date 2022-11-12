use std::{fmt, ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg}};

use num::{Float, Zero};

use crate::config::{RaytracerFloat, RaytracerInt};

use super::point::{Point2, Point3};

#[derive(Clone, Copy)]
pub struct Vector2<T> {
  pub x: T,
  pub y: T
}

impl<T> Vector2<T> {
  pub fn new(x: T, y: T) -> Self {
    Self {
      x,
      y
    }
  }
}

impl<T> Vector2<T>
  where T: Float {
    pub fn has_nans(&self) -> bool {
      self.x.is_nan() || self.y.is_nan()
    }

    pub fn length_squared(&self) -> T {
      self.x*self.x + self.y*self.y
    }

    pub fn length(&self) -> T {
      self.length_squared().sqrt()
    }
}

// Conversions
impl<T> From<Point2<T>> for Vector2<T>
  where T: Copy + Clone {
  fn from(point: Point2<T>) -> Self {
      Self::new(point.x, point.y)
  }
}

impl<T> From<Point3<T>> for Vector2<T>
  where T: Copy + Clone {
  fn from(point: Point3<T>) -> Self {
      Self::new(point.x, point.y)
  }
}

// Operators
impl<T> Add<Vector2<T>> for Vector2<T>
  where T: Add<T, Output = T> {
    type Output = Vector2<T>;
    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Self::Output::new(
          self.x + rhs.x,
          self.y + rhs.y
        )
    }
}

impl<T> AddAssign<Vector2<T>> for Vector2<T>
  where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Vector2<T>) {
        self.x += rhs.x;
        self.y += rhs.y
    }
  }

impl<T> Sub<Vector2<T>> for Vector2<T>
  where T: Sub<T, Output = T> {
    type Output = Vector2<T>;
    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Self::Output::new(
          self.x - rhs.x,
          self.y - rhs.y
        )
    }
}

impl<T> SubAssign<Vector2<T>> for Vector2<T>
  where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Vector2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y
    }
  }

impl<T> PartialEq<Vector2<T>> for Vector2<T> 
  where T: PartialEq<T> {
    fn eq(&self, other: &Vector2<T>) -> bool {
        self.x == other.x && self.x == other.y
    }

    fn ne(&self, other: &Vector2<T>) -> bool {
      self.x != other.x || self.x != other.y
    }
  }

impl<T> Mul<T> for Vector2<T>
  where T: Mul<T, Output = T> {
    type Output = Vector2<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output::new(
          self.x * rhs,
          self.y * rhs
        )
    }
}

impl<T> MulAssign<T> for Vector2<T>
  where T: MulAssign<T> {
    fn mul_assign(&mut self, rhs: T) {
      self.y *= rhs;
      self.x *= rhs;
    }
}

impl<T> Div<T> for Vector2<T>
  where T: Div<T, Output = T> {
    type Output = Vector2<T>;
    fn div(self, rhs: T) -> Self::Output {
        Self::Output::new(
          self.x / rhs,
          self.y / rhs
        )
    }
}

impl<T> DivAssign<T> for Vector2<T>
  where T: DivAssign<T> {
    fn div_assign(&mut self, rhs: T) {
      self.y /= rhs;
      self.x /= rhs;
    }
}

impl<T> Neg for Vector2<T>
  where T: Neg<Output = T> {
    type Output = Vector2<T>;
    fn neg(self) -> Self::Output {
      Self::Output::new(-self.x, -self.y)
    }
  }

// Default 
impl<T> Default for Vector2<T> 
  where T: Zero {
    fn default() -> Self {
        Self {
          x: T::zero(),
          y: T::zero(),
        }
    }
}

// Printing
impl<T> fmt::Display for Vector2<T>
where T: fmt::Display {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Vector2 ({}, {})", self.x, self.y)
  }
}


pub type Vector2f = Vector2<RaytracerFloat>;
pub type Vector2i = Vector2<RaytracerInt>;