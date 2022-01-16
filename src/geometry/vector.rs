use std::fmt::Display;
use std::ops;
use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use num::Float;
use num::Integer;
use num::NumCast;
use num::One;
use num::Signed;
use num::Zero;

use crate::config::RaytracerFloat;
use crate::config::RaytracerInt;

use super::point::Point3;
use super::utils::MinMax;

#[derive(Clone, Copy)]
pub struct Vector3<T> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T> Vector3<T>
  where T: Copy + Clone + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Zero {
  pub fn new(x: T, y: T, z: T) -> Self {
    Self {
      x,
      y,
      z,
    }
  }

  pub fn length_squared(&self) -> T {
    (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
  }

  pub fn dot(&self, vec: &Vector3<T>) -> T {
    (self.x * vec.x) + (self.y * vec.y) + (self.z * vec.z)
  }

  pub fn cross(&self, vec: &Vector3<T>) -> Self {
    let x = self.y * vec.z - self.z * vec.y;
    let y = self.z * vec.x - self.x * vec.z;
    let z = self.x * vec.y - self.y * vec.x;
    Self::new(x, y, z)
  }

  pub fn decompose(&self) -> [Vector3<T>; 3] {
    [
      Vector3::new(self.x, T::zero(), T::zero()),
      Vector3::new(T::zero(), self.y, T::zero()),
      Vector3::new(T::zero(), T::zero(), self.z),
    ]
  }

  pub fn as_Point3(&self) -> Point3<T> {
    Point3::new(self.x, self.y, self.z)
  }

  pub fn column(&self) -> Vec<&T> {
    vec![&self.x, &self.y, &self.z]
  }

  pub fn row(&self, y: usize) -> Vec<&T> {
    assert!(y < 3);
    match y {
      0 => vec![&self.x],
      1 => vec![&self.y],
      2 => vec![&self.z],
      _ => vec![]
    }
  }
}

impl<T> Vector3<T> 
  where T: Copy + Clone + Zero + Signed {
  pub fn abs(&self) -> Self {
    Self::new(
      self.x.abs(), 
      self.y.abs(), 
      self.z.abs()
    )
  }
}

impl<T> Vector3<T> 
  where T: Copy + Clone + Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Zero + MinMax  {
    pub fn min_elementwise(a: &Vector3<T>, b: &Vector3<T>) -> Self {
      let x = MinMax::min(&a.x, &b.x);
      let y = MinMax::min(&a.y, &b.y);
      let z = MinMax::min(&a.z, &b.z);
      Self::new(x, y, z)
    }

    pub fn max_elementwise(a: &Vector3<T>, b: &Vector3<T>) -> Self {
      let x = MinMax::max(&a.x, &b.x);
      let y = MinMax::max(&a.y, &b.z);
      let z = MinMax::max(&a.z, &b.z);
      Self::new(x, y, z)
    }

    pub fn clamp(&mut self, min: &T, max: &T) {
      self.x = MinMax::max(min, &MinMax::min(&self.x, max));
      self.y = MinMax::max(min, &MinMax::min(&self.y, max));
      self.z = MinMax::max(min, &MinMax::min(&self.z, max));
    }
}

impl<T> Vector3<T>
where T: Copy + Clone + Signed + MinMax + Zero + Float + One  {
  
  pub fn length(&self) -> T {
    self.length_squared().sqrt()
  }

  pub fn normalize(&self) -> Self {
    let inv_length = T::one() / self.length();
    Self::new(
      self.x * inv_length,
      self.y * inv_length,
      self.z * inv_length
    )
  }

  pub fn near_zero(&self) -> bool {
    self.length_squared() < T::epsilon()
  }

  pub fn from_hex(color: &str) -> Self {
    // only accept #rbg or #rrbbgg strings
    let len = color.len();
    assert!( len == 4 || len == 7 );
    let bytes_per_field = if len == 4 { 1 } else { 2 };

    let max_value: T = if bytes_per_field == 1 { NumCast::from(0xf).unwrap() } else { NumCast::from(0xff).unwrap() };

    let max_value_inv = T::one() / max_value;
    let r = i32::from_str_radix(&color[1..(1 + bytes_per_field )], 16).unwrap_or(0);
    let g = i32::from_str_radix(&color[(1 + bytes_per_field)..(1 + 2*bytes_per_field )], 16).unwrap_or(0);
    let b = i32::from_str_radix(&color[(1 + 2*bytes_per_field)..(1 + 3*bytes_per_field )], 16).unwrap_or(0);

    let rf: T = NumCast::from(r).unwrap();
    let rg: T = NumCast::from(g).unwrap();
    let rb: T = NumCast::from(b).unwrap();
    
    
    Self::new(rf * max_value_inv, rg * max_value_inv, rb * max_value_inv)
  }
}

impl<T> Vector3<T>
where T: Copy + Clone + Signed + MinMax + Zero + Integer {

  // pub fn near_zero(&self) -> bool {
  //   self.length_squared() == 0
  // }

}

impl<T: Zero + Copy + Clone + Signed + MinMax> Default for Vector3<T> {
  fn default() -> Self {
    let val = T::zero();
    Self::new(val, val, val)
  }
}

// Operators

// Addition
impl<T: Add<Output=T> + Copy + Clone + Signed + MinMax + Zero> ops::Add for Vector3<T>{
  type Output = Self;
  fn add(self, _rhs: Self) -> Self::Output {
    Self::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
  }
}

impl ops::Add<f32> for Vector3<f32> {
  type Output = Self;
  fn add(self, _rhs: f32) -> Self::Output {
    Self::new(self.x + _rhs, self.y + _rhs, self.z + _rhs)
  }
}

impl<T: AddAssign> ops::AddAssign for Vector3<T> {
  fn add_assign(&mut self, _rhs: Self) {
    self.x += _rhs.x;
    self.y += _rhs.y;
    self.z += _rhs.z;
  }
}

// Subtraction
impl<T: Sub<Output=T> + Copy + Clone + Signed + MinMax + Zero> ops::Sub for Vector3<T> {
  type Output = Self;
  fn sub(self, _rhs: Self) -> Self::Output {
    Self::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
  }
}

impl ops::Sub<Vector3<f32>> for f32 {
  type Output = Vector3<f32>;
  fn sub(self, _rhs: Vector3<f32>) -> Self::Output {
    Self::Output::new(self - _rhs.x, self- _rhs.y, self - _rhs.z)
  }
}

impl<T: SubAssign> ops::SubAssign for Vector3<T> {
  fn sub_assign(&mut self, _rhs: Self) {
    self.x -= _rhs.x;
    self.y -= _rhs.y;
    self.z -= _rhs.z;
  }
}

// Negation
impl<T: Neg<Output=T> + Copy + Clone + Signed + MinMax + Zero> ops::Neg for Vector3<T> {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Self::new(-self.x, -self.y, -self.z)
  }
}

