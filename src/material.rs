
use crate::geometry::ray::{Ray, HitRecord};
use crate::geometry::utils::{random_vector, reflect_vector, random_vector_in_unit_sphere, refract_vector, generate_orthonormal_system};
use crate::geometry::vector::Vector3f;
use crate::scene::{ RTXContext };

// What does a material do?
// Given a certain scene, it should give you back a Color.
// Materials should handle depth -> Ideally, no.
// Idk lol

pub trait RTXMaterial {
  fn is_emissive(&self) -> bool;
  fn counts_as_light(&self) -> bool;
  fn scatter(&self, ray: &Ray, next_ray: &mut Ray, attenutation: &mut Vector3f, record: &HitRecord, context: &mut RTXContext) -> bool;

  fn just_scatter(&self, ray: &Ray, next_ray: &mut Ray, attenuation: &mut Vector3f, record: &HitRecord, context: &mut RTXContext) -> bool;
  fn emission_at(&self, ray: &Ray, record: &HitRecord) -> Vector3f;
}


// Helper funtions

fn hemisphere(u1: f32, u2: f32) -> Vector3f {
  let r = f32::sqrt(1.0 - u1*u1);
  let phi = 2.0 * std::f32::consts::PI * u2;
  Vector3f::new(f32::cos(phi)*r, f32::sin(phi)*r, u1)
}


// Diffuse Material

pub struct DiffuseMaterial {
  albedo: Vector3f
}

impl DiffuseMaterial {
  #[allow(dead_code)]
  pub fn new(albedo: Vector3f) -> Self {
    Self {
      albedo: albedo.normalize()
    }
  }
}

impl RTXMaterial for DiffuseMaterial {
  fn scatter(&self, _ray: &Ray, next_ray: &mut Ray, attenutation: &mut Vector3f, record: &HitRecord, context: &mut RTXContext) -> bool {
    // We could influence the scatter direction in the direction of a random light...
    // let light_position = context.scene.get_random_light(context).get_position();
    // let light_direction = light_position -  record.point;

    let mut scatter_direction = record.normal + random_vector(context.rng);  //+ (0.2 * light_direction);
    
    if scatter_direction.near_zero() {
      scatter_direction = record.normal;
    }

    // For diffuse objects, scatter in a hemisphere around the point
    // the normal points outwards
    // random offset just needs to be applied to the plane ontop of the normal
    // let orthonormal_system = record.normal.generate_orthonormal_system();
    // let r1 = (context.rng.next_f32() - 0.5) * 2.0;
    // let r2 = (context.rng.next_f32() - 0.5) * 2.0;

    // let direction = 






    *next_ray = Ray::new(record.point, scatter_direction);
    *attenutation = self.albedo;
    true
  }

  fn just_scatter(&self, _ray: &Ray, next_ray: &mut Ray, attenuation: &mut Vector3f, record: &HitRecord, context: &mut RTXContext) -> bool {
    let sampled_direction = hemisphere(context.rng.next_f32(), context.rng.next_f32());

    // Now comes some black-magic I don't understand
    let ons = generate_orthonormal_system(&record.normal);
    let rdx = Vector3f::new(ons[1].x, ons[2].x, ons[0].x).dot(&sampled_direction);
    let rdy = Vector3f::new(ons[1].y, ons[2].y, ons[0].y).dot(&sampled_direction);
    let rdz = Vector3f::new(ons[1].z, ons[2].z, ons[0].z).dot(&sampled_direction);
    
    *next_ray = Ray::new(record.point, Vector3f::new(rdx, rdy, rdz));
    
    // Diffuse BRDF cos_theta:
    // The factor is weird, though
    *attenuation = self.albedo * next_ray.direction.dot(&record.normal) * 0.1;
    true
  }

  fn is_emissive(&self) -> bool { false }
  fn counts_as_light(&self) -> bool { false }
  fn emission_at(&self, _ray: &Ray, _record: &HitRecord) -> Vector3f { Vector3f::default() }
}


// Metal Material

pub struct MetalMaterial {
  roughness: f32,
  albedo: Vector3f
}

impl MetalMaterial {
  #[allow(dead_code)]
  pub fn new(albedo: Vector3f, roughness: f32) -> Self {
    Self {
      roughness,
      albedo
    }
  }
}

impl RTXMaterial for MetalMaterial {
  fn scatter(&self, ray: &Ray, next_ray: &mut Ray, attenutation: &mut Vector3f, record: &HitRecord, context: &mut RTXContext) -> bool {
    let reflected = reflect_vector(&ray.direction, &record.normal);
    let direction = reflected + self.roughness * random_vector_in_unit_sphere(context.rng);
    *next_ray = Ray::new(record.point, direction);
    *attenutation = self.albedo;
    next_ray.direction.dot(&record.normal) > 0.0
  }

