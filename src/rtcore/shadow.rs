use crate::geometry::point::Point3f;
use crate::geometry::ray::{Ray, HitRecord};
use crate::geometry::vector3::Vector3f;
use crate::prng::PRNG;
use crate::scene::RTXContext;

#[allow(dead_code)]
pub fn trace(ray: &Ray, depth: usize, context: &mut RTXContext, color: &mut Vector3f, rng: &mut dyn PRNG) {
    // Add roussian roulette path termination
    // Which includes a scalar, which scales ray inclusion
    // let rr_startpoint = max_depth/2;

    let mut rr_factor = 1.0;
    // Start terminating rays after 10 bounces
    if depth > 10 {
      // this should ramp from 0 - 1;
      // let rr_stop_probability = (depth - rr_startpoint) as f32 / max_depth as f32;
      let rr_stop_probability = 0.1;
      if rng.next_f32() <= rr_stop_probability {
        return;
      }
      rr_factor = 1.0 / (1.0 - rr_stop_probability);
    }

    // No more hard limits.
    // if depth == max_depth { return; }

    let mut record = HitRecord::new();

    // Check for intersection
    let hit = context.scene.intersect(ray, 0.001, 100000.0, &mut record);
    if !hit { return };
    
    let mut next_ray: Ray = Ray::new(Point3f::default(), Vector3f::new(0.0, 0.0, 1.0));
    let mut attenuation = Vector3f::default();
    let material = record.material.as_ref().unwrap();

    // Cast a new ray for each lightsource in the scene, now

    // process the first term of the rendering equation (self-emittance)
    *color += material.emission_at(ray, &record) * rr_factor;

    if material.just_scatter(ray, &mut next_ray, &mut attenuation, &record, context, rng) {
      // Process the next ray :)
      let mut tmp_color = Vector3f::default();
      trace(&next_ray, depth + 1, context, &mut tmp_color, rng);
      *color += tmp_color * attenuation * rr_factor;
    }
}