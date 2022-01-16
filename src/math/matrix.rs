use std::{ops, fmt};

use float_cmp::approx_eq;

use crate::geometry::vector::Vector3f;


#[derive(Clone)]
pub struct Matrix {
  data: Vec<f32>,
  w: usize,
  h: usize
}

impl Matrix {
  pub fn zeros(h: usize, w: usize) -> Self {
    Self {
      data: vec![0.0; w*h],
      w,
      h,
    }
  }

  pub fn identity(h: usize, w: usize) -> Self {
    let mut data = vec![0.0; w*h];
    for y in 0..h {
      for x in 0..w {
        if x == y {
          let index = x + (y * w);
          data[index] = 1.0;
        }
      }
    }
    Self {
      data,
      w,
      h
    }
  }

  pub fn new(data: Vec<f32>, h: usize, w: usize) -> Self {
    assert!(w*h == data.len());
    Self {
      data,
      w,
      h
    }
  }

  pub fn is_identity(&self) -> bool {
    for y in 0..self.h {
      for x in 0..self.w {
        let index = x + (y * self.w);
        let expected_value = if x == y { 1.0 } else { 0.0 };
        if !approx_eq!(f32, self.data[index], expected_value) {
          return false;
        }
      }
    }
    true
  }

  pub fn is_zero(&self) -> bool {
    let sum = self.data.iter().fold(0.0, |acc, e| acc + e.abs());
    sum < f32::EPSILON
  }

  pub fn row<'ls>(&'ls self, y: usize) -> Vec<&f32> {
    assert!(y < self.h);
    (0..self.w).map(|x| {
      let index = x + (y * self.w);
      &self.data[index]
    }).collect()
  }

  pub fn column<'ls>(&'ls self, x: usize) -> Vec<&f32> {
    assert!(x < self.w);
    (0..self.h).map(|y| {
      let index = x + (y * self.w);
      &self.data[index]
    }).collect()
  }

  pub fn submatrix(&self, n: usize) -> Self {
    let indices_x: Vec<usize> = (0..self.w).filter(|x| (*x) != n).collect();
    let indices_y: Vec<usize> = (0..self.h).filter(|y| (*y) != n).collect();

    let data = indices_y.iter().flat_map(|y| {
      indices_x.iter().map(|x| (*x) + (*y * self.w))
    })
    .map(|index| self.data[index])
    .collect();

    Self::new(data, self.h - 1, self.w - 1)
  }

  pub fn determinant(&self) -> f32 {
    assert!(self.h == self.w);
    match self.h {
      0 => 0.0,
      1 => self.data[0],
      2 => self.data[0]*self.data[3] - self.data[1]*self.data[2],
      3 => {
          self.data[0] * (self.data[4]*self.data[8] - self.data[5]*self.data[7])
        - self.data[1] * (self.data[3]*self.data[8] - self.data[5]*self.data[6])
        + self.data[2] * (self.data[3]*self.data[7] - self.data[4]*self.data[6]) 
      },
      _ => {
        let mut det = 0.0;
        for n in 0..self.w {
          let sign = if (n & 0x01) == 0 { 1.0 } else { -1.0 };

          // Skip emtpy values
          if self.data[n].abs() <= 2.0 * f32::EPSILON { continue };
          let submatrix = self.submatrix(n);
          // println!("Submat n={}, {}, sign={}", n, submatrix, sign);
          det += sign * self.data[n] * submatrix.determinant();
        }
        det
      }
    }
  }
}



// Matrix Operations
impl ops::Add for Matrix {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
      assert!(self.w == rhs.w && self.h == rhs.h);
      let data = self.data.iter().zip(rhs.data.iter()).map(|(a, b)| a + b).collect();
      Self::new(data, self.w, self.h)
  }
}

impl ops::Sub for Matrix {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
      assert!(self.w == rhs.w && self.h == rhs.h);
      let data = self.data.iter().zip(rhs.data.iter()).map(|(a, b)| a - b).collect();
      Self::new(data, self.w, self.h)
  }
}

fn mul_elementwise(a: &Vec<&f32>, b: &Vec<&f32>) -> f32 {
  assert!(a.len() == b.len());
  a.iter().enumerate().fold(0.0, |acc, (index, val)| acc + *val * b[index])
}

impl ops::Mul for Matrix {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
      assert!(self.w == rhs.h);
      let mut data = vec![0.0; self.h * rhs.w];

      for y in 0..self.h {
        for x in 0..rhs.w {
          let index = x + (y * rhs.w);
          data[index] = mul_elementwise(&self.row(y), &rhs.column(x));
        }
      }

