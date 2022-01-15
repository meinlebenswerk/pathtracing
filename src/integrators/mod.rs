// TODO move the shit from rtcore here, and modify the function signatures


pub trait Integrator {
  fn render(&self, scene: &Scene);
  fn Li(ray: &Ray, scene: &Scene, depth: usize);
  // These methods produce colors, not points/vectors.
  fn SpecularReflect(ray: &Ray, record: &HitRecord, depth: usize) -> Vector3;
  fn SpecularTransmit(ray: &Ray, record: &HitRecord, depth: usize) -> Vector3;
}