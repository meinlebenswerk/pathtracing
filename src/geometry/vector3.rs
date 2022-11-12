use std::{fmt, ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg, Index}};
use num::{Float, Zero, One, Signed};

use crate::config::{RaytracerFloat, RaytracerInt};

use super::{point3::Point3, normal3::Normal3};

#[derive(Clone, Copy)]
pub struct Vector3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
      Self {
        x,
        y,
        z
      }
    }
}

impl<T> Vector3<T> 
  where T: Float {
    pub fn has_nans(&self) -> bool {
      self.x.is_nan() || self.y.is_nan() || self.z.is_nan()      
    }

    pub fn length(&self) -> T {
      return self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
      *self / self.length()
    }

    pub fn coordinate_system(&self) -> [Vector3<T>; 3] {
      let v2 = if self.x.abs() > self.y.abs() {
        Self {
          x: -self.z,
          y: T::zero(),
          z: self.x
        } / (self.x * self.x + self.z * self.z).sqrt()
      } else {
        Self {
          x: T::zero(),
          y: self.z,
          z: -self.y
        } / (self.y * self.y + self.z * self.z).sqrt()
      };
      let mut v3 = self.cross(&v2);

      [ self.clone(), v2, v3 ]
    }

    pub fn inverse(&self) -> Self {
      Self {
        x: T::one() / self.x,
        y: T::one() / self.y,
        z: T::one() / self.z
      }
    }

    pub fn mul_elementwise(&self, other: &Vector3<T>) -> Self {
      Self {
        x: self.x * other.x,
        y: self.y * other.y,
        z: self.z * other.z
      }
    }

    pub fn min_elementwise(&self, other: &Vector3<T>) -> Self {
      Self {
        x: self.x.min(other.x),
        y: self.y.min(other.y),
        z: self.z.min(other.z)
      }
    }

    pub fn max_elementwise(&self, other: &Vector3<T>) -> Self {
      Self {
        x: self.x.max(other.x),
        y: self.y.max(other.y),
        z: self.z.max(other.z)
      }
    }

    pub fn near_zero(&self) -> bool {
      self.length_squared() < T::epsilon()
    }
}

impl<T> Vector3<T> 
where T: Mul<T, Output = T> + Add<T, Output = T> + Sub<T, Output = T> + Copy {
  pub fn length_squared(&self) -> T {
    self.x*self.x + self.y*self.y
  }

  pub fn dot(&self, rhs: &Self) -> T {
    self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
  }

  pub fn cross(&self, rhs: &Self) -> Self {
    Self {
      x: (self.y * rhs.z) - (self.z * rhs.y),
      y: (self.z * rhs.x) - (self.x * rhs.z),
      z: (self.x * rhs.y) - (self.y * rhs.x),
    }
  }
}

impl<T> Vector3<T> 
where T: Mul<T, Output = T> + Add<T, Output = T> + Signed + Copy {
  pub fn abs_dot(&self, rhs: &Self) -> T {
    self.dot(rhs).abs()
  }
}

impl<T> Vector3<T>
where T: Signed {
    pub fn abs(&self) -> Self {
      Self {
        x: self.x.abs(),
        y: self.y.abs(),
        z: self.z.abs()
      }
    }
  }

impl<T> Vector3<T> 
where T: Ord + Copy {
  pub fn max_component(&self) -> T {
    self.x.max(self.y.max(self.z))
  }

  pub fn min_component(&self) -> T {
    self.x.min(self.y.min(self.z))
  }

  pub fn max_dimension(&self) -> usize {
    if self.x > self.y { if self.x > self.z { 0 } else { 2 } } else { if self.y > self.z { 1 } else { 2 } }
  }

  pub fn max(&self, other: Vector3<T>) -> Self {
    Self {
      x: self.x.max(other.x),
      y: self.y.max(other.y),
      z: self.z.max(other.z),
    }
  }

  pub fn min(&self, other: Vector3<T>) -> Self {
    Self {
      x: self.x.min(other.x),
      y: self.y.min(other.y),
      z: self.z.min(other.z),
    }
  }
}

// Conversions
impl<T> From<Point3<T>> for Vector3<T>
  where T: Copy + Clone {
  fn from(point: Point3<T>) -> Self {
      Self::new(point.x, point.y, point.z)
  }
}

impl<T> From<Normal3<T>> for Vector3<T>
  where T: Copy + Clone {
  fn from(point: Normal3<T>) -> Self {
      Self::new(point.x, point.y, point.z)
  }
}

