use crate::prng::PRNG;

use super::{vector3::Vector3, point::Point3};

// Vector utils

pub fn random_vector(prng: &mut dyn PRNG) -> Vector3 {
  let a: f32 = (prng.next_f32() - 0.5) * 2.0;
  let b: f32 = (prng.next_f32() - 0.5) * 2.0;
  let c: f32 = (prng.next_f32() - 0.5) * 2.0;
  Vector3::new(a, b, c)
}

pub fn random_vector_ranged(prng: &mut dyn PRNG, min: f32, max: f32) -> Vector3 {
  let diff = max - min;
  let a: f32 = prng.next_f32()*diff + min;
  let b: f32 = prng.next_f32()*diff + min;
  let c: f32 = prng.next_f32()*diff + min;
  Vector3::new(a, b, c)
}

pub fn random_vector_in_sphere(prng: &mut dyn PRNG, position: Vector3, radius: f32) -> Vector3 {
  let phi = prng.next_f32() * 2.0 * std::f32::consts::PI;
  let theta = prng.next_f32() * 2.0 * std::f32::consts::PI;
  let r = prng.next_f32() * radius;

  let x = r * f32::cos(phi) * f32::sin(theta);
  let y = r * f32::sin(phi) * f32::sin(theta);
  let z = r * f32::cos(theta);

  position + Vector3::new(x, y, z)
}

pub fn random_vector_on_sphere(prng: &mut dyn PRNG, position: Point3, radius: f32) -> Point3 {
  let phi = prng.next_f32() * 2.0 * std::f32::consts::PI;
  let theta = prng.next_f32() * 2.0 * std::f32::consts::PI;
  
  let x = radius * f32::cos(phi) * f32::sin(theta);
  let y = radius * f32::sin(phi) * f32::sin(theta);
  let z = radius * f32::cos(theta);

  position + Vector3::new(x, y, z)
}


pub fn random_vector_in_unit_sphere(prng: &mut dyn PRNG) -> Vector3 {
  let phi = prng.next_f32() * 2.0 * std::f32::consts::PI;
  let theta = prng.next_f32() * 2.0 * std::f32::consts::PI;

  let x = f32::cos(phi) * f32::sin(theta);
  let y = f32::sin(phi) * f32::sin(theta);
  let z = f32::cos(theta);
  
  Vector3::new(x, y, z)
}


//  TODO: Move them somewhere else.
pub fn reflect_vector(v: &Vector3, n: &Vector3) -> Vector3 {
  *v - 2.0 * v.dot(n) * *n
}

pub fn refract_vector(incident: &Vector3, normal: &Vector3, refraction_ratio: f32) -> Vector3 {
  // ior = c / v
  // Snell: n2 * sin(phi2) = n1 * sin(phi1)
  // where phi2 and n2 are the angle/ior of the material we refract into

  let cos_theta_i = incident.dot(normal);
  let sin2_theta_t = (refraction_ratio * refraction_ratio) * (1.0 - cos_theta_i*cos_theta_i);

  // for refraction_ratio > 1 -> then total internal reflection, handled outside this function
  let t_incident = refraction_ratio * (*incident);
  let t_normal = (refraction_ratio * cos_theta_i - f32::sqrt(1.0 - sin2_theta_t)) * (*normal);
  t_incident + t_normal
}


// Stolen from smallpaint https://github.com/8BitRick/smallpaint/blob/master/with_bvh/smallpaint.cpp
pub fn generate_orthonormal_system(v: &Vector3) -> [Vector3; 3] {
  let v2;
  let inverse_length;
  if f32::abs(v.x) > f32::abs(v.y) {
    // project onto the y = 0 plane and construct normalized orthogonal vector there
    inverse_length = 1.0 / (v.x * v.x + v.z * v.z).sqrt();
    v2 = Vector3::new(-v.z * inverse_length, 0.0, v.x * inverse_length);
  } else {
    // project onto the x = 0 plane and construct normalized orthogonal vector there
    inverse_length = 1.0 / (v.y * v.y + v.z * v.z).sqrt();
    v2 = Vector3::new(0.0, v.z * inverse_length, -v.y * inverse_length);
  }
  let v3 = v.cross(&v2);
  [v.clone(), v2, v3]
}

// Point Utils

pub fn distance(p1: &Point3, p2: &Point3) -> f32 {
  (p1 - p2).length()
}

pub fn distance_squared(p1: &Point3, p2: &Point3) -> f32 {
  (p1 - p2).length_squared()
}

pub fn lerp(t:f32, a: &Point3, b: &Point3) -> Point3 {
  (1.0 - t) * a + t * b
}