  fn just_scatter(&self, ray: &Ray, next_ray: &mut Ray, attenuation: &mut Vector3f, record: &HitRecord, context: &mut RTXContext) -> bool {
    let sampled_direction = hemisphere(context.rng.next_f32(), context.rng.next_f32());

    // Now comes some black-magic I don't understand
    let ons = generate_orthonormal_system(&record.normal);
    let rdx = Vector3f::new(ons[1].x, ons[2].x, ons[0].x).dot(&sampled_direction);
    let rdy = Vector3f::new(ons[1].y, ons[2].y, ons[0].y).dot(&sampled_direction);
    let rdz = Vector3f::new(ons[1].z, ons[2].z, ons[0].z).dot(&sampled_direction);
    let direction_diffuse = Vector3f::new(rdx, rdy, rdz);
    

    // Metallic scattering
    let cost_metallic = ray.direction.dot(&record.normal);
    let direction_metallic = ray.direction - record.normal*(cost_metallic*2.0);

    // Interpolate between the two
    let direction = self.roughness * direction_diffuse * (1.0 - self.roughness) * direction_metallic.normalize() ;
    *next_ray = Ray::new(record.point, direction);
    
    // Metallic BRDF
    // But this time it's more complicated, since our metallic BRDF can have diffuse properties
    
    let cost_diffuse = next_ray.direction.dot(&record.normal) ;
    let cos_theta = self.roughness * cost_diffuse * (1.0 - self.roughness) * cost_metallic;
    *attenuation = self.albedo * cos_theta;
    true
  }

  fn is_emissive(&self) -> bool { false }
  fn counts_as_light(&self) -> bool { false }
  fn emission_at(&self, _ray: &Ray, _record: &HitRecord) -> Vector3f { Vector3f::default() }
}

// Dielectric Material

pub struct DielectricMaterial {
  ior: f32,
}

impl DielectricMaterial {
  #[allow(dead_code)]
  pub fn new(ior: f32) -> Self {
    Self {
      ior
    }
  }

  fn reflectance(cosine_angle: f32, refractive_index: f32) -> f32 {
    let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    r0 = r0 * r0;
    return r0 + (1.0 - r0)*f32::powf(1.0 - cosine_angle, 5.0)
  }
}

impl RTXMaterial for DielectricMaterial {
  fn scatter(&self, ray: &Ray, next_ray: &mut Ray, attenutation: &mut Vector3f, record: &HitRecord, context: &mut RTXContext) -> bool {
    let refraction_ratio = if record.front_face { 1.0/self.ior } else { self.ior };

    let cos_theta = f32::min((-ray.direction).dot(&record.normal), 1.0);
    let sin_theta = f32::sqrt(1.0 - cos_theta*cos_theta);

    let cannot_refract = (refraction_ratio * sin_theta) > 1.0;

    let direction;
    if cannot_refract || DielectricMaterial::reflectance(cos_theta, refraction_ratio) > context.rng.next_f32() {
      direction = reflect_vector(&ray.direction, &record.normal);
    } else {
      direction = refract_vector(&ray.direction, &record.normal, refraction_ratio);
    }

    // direction = Vector3f::refract(&ray.direction, &record.normal, refraction_ratio);

    *next_ray = Ray::new(record.point, direction);
    *attenutation = Vector3f::new(1.0, 1.0, 1.0);

    true
  }

  fn just_scatter(&self, ray: &Ray, next_ray: &mut Ray, attenuation: &mut Vector3f, record: &HitRecord, context: &mut RTXContext) -> bool {
    let mut normal = record.normal.clone();
    let mut ior = 1.0 / self.ior;

    let mut r_0 = (1.0 - self.ior) / (1.0 + self.ior);
    r_0 += r_0;

    // Check if we're inside the medium?
    if normal.dot(&ray.direction) > 0.0 {
      normal = -normal;
      ior = self.ior;
    }

    let cos_theta1 = -normal.dot(&ray.direction);
    let cos_theta2 = 1.0 - ior*ior*(1.0 - cos_theta1*cos_theta1);
    // Schlick-approximation
    let rprob = r_0 + (1.0 - r_0) * (1.0 - cos_theta1).powf(5.0);

    let direction;
    if cos_theta2 > 0.0 && context.rng.next_f32() > rprob { // refraction direction
      direction = (ray.direction * ior) + (normal * (ior * cos_theta1 - cos_theta2.sqrt()));
		}
		else { // reflection direction
      direction = ray.direction + normal*(cos_theta1 * 2.0);
		}

    *next_ray = Ray::new(record.point, direction);

    // Why? who the fuck knows
    *attenuation = Vector3f::new(1.15, 1.15, 1.15);

    true
  }

  fn is_emissive(&self) -> bool { false }
  fn counts_as_light(&self) -> bool { false }
  fn emission_at(&self, _ray: &Ray, _record: &HitRecord) -> Vector3f { Vector3f::default() }
}

// Dielectric Material

pub struct NormalMaterial {}

impl NormalMaterial {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self {}
  }
}

impl RTXMaterial for NormalMaterial {
  fn scatter(&self, _ray: &Ray, _next_ray: &mut Ray, attenutation: &mut Vector3f, record: &HitRecord, _context: &mut RTXContext) -> bool {
    // let normal = if record.front_face { record.normal } else { -record.normal };
    let normal = record.normal;
    *attenutation = (normal + 1.0) / 2.0;
    // *attenutation = record.normal;
    // *attenutation = Vector3f::new(1.0, 1.0, 1.0);
    false
  }

