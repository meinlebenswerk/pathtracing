use std::ops;
use std::fmt;

use super::point::Point3;

#[derive(Clone, Copy)]
pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vector3 {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self {
      x,
      y,
      z,
    }
  }

  pub fn from_hex(color: &str) -> Self {
    // only accept #rbg or #rrbbgg strings
    let len = color.len();
    assert!( len == 4 || len == 7 );
    let bytes_per_field = if len == 4 { 1 } else { 2 };
    let max_value = if bytes_per_field == 1 { 0xf as f32 } else { 0xff as f32 };
    let max_value_inv = 1.0 / max_value;
    let r = i32::from_str_radix(&color[1..(1 + bytes_per_field )], 16).unwrap_or(0) as f32;
    let g = i32::from_str_radix(&color[(1 + bytes_per_field)..(1 + 2*bytes_per_field )], 16).unwrap_or(0) as f32;
    let b = i32::from_str_radix(&color[(1 + 2*bytes_per_field)..(1 + 3*bytes_per_field )], 16).unwrap_or(0) as f32;
    Self::new(r * max_value_inv, g * max_value_inv, b * max_value_inv)
  }

  pub fn length_squared(&self) -> f32 {
    (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
  }

  pub fn length(&self) -> f32 {
    self.length_squared().sqrt()
  }

  pub fn normalize(&self) -> Self {
    let inv_length = 1.0 / self.length();
    Self::new(
      self.x * inv_length,
      self.y * inv_length,
      self.z * inv_length
    )
  }

  pub fn dot(&self, vec: &Vector3) -> f32 {
    (self.x * vec.x) + (self.y * vec.y) + (self.z * vec.z)
  }

  pub fn cross(&self, vec: &Vector3) -> Self {
    let x = self.y * vec.z - self.z * vec.y;
    let y = self.z * vec.x - self.x * vec.z;
    let z = self.x * vec.y - self.y * vec.x;
    Self::new(x, y, z)
  }

  pub fn clamp(&mut self, min: f32, max: f32) {
    self.x = f32::max(min, f32::min(self.x, max));
    self.y = f32::max(min, f32::min(self.y, max));
    self.z = f32::max(min, f32::min(self.z, max));
  }

  pub fn near_zero(&self) -> bool {
    self.length_squared() < std::f32::EPSILON
  }

  pub fn abs(&self) -> Self {
    Self::new(
      self.x.abs(), 
      self.y.abs(), 
      self.z.abs()
    )
  }

  pub fn min_elementwise(a: &Vector3, b: &Vector3) -> Self {
    let x = a.x.min(b.x);
    let y = a.y.min(b.y);
    let z = a.z.min(b.z);
    Self::new(x, y, z)
  }

  pub fn max_elementwise(a: &Vector3, b: &Vector3) -> Self {
    let x = a.x.max(b.x);
    let y = a.y.max(b.y);
    let z = a.z.max(b.z);
    Self::new(x, y, z)
  }

  pub fn decompose(&self) -> [Vector3; 3] {
    [
      Vector3::new(self.x, 0.0, 0.0),
      Vector3::new(0.0, self.y, 0.0),
      Vector3::new(0.0, 0.0, self.z),
    ]
  }

  pub fn as_point3(&self) -> Point3 {
    Point3::new(self.x, self.y, self.z)
  }

  pub fn column(&self) -> Vec<&f32> {
    vec![&self.x, &self.y, &self.z]
  }

  pub fn row(&self, y: usize) -> Vec<&f32> {
    assert!(y < 3);
    match y {
      0 => vec![&self.x],
      1 => vec![&self.y],
      2 => vec![&self.z],
      _ => vec![]
    }
  }
}

impl Default for Vector3 {
  fn default() -> Self {
    Self::new(0.0, 0.0, 0.0)
  }
}

// Operators