impl<T> From<T> for Vector3<T>
  where T: Copy + Clone {
  fn from(val: T) -> Self {
      Self {
        x: val,
        y: val,
        z: val
      }
  }
}

// impl<T> From<Normal3<T>> for Vector3<T>
//   where T: Copy + Clone {
//   fn from(point: Point3<T>) -> Self {
//       Self::new(point.x, point.y, point.z)
//   }
// }

// Operators
impl<T> Add<Vector3<T>> for Vector3<T>
  where T: Add<T, Output = T> {
    type Output = Vector3<T>;
    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Self::Output::new(
          self.x + rhs.x,
          self.y + rhs.y,
          self.z + rhs.z
        )
    }
}

impl<T> AddAssign<Vector3<T>> for Vector3<T>
  where T: AddAssign<T> {
    fn add_assign(&mut self, rhs: Vector3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
  }

impl<T> Sub<Vector3<T>> for Vector3<T>
  where T: Sub<T, Output = T> {
    type Output = Vector3<T>;
    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Self::Output::new(
          self.x - rhs.x,
          self.y - rhs.y,
          self.z - rhs.z
        )
    }
}

impl<T> SubAssign<Vector3<T>> for Vector3<T>
  where T: SubAssign<T> {
    fn sub_assign(&mut self, rhs: Vector3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
  }

impl<T> PartialEq<Vector3<T>> for Vector3<T> 
  where T: PartialEq<T> {
    fn eq(&self, other: &Vector3<T>) -> bool {
        self.x == other.x && self.x == other.y && self.z == other.z
    }

    fn ne(&self, other: &Vector3<T>) -> bool {
      self.x != other.x || self.y != other.y || self.z != other.z
    }
  }

impl<T> Mul<T> for Vector3<T>
  where T: Mul<T, Output = T> + Copy {
    type Output = Vector3<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output::new(
          self.x * rhs,
          self.y * rhs,
          self.z * rhs
        )
    }
}

impl<T> MulAssign<T> for Vector3<T>
  where T: MulAssign<T> + Copy {
    fn mul_assign(&mut self, rhs: T) {
      self.x *= rhs;
      self.y *= rhs;
      self.z *= rhs;
    }
}

impl<T> Div<T> for Vector3<T>
  where T: Mul<T, Output = T> + Div<T, Output = T> + One + Copy {
    type Output = Vector3<T>;
    fn div(self, rhs: T) -> Self::Output {
      let inv = T::one() / rhs;
      Self::Output::new(
        self.x * inv,
        self.y * inv,
        self.z * inv
      )
    }
}

impl<T> DivAssign<T> for Vector3<T>
  where T: One +  MulAssign<T> + Div<T, Output = T> + Copy {
    fn div_assign(&mut self, rhs: T) {
      let inv = T::one() / rhs;
      self.x *= inv;
      self.y *= inv;
      self.z *= inv;
    }
}

impl<T> Neg for Vector3<T>
where T: Neg<Output = T> {
  type Output = Vector3<T>;

  fn neg(self) -> Self::Output {
    Self::Output::new(-self.x, -self.y, -self.z)
  }
}

impl<T> Index<usize> for Vector3<T> {
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
impl<T> Default for Vector3<T>
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
impl<T> fmt::Display for Vector3<T>
where T: fmt::Display {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Vector3 ({}, {}, {})", self.x, self.y, self.z)
  }
}


#[allow(dead_code)]
pub type Vector3f = Vector3<RaytracerFloat>;
#[allow(dead_code)]
pub type Vector3i= Vector3<RaytracerInt>;
#[allow(dead_code)]
pub type Normal3f = Vector3<RaytracerFloat>;
#[allow(dead_code)]
pub type Normal3i = Vector3<RaytracerInt>;


// Vector3 Testing
#[cfg(test)]
mod tests {
  use super::*;
  use float_cmp::{approx_eq};
  #[test]
  fn test_vector_new() {
    let vector = Vector3::new(1.0, 1.5, 2.0);
    assert!( approx_eq!(f32, vector.x, 1.0) );
    assert!( approx_eq!(f32, vector.y, 1.5) );
    assert!( approx_eq!(f32, vector.z, 2.0) );
  }

  #[test]
  fn test_vector_dot() {
    let vector = Vector3::new(1.0, 1.5, 2.0);
    let dot = vector.dot(&vector);
    assert!( approx_eq!(f32, dot, vector.length_squared()) );
  }

