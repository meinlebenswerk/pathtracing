use std::ops;
use std::fmt;

use super::vector3::Vector3;



#[derive(Copy, Clone)]
pub struct Point3 {
  pub x: f32,
  pub y: f32,
  pub z: f32
}

impl Point3 {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
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
    let x = self.x.min(b.x);
    let y = self.y.min(b.y);
    let z = self.z.min(b.z);
    Self::new(x, y, z)
  }

  pub fn max(&self, b: &Self) -> Self {
    let x = self.x.max(b.x);
    let y = self.y.max(b.y);
    let z = self.z.max(b.z);
    Self::new(x, y, z)
  }

  pub fn as_vector3(&self) -> Vector3 {
    Vector3::new(self.x, self.y, self.z)
  }
}

// Point3 Addition
impl ops::Add for Point3 {
  type Output = Point3;
  fn add(self, rhs: Self) -> Self::Output {
      Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl ops::Add<Vector3> for Point3 {
  type Output = Point3;
  fn add(self, rhs: Vector3) -> Self::Output {
      Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}


// Point3 Subtraction
impl ops::Sub for Point3 {
  type Output = Vector3;
  fn sub(self, _rhs: Self) -> Self::Output {
    Self::Output::new(
      self.x - _rhs.x, 
      self.y - _rhs.y, 
      self.z - _rhs.z
    )
  }
}

impl ops::Sub for &Point3 {
  type Output = Vector3;
  fn sub(self, _rhs: Self) -> Self::Output {
    Self::Output::new(
      self.x - _rhs.x, 
      self.y - _rhs.y, 
      self.z - _rhs.z
    )
  }
}

impl ops::Sub<Vector3> for Point3 {
  type Output = Point3;
  fn sub(self, _rhs: Vector3) -> Self::Output {
    Self::Output::new(
      self.x - _rhs.x, 
      self.y - _rhs.y, 
      self.z - _rhs.z
    )
  }
}


// Point3 Multiplication
impl ops::Mul<f32> for Point3 {
  type Output = Point3;
  fn mul(self, rhs: f32) -> Self::Output {
      Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs)
  }
}

impl ops::Mul<Point3> for f32 {
  type Output = Point3;
  fn mul(self, rhs: Point3) -> Self::Output {
    Self::Output::new(self * rhs.x, self * rhs.y, self * rhs.z)
  }
}

impl ops::Mul<&Point3> for f32 {
  type Output = Point3;
  fn mul(self, rhs: &Point3) -> Self::Output {
    Self::Output::new(self * rhs.x, self * rhs.y, self * rhs.z)
  }
}


// Point3 Element access
impl ops::Index<usize> for Point3 {
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


// Point3 Printing
impl fmt::Display for Point3 {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "Point3({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
  }
}

// Point3 Default
impl Default for Point3 {
  fn default() -> Self {
    Self::new(0.0, 0.0, 0.0)
  }
}

pub struct Point2 {
  x: f32,
  y: f32
}

impl Point2 {
  pub fn new(x: f32, y: f32) -> Self {
    Self {
      x,
      y,
    }
  }

  pub fn from_point3(p: &Point3) -> Self {
    Self::new(p.x, p.y)
  }
}