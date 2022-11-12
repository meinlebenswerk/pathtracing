use std::sync::Arc;

use crate::{scene::Scene, sampler::RTXSampler, camera::Camera, threading::RTXThreadMemoryArena, primitive::SurfaceInteraction};

pub mod sampler_integrator;

pub trait RTXIntegrator {
  fn render(&self, scene: &Scene);
}

pub struct RayDifferential {

}

pub trait SamplerIntegrator<Sampler, Spectrum>
where Sampler: RTXSampler {
  fn new(camera: Arc<Camera>, sampler: Arc<Sampler>) -> Self;
  fn preprocess(&self, scene: &Scene);
  fn render(&self, scene: &Scene);
  fn li(ray: &RayDifferential, scene: &Scene, arena: &mut RTXThreadMemoryArena, depth: usize) -> Spectrum;
  fn specular_reflect(ray: &RayDifferential, iset: &mut SurfaceInteraction, scene: &Scene, arena: &mut RTXThreadMemoryArena, depth: usize) -> Spectrum;
  fn specular_transmit(ray: &RayDifferential, iset: &mut SurfaceInteraction, scene: &Scene, arena: &mut RTXThreadMemoryArena, depth: usize) -> Spectrum;
}