  #[test]
  fn test_vector_cross() {
    let va = Vector3::new(1.0, 0.0, 0.0);
    let vb = Vector3::new(0.0, 1.0, 0.0);
    let cross = va.cross(&vb);
    assert!( approx_eq!(f32, cross.x, 0.0) );
    assert!( approx_eq!(f32, cross.y, 0.0) );
    assert!( approx_eq!(f32, cross.z, 1.0) );
  }

  #[test]
  fn test_vector_add_vector() {
    let va = Vector3::new(1.0, 1.5, 2.0);
    let vb = Vector3::new(1.0, 1.5, 2.0);

    let vc = va + vb;

    assert!( approx_eq!(f32, vc.x, 2.0) );
    assert!( approx_eq!(f32, vc.y, 3.0) );
    assert!( approx_eq!(f32, vc.z, 4.0) );
  }

  #[test]
  fn test_vector_add_vector_neg() {
    let va = Vector3::new(1.0, 1.5, 2.0);
    let vb = Vector3::new(1.0, 1.5, 2.0);

    let vc = va + (-vb);

    assert!( approx_eq!(f32, vc.x, 0.0) );
    assert!( approx_eq!(f32, vc.y, 0.0) );
    assert!( approx_eq!(f32, vc.z, 0.0) );
  }

  #[test]
  fn test_vector_add_assign_vector() {
    let mut va = Vector3::new(1.0, 1.5, 2.0);
    let vb = Vector3::new(1.0, 1.5, 2.0);
    va += vb;

    assert!( approx_eq!(f32, va.x, 2.0) );
    assert!( approx_eq!(f32, va.y, 3.0) );
    assert!( approx_eq!(f32, va.z, 4.0) );
  }

  #[test]
  fn test_vector_sub_vector() {
    let va = Vector3::new(1.0, 1.5, 2.0);
    let vb = Vector3::new(2.0, 1.5, 1.0);

    let vc = va - vb;

    assert!( approx_eq!(f32, vc.x, -1.0) );
    assert!( approx_eq!(f32, vc.y,  0.0) );
    assert!( approx_eq!(f32, vc.z,  1.0) );
  }

  #[test]
  fn test_vector_sub_assign_vector() {
    let mut va = Vector3::new(1.0, 1.5, 2.0);
    let vb = Vector3::new(2.0, 1.5, 1.0);
    va -= vb;

    assert!( approx_eq!(f32, va.x, -1.0) );
    assert!( approx_eq!(f32, va.y,  0.0) );
    assert!( approx_eq!(f32, va.z,  1.0) );
  }


  #[test]
  fn test_vector_mul_f32() {
    let va = Vector3::new(1.0, 1.5, 2.0);
    let vb = va * 2.0;

    assert!( approx_eq!(f32, vb.x, 2.0) );
    assert!( approx_eq!(f32, vb.y, 3.0) );
    assert!( approx_eq!(f32, vb.z, 4.0) );
  }

  #[test]
  fn test_vector_mul_assign_f32() {
    let mut va = Vector3::new(1.0, 1.5, 2.0);
    va *= 2.0;

    assert!( approx_eq!(f32, va.x, 2.0));
    assert!( approx_eq!(f32, va.y, 3.0));
    assert!( approx_eq!(f32, va.z, 4.0));
  }


  #[test]
  fn test_vector_div_f32() {
    let va = Vector3::new(2.0, 4.0, 8.0);
    let vb = va / 2.0;

    assert!( approx_eq!(f32, vb.x, 1.0) );
    assert!( approx_eq!(f32, vb.y, 2.0) );
    assert!( approx_eq!(f32, vb.z, 4.0) );
  }

  #[test]
  fn test_vector_div_assign_f32() {
    let mut va = Vector3::new(2.0, 4.0, 8.0);
    va /= 2.0;

    assert!( approx_eq!(f32, va.x, 1.0) );
    assert!( approx_eq!(f32, va.y, 2.0) );
    assert!( approx_eq!(f32, va.z, 4.0) );
  }

  #[test]
  fn test_vector_length_squared() {
    let va = Vector3::new(2.0, 4.0, 8.0);
    assert!( approx_eq!(f32, va.length_squared(), 84.0) );
  }

  #[test]
  fn test_vector_length() {
    let va = Vector3::new(0.0, 3.0, 4.0);
    assert!( approx_eq!(f32, va.length(), 5.0) );
  }

  #[test]
  fn test_vector_normalize() {
    let va = Vector3::new(10.0, 3.0, 4.0).normalize();
    assert!( approx_eq!(f32, va.length(), 1.0) );
  }
}