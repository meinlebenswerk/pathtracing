use core::fmt;
use std::ops::{Neg, Add, Mul, AddAssign, Sub, SubAssign, MulAssign, Div, DivAssign};

use num::{Float, One};

use super::vector3::Vector3;

#[derive(Clone, Copy)]
pub struct Normal3<T> {
  pub x: T,
  pub y: T,
  pub z: T
}

impl<T> Normal3<T> {
  pub fn new(x: T, y: T, z: T) -> Self {
    Self { 
      x,
      y,
      z
    }
  }
}

impl<T> Normal3<T>
where T: Add<T, Output = T> + Mul<T, Output = T> + Copy{
  pub fn length_squared(self) -> T {
    self.x*self.x + self.y*self.y + self.z*self.z
  }
}

impl<T> Normal3<T>
where T: Float {
  pub fn has_nans(&self) -> bool {
    self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
  }

  pub fn length(&self) -> T {
    self.length_squared().sqrt()
  }

  pub fn normalize(&self) -> Self {
    *self / self.length()
  }

  pub fn face_forward(&self, vec: &Vector3<T>) -> Self {
    if vec.dot(&Vector3::from(*self)) < T::zero() { -*self } else { self.clone() }
  }
}

// Conversions
// Operators

impl<T> Neg for Normal3<T>
where T: Neg<Output = T> {
  type Output = Normal3<T>;
  fn neg(self) -> Self::Output {
      Self::Output {
        x: -self.x,
        y: -self.y,
        z: -self.z,
      }
  }
}

impl<T> Add<Normal3<T>> for Normal3<T>
where T: Add<T, Output = T>{
  type Output = Normal3<T>;
  fn add(self, rhs: Normal3<T>) -> Self::Output {
      Self::Output{
        x: self.x + rhs.x,
        y: self.y + rhs.y,
        z: self.z + rhs.z,
      }
  }
}

impl<T> AddAssign<Normal3<T>> for Normal3<T>
where T: AddAssign<T> {
  fn add_assign(&mut self, rhs: Normal3<T>) {
      self.x += rhs.x;
      self.y += rhs.y;
      self.z += rhs.z;
  }
}

impl<T> Sub<Normal3<T>> for Normal3<T>
where T: Sub<T, Output = T>{
  type Output = Normal3<T>;
  fn sub(self, rhs: Normal3<T>) -> Self::Output {
      Self::Output{
        x: self.x - rhs.x,
        y: self.y - rhs.y,
        z: self.z - rhs.z,
      }
  }
}

impl<T> SubAssign<Normal3<T>> for Normal3<T>
where T: SubAssign<T> {
  fn sub_assign(&mut self, rhs: Normal3<T>) {
      self.x -= rhs.x;
      self.y -= rhs.y;
      self.z -= rhs.z;
  }
}


impl<T> Mul<T> for Normal3<T>
where T: Mul<T, Output = T> + Copy {
  type Output = Normal3<T>;
  fn mul(self, rhs: T) -> Self::Output {
      Self::Output {
        x: self.x * rhs,
        y: self.y * rhs,
        z: self.z * rhs,
      }
  }
}

impl<T> MulAssign<T> for Normal3<T>
where T: MulAssign<T> + Copy {
  fn mul_assign(&mut self, rhs: T) {
      self.x *= rhs;
      self.y *= rhs;
      self.z *= rhs;
  }
}


impl<T> Div<T> for Normal3<T>
where T: Mul<T, Output = T> + Div<T, Output = T> + One + Copy {
  type Output = Normal3<T>;
  fn div(self, rhs: T) -> Self::Output {
    let inv = T::one() / rhs;
    Self::Output {
      x: self.x * inv,
      y: self.y * inv,
      z: self.z * inv,
    }
  }
}

impl<T> DivAssign<T> for Normal3<T>
where T: MulAssign<T> + Div<T, Output = T> + One + Copy {
  fn div_assign(&mut self, rhs: T) {
    let inv = T::one() / rhs;
    self.x *= inv;
    self.y *= inv;
    self.z *= inv;
  }
}

impl<T> PartialEq<Normal3<T>> for Normal3<T> 
  where T: PartialEq<T> {
    fn eq(&self, other: &Normal3<T>) -> bool {
        self.x == other.x && self.x == other.y && self.z == other.z
    }

    fn ne(&self, other: &Normal3<T>) -> bool {
      self.x != other.x || self.y != other.y || self.z != other.z
    }
  }


// Printing
impl<T> fmt::Display for Normal3<T>
where T: fmt::Display {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Normal3 ({}, {}, {})", self.x, self.y, self.z)
  }
}