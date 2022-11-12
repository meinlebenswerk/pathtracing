use crate::{geometry::{ vector3::Vector3f, ray::Ray, point3::Point3f }, film::Film, config::RaytracerFloat};


pub struct Camera {
  position: Point3f,
  pub w: Vector3f,
  pub u: Vector3f,
  pub v: Vector3f,
  film: Film
}

impl Camera {
  pub fn new(position: Point3f, look_at: Point3f, v_up: Vector3f, vfov: f32, aspect_ratio: f32, film: Film) -> Self {

    let theta = vfov/180.0 * std::f32::consts::PI;
    let height = f32::tan(theta/2.0);
    // let width = aspect_ratio * height;

    // camera "normal" vector, in view direction
    let w = (look_at - position).normalize();

    // horizontal base vector for the image plane
    let u = v_up.cross(&w).normalize();

    // vertical base-vector for the view-direction
    let v = w.cross(&u);

    Self {
      position,
      w,
      u,
      v,
      film
    }
  }

  pub fn generate_ray(&self, u: f32, v: f32) -> Ray {
    // u is the x-dir, y is the y-dir (in camera-coordinates)
    // We'll need to scale the directions by the aspect-ratio
    // aspect-ratio = width / height

    // This does not take fov into account, yet.

    // scale the local-coords (0-1) into (-1, 1), but also take the aspect-ratio into account.
    let x = (u - 0.5) * 2.0;
    let y = (v - 0.5) * 2.0;

    let direction = (self.u * x * (self.film.resolution.x as RaytracerFloat)) + (self.v * y * (self.film.resolution.y as RaytracerFloat)) + self.w;
    Ray::new(self.position, direction)
  }
}