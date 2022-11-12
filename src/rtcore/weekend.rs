use crate::geometry::point::Point3f;
use crate::geometry::ray::{Ray, HitRecord};
use crate::geometry::vector::Vector3f;
use crate::prng::PRNG;
use crate::scene::RTXContext;

pub fn ray_color_weekend(ray: &Ray, depth: usize, context: &RTXContext, rng: &mut dyn PRNG) -> Vector3f {
    // Add roussian roulette path termination
    // Which includes a scalar, which scales ray inclusion

    if depth == 0 {
        // return Vector3f::default();
        return Vector3f::new(1.0, 1.0, 1.0);
    }

    let mut record = HitRecord::new();
    let hit = context.scene.intersect(ray, 0.001, 100000.0, &mut record);
    if hit {
        // println!("Ray Hit something");
        let mut next_ray: Ray = Ray::new(Point3f::default(), Vector3f::new(0.0, 0.0, 1.0));
        let mut attenuation: Vector3f = Vector3f::default();

        if let Some(material) = &record.material {
            if material.scatter(ray, &mut next_ray, &mut attenuation, &record, context, rng) {
                return attenuation * ray_color_weekend(&next_ray, depth - 1, context, rng)
            }

            if material.is_emissive() {
                return attenuation
            }
        }

        return Vector3f::default();
        // let target = Vector3f::random_in_sphere(context.rng, record.point + record.normal, 1.0).normalize();
        // let next_ray = Ray::new(record.point, target - record.point);
        // return 0.5 * ray_color(&next_ray, depth - 1, context);
        // return 0.5 * (record.normal + Vector3f::new(1.0, 1.0, 1.0));
    }

    // This is the world-intersection, we should use a word-sphere for that.
    // let t = 0.5 * (ray.direction.y() + 1.0 );
    // (1.0 - t) * Vector3f::new(1.0, 1.0, 1.0) + t * Vector3f::new(0.5, 0.7, 1.0)
    Vector3f::default()
}