// Multiplication
impl<T: Mul<Output=T> + Copy + Clone + Signed + MinMax + Zero> ops::Mul<T> for Vector3<T> {
  type Output = Self;
  fn mul(self, _rhs: T) -> Self::Output {
    Self::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
  }
}

impl<T: Mul<Output=T> + Copy + Clone + Signed + MinMax + Zero> ops::Mul<T> for &Vector3<T> {
  type Output = Vector3<T>;
  fn mul(self, _rhs: T) -> Self::Output {
    Self::Output::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
  }
}

impl ops::Mul<Vector3<f32>> for f32 {
  type Output = Vector3<f32>;
  fn mul(self, _rhs: Vector3<f32>) -> Self::Output {
    Self::Output::new(_rhs.x * self, _rhs.y * self, _rhs.z * self )
  }
}

impl<T: Mul<Output=T> + Copy + Clone + Signed + MinMax + Zero> ops::Mul<Vector3<T>> for Vector3<T> {
  type Output = Self;
  fn mul(self, _rhs: Self) -> Self::Output {
    Self::Output::new(_rhs.x * self.x, _rhs.y *  self.y, _rhs.z *  self.z)
  }
}

impl<T: Mul<Output=T> + Copy + Clone + Signed + MinMax + Zero> ops::Mul<&Vector3<T>> for Vector3<T> {
  type Output = Self;
  fn mul(self, _rhs: &Self) -> Self::Output {
    Self::Output::new(_rhs.x * self.x, _rhs.y *  self.y, _rhs.z *  self.z)
  }
}

impl<T: MulAssign + Copy + Clone + Signed + MinMax + Zero> ops::MulAssign<T> for Vector3<T> {
  fn mul_assign(&mut self, _rhs: T) {
    self.x *= _rhs;
    self.y *= _rhs;
    self.z *= _rhs;
  }
}

// Division
impl<T: Div<Output=T> + Copy + Clone + Signed + MinMax + Zero> ops::Div<T> for Vector3<T> {
  type Output = Self;
  fn div(self, _rhs: T) -> Self::Output {
    Self::Output::new(self.x / _rhs, self.y / _rhs, self.z / _rhs)
  }
}

impl<T: DivAssign + Copy + Clone + Signed + MinMax + Zero> ops::DivAssign<T> for Vector3<T> {
  fn div_assign(&mut self, _rhs: T) {
    self.x /= _rhs;
    self.y /= _rhs;
    self.z /= _rhs;
  }
}

impl ops::Div<Vector3<f32>> for f32 {
  type Output = Vector3<f32>;
  fn div(self, rhs: Vector3<f32>) -> Self::Output {
      Vector3::new(self / rhs.x, self / rhs.y, self / rhs.z)
  }
}


// Element access
impl<T: Zero> ops::Index<usize> for Vector3<T> {
  type Output = T;
  fn index(&self, index: usize) -> &Self::Output {
    assert!(index <= 2);
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,

      // This cannot happen, so return a ref to x
      _ => &self.x
    }
  }
}


// Printing

impl<T: Display> fmt::Display for Vector3<T> {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
  }
}


// Usable Specialized Types

#[allow(dead_code)]
pub type Vector3f = Vector3<RaytracerFloat>;
#[allow(dead_code)]
pub type Vector3i= Vector3<RaytracerInt>;


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