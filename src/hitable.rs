use crate::{
    vec3::Vec3,
    material::MaterialType,
    ray::Ray,
    lambertian::Lambertian,
};

#[derive(Clone)]
pub struct HitRecord {
	pub t: f32,
	pub p: Vec3,
	pub normal: Vec3,
	pub mat: MaterialType,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3 { e0: 0.0, e1: 0.0, e2: 0.0 },
            normal: Vec3 { e0: 0.0, e1: 0.0, e2: 0.0 },
            mat: MaterialType::LambertianMat(Lambertian::default()),
        }
    }
}

pub trait Hitable: Clone {
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