// Addition
impl ops::Add for Vector3 {
  type Output = Self;
  fn add(self, _rhs: Self) -> Self::Output {
    Self::new(self.x + _rhs.x, self.y + _rhs.y, self.z + _rhs.z)
  }
}

impl ops::Add<f32> for Vector3 {
  type Output = Self;
  fn add(self, _rhs: f32) -> Self::Output {
    Self::new(self.x + _rhs, self.y + _rhs, self.z + _rhs)
  }
}

impl ops::AddAssign for Vector3 {
  fn add_assign(&mut self, _rhs: Self) {
    self.x += _rhs.x;
    self.y += _rhs.y;
    self.z += _rhs.z;
  }
}

// Subtraction
impl ops::Sub for Vector3 {
  type Output = Self;
  fn sub(self, _rhs: Self) -> Self::Output {
    Self::new(self.x - _rhs.x, self.y - _rhs.y, self.z - _rhs.z)
  }
}

impl ops::Sub<Vector3> for f32 {
  type Output = Vector3;
  fn sub(self, _rhs: Vector3) -> Self::Output {
    Self::Output::new(self - _rhs.x, self- _rhs.y, self - _rhs.z)
  }
}

impl ops::SubAssign for Vector3 {
  fn sub_assign(&mut self, _rhs: Self) {
    self.x -= _rhs.x;
    self.y -= _rhs.y;
    self.z -= _rhs.z;
  }
}

// Negation
impl ops::Neg for Vector3 {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Self::new(-self.x, -self.y, -self.z)
  }
}

// Multiplication
impl ops::Mul<f32> for Vector3 {
  type Output = Self;
  fn mul(self, _rhs: f32) -> Self::Output {
    Self::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
  }
}

impl ops::Mul<f32> for &Vector3 {
  type Output = Vector3;
  fn mul(self, _rhs: f32) -> Self::Output {
    Self::Output::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
  }
}

impl ops::Mul<Vector3> for f32 {
  type Output = Vector3;
  fn mul(self, _rhs: Vector3) -> Self::Output {
    Self::Output::new(_rhs.x * self, _rhs.y * self, _rhs.z * self )
  }
}

impl ops::Mul<Vector3> for Vector3 {
  type Output = Vector3;
  fn mul(self, _rhs: Vector3) -> Self::Output {
    Self::Output::new(_rhs.x * self.x, _rhs.y *  self.y, _rhs.z *  self.z)
  }
}

impl ops::Mul<&Vector3> for Vector3 {
  type Output = Vector3;
  fn mul(self, _rhs: &Vector3) -> Self::Output {
    Self::Output::new(_rhs.x * self.x, _rhs.y *  self.y, _rhs.z *  self.z)
  }
}

impl ops::MulAssign<f32> for Vector3 {
  fn mul_assign(&mut self, _rhs: f32) {
    self.x *= _rhs;
    self.y *= _rhs;
    self.z *= _rhs;
  }
}

// Division
impl ops::Div<f32> for Vector3 {
  type Output = Self;
  fn div(self, _rhs: f32) -> Self::Output {
    Self::Output::new(self.x / _rhs, self.y / _rhs, self.z / _rhs)
  }
}

impl ops::DivAssign<f32> for Vector3 {
  fn div_assign(&mut self, _rhs: f32) {
    self.x /= _rhs;
    self.y /= _rhs;
    self.z /= _rhs;
  }
}

impl ops::Div<Vector3> for f32 {
  type Output = Vector3;
  fn div(self, rhs: Vector3) -> Self::Output {
      Vector3::new(self / rhs.x, self / rhs.y, self / rhs.z)
  }
}


// Element access
impl ops::Index<usize> for Vector3 {
  type Output = f32;
  fn index(&self, index: usize) -> &Self::Output {
    assert!(index <= 2);
    match index {
      0 => &self.x,
      1 => &self.y,
      2 => &self.z,
      _ => &0.0
    }
  }
}


// Printing

impl fmt::Display for Vector3 {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
  }
}
