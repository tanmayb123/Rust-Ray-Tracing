use crate::{
    vec3::Vec3,
    hitable::HitRecord,
    ray::Ray,
    material::Material,
};

#[derive(Clone, Default)]
pub struct Lambertian {
	pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
	fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
		let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
		*scattered = Ray{a: rec.p, b: target - rec.p};
		*attenuation = self.albedo;
		true
	}
}