  fn just_scatter(&self, _ray: &Ray, next_ray: &mut Ray, attenuation: &mut Vector3f, record: &HitRecord, context: &mut RTXContext) -> bool {
    // This behaves just like a diffuse material :)
    let sampled_direction = hemisphere(context.rng.next_f32(), context.rng.next_f32());

    // Now comes some black-magic I don't understand
    let ons = generate_orthonormal_system(&record.normal);
    let rdx = Vector3f::new(ons[1].x, ons[2].x, ons[0].x).dot(&sampled_direction);
    let rdy = Vector3f::new(ons[1].y, ons[2].y, ons[0].y).dot(&sampled_direction);
    let rdz = Vector3f::new(ons[1].z, ons[2].z, ons[0].z).dot(&sampled_direction);
    
    *next_ray = Ray::new(record.point, Vector3f::new(rdx, rdy, rdz));
    
    // Diffuse BRDF cos_theta:
    // The factor is weird, though
    let albedo = Vector3f::new(1.0, 1.0, 1.0);
    *attenuation = albedo * next_ray.direction.dot(&record.normal) * 0.1;

    false
  }

  fn is_emissive(&self) -> bool { true }
  fn counts_as_light(&self) -> bool { true }
  fn emission_at(&self, _ray: &Ray, record: &HitRecord) -> Vector3f { 
    (record.normal + 1.0) / 2.0
  }
}


// A proper diffuse material
// light which hits surface -> the smaller the angle between normal and ray is, the higher the chance it'll reflect back
// RayIntensity = LightIntensity + kd * cos(theta)
// -> the attenuation = RayIntensity / LightIntensity
// -> attenuation = kd * cos(theta) 

// Emissive Material
pub struct EmissiveMaterial {
  albedo: Vector3f,
  intensity: f32
}

impl EmissiveMaterial {
  #[allow(dead_code)]
  pub fn new(albedo: Vector3f, intensity: f32) -> Self {
    Self {
      albedo,
      intensity
    }
  }
}

impl RTXMaterial for EmissiveMaterial {
  fn scatter(&self, _ray: &Ray, _next_ray: &mut Ray, attenutation: &mut Vector3f, _record: &HitRecord, _context: &mut RTXContext) -> bool {
    *attenutation = self.albedo * self.intensity;
    false
  }

  // Also just use the diffuse scatter
  fn just_scatter(&self, _ray: &Ray, next_ray: &mut Ray, attenuation: &mut Vector3f, record: &HitRecord, context: &mut RTXContext) -> bool {
    let sampled_direction = hemisphere(context.rng.next_f32(), context.rng.next_f32());

    // Now comes some black-magic I don't understand
    let ons = generate_orthonormal_system(&record.normal);
    let rdx = Vector3f::new(ons[1].x, ons[2].x, ons[0].x).dot(&sampled_direction);
    let rdy = Vector3f::new(ons[1].y, ons[2].y, ons[0].y).dot(&sampled_direction);
    let rdz = Vector3f::new(ons[1].z, ons[2].z, ons[0].z).dot(&sampled_direction);
    
    *next_ray = Ray::new(record.point, Vector3f::new(rdx, rdy, rdz));
    
    // Diffuse BRDF cos_theta:
    // The factor is weird, though
    *attenuation = self.albedo * next_ray.direction.dot(&record.normal) * 0.1;
    true
  }
  

  fn is_emissive(&self) -> bool { true }
  fn counts_as_light(&self) -> bool { true }
  fn emission_at(&self, _ray: &Ray, _record: &HitRecord) -> Vector3f { 
    self.albedo * self.intensity
  }
}


// World Material
pub struct WorldMaterial {
  albedo_top: Vector3f,
  albedo_bottom: Vector3f,
}

impl WorldMaterial {
  #[allow(dead_code)]
  pub fn new(albedo_top: Vector3f, albedo_bottom: Vector3f) -> Self {
    Self {
      albedo_top,
      albedo_bottom
    }
  }
}

impl RTXMaterial for WorldMaterial {
  fn scatter(&self, ray: &Ray, _next_ray: &mut Ray, attenutation: &mut Vector3f, _record: &HitRecord, _context: &mut RTXContext) -> bool {
    let t = 0.5 * (ray.direction.y + 1.0 );
    *attenutation = (1.0 - t) * self.albedo_bottom + t * self.albedo_top;
    false
  }

  fn just_scatter(&self, _ray: &Ray, _next_ray: &mut Ray, _attenuation: &mut Vector3f, _record: &HitRecord, _context: &mut RTXContext) -> bool {
    false
  }

  fn is_emissive(&self) -> bool { true }
  fn counts_as_light(&self) -> bool { false }
  fn emission_at(&self, ray: &Ray, _record: &HitRecord) -> Vector3f { 
    let t = 0.5 * (ray.direction.y + 1.0 );
    (1.0 - t) * self.albedo_bottom + t * self.albedo_top
  }
}

