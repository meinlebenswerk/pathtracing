use crate::{objects::triangle::Triangle, geometry::vector3::Vector3};


pub fn triangulate_square(points: &[Vector3]) -> Vec<Triangle> {
  assert!(points.len() == 4);
  let tris = vec![
    Triangle::new(points[0], points[1], points[2]),
    Triangle::new(points[0], points[2], points[3])
  ];
  tris
}