      Self::Output::new(data, rhs.w, self.h)
  }
}

impl ops::Mul<Vector3f> for Matrix {
  type Output = Self;
  fn mul(self, rhs: Vector3f) -> Self::Output {
      assert!(self.w == 3);
      let mut data = vec![0.0; self.h];

      for y in 0..self.h {
        data[y] = mul_elementwise(&self.row(y), &rhs.column());
      }

      Self::Output::new(data, self.h, 1)
  }
}

// Matrix printing
impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Matrix{}x{}\n", self.h, self.w)?;
    for y in 0..self.h {
      for x in 0..self.w {
        let index = x + (y * self.w);
        write!(f, "{:.2}\t", self.data[index])?;
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}


// Vector <-> mat conversion:
impl From<Vector3f> for Matrix {
  fn from(vec: Vector3f) -> Self {
      Self::new(
        vec![vec.x, vec.y, vec.z],
        3,
        1
      )
  }
}

impl From<Matrix> for Vector3f {
  fn from(mat: Matrix) -> Self {
    assert!(mat.h == 1 && mat.w == 1);
    Vector3f::new(
      mat.data[0],
      mat.data[1],
      mat.data[2]
    )
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_mat_identity() {
    let mat1 = Matrix::identity(2, 2);
    let mat2 = Matrix::identity(3, 3);
    let mat3 = Matrix::identity(4, 4);
    let mat4 = Matrix::identity(2, 8);
    let mat5 = Matrix::zeros(16, 16);
    assert!(mat1.is_identity());
    assert!(mat2.is_identity());
    assert!(mat3.is_identity());
    assert!(mat4.is_identity());
    assert!(!mat5.is_identity());
  }

  #[test]
  fn test_mat_zero() {
    let mat1 = Matrix::zeros(2, 2);
    let mat2 = Matrix::zeros(3, 3);
    let mat3 = Matrix::zeros(4, 4);
    let mat4 = Matrix::zeros(2, 8);
    let mat5 = Matrix::identity(16, 16);
    assert!(mat1.is_zero());
    assert!(mat2.is_zero());
    assert!(mat3.is_zero());
    assert!(mat4.is_zero());
    assert!(!mat5.is_zero());
  }

  #[test]
  fn test_mat_multiply() {
    let mat1 = Matrix::identity(8, 16);
    let mat2 = Matrix::identity(16, 8);
    let output = mat1 * mat2;
    assert!(output.is_identity());

    let mat3 = Matrix::new(
      vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
      2, 3
    );
    let mat4 = Matrix::new(
      vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
      3, 2
    );
    let output2 = mat3 * mat4;
    let m = Matrix::new(
      vec![21.0, 28.0, 49.0, 63.0],
      2, 2
    );

    assert!((output2 - m).is_identity());
  }

  #[test]
  fn test_mat_vec_multiply() {
    let mat1 = Matrix::identity(5, 3);
    let vec1 = Vector3f::new(2.0, 3.0, 6.0);
    let offset = Matrix::new(
      vec![1.0, 3.0, 6.0, 0.0, 0.0],
    5, 1);
    let output = mat1 * vec1;
    assert!((output- offset).is_identity());
  }

  #[test]
  fn test_mat_determinant() {
    let mat1 = Matrix::identity(1, 1);
    let mat2 = Matrix::identity(2, 2);
    let mat3 = Matrix::identity(3, 3);
    let mat4 = Matrix::identity(16, 16);
    let mat5 = Matrix::zeros(16, 16);
    assert!(approx_eq!(f32, mat1.determinant(), 1.0));
    assert!(approx_eq!(f32, mat2.determinant(), 1.0));
    assert!(approx_eq!(f32, mat3.determinant(), 1.0));
    assert!(approx_eq!(f32, mat4.determinant(), 1.0));
    assert!(approx_eq!(f32, mat5.determinant(), 0.0));

    let mat6 = Matrix::new(
      vec![
          6.0, 1.0, 1.0,
          4.0, -2.0, 5.0,
          2.0, 8.0, 7.0
         ],
      3, 3 
    );
    assert!(approx_eq!(f32, mat6.determinant(), -306.0));

    let mat7 = Matrix::new(
      vec![
        1.0, 2.0, 3.0, 4.0, 
        9.0, 22.0, 92.0, 1.0, 
        11.0, 34.0, 23.0, 67.0, 
        7.0, 0.0, 0.0, 0.0],
      4, 4
    );
    assert!(approx_eq!(f32, mat7.determinant(), 17682.0));

  }
}