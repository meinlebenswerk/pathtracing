use crate::{geometry::{point3::{Point3, Point3f}, ray::{Ray, HitRecord}, vector3::{Vector3f, Normal3f}, point2::Point2f}, config::RaytracerFloat};


#[derive(Copy, Clone)]
pub struct Bounds3<T>
where T: Copy {
  min: Point3<T>,
  max: Point3<T>
}

type Bounds3f = Bounds3<RaytracerFloat>;


pub struct SurfaceInteraction {
    uv: Point2f,
    dpdu: Point3f,
    dpdv: Point3f,
    dndu: Normal3f,
    dndv: Point3f,
    // shape-ref,
    // shading-struct
    // primitive-ref
    // BDSF
    // BSSRDF
    dpdx: Vector3f,
    dpdy: Vector3f,
    dudx: RaytracerFloat,
    dvdx: RaytracerFloat,
    dudy: RaytracerFloat,
    dvdy: RaytracerFloat,
}

pub trait RTXPrimitive {

    fn get_world_bounds(&self) -> Bounds3f;
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    // virtual Bounds3f WorldBound() const = 0;
    // virtual bool Intersect(const Ray &r, SurfaceInteraction *) const = 0;
    // virtual bool IntersectP(const Ray &r) const = 0;
    // virtual const AreaLight *GetAreaLight() const = 0;
    // virtual const Material *GetMaterial() const = 0;
    // virtual void ComputeScatteringFunctions(SurfaceInteraction *isect,
    //     MemoryArena &arena, TransportMode mode,
    //     bool allowMultipleLobes) const = 0;
}