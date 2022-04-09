use crate::{
    vec3::Vec3,
    hitable::HitRecord,
    ray::Ray,
    material::Material,
    dialectric::reflect,
};

#[derive(Clone, Default)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
        *scattered = Ray{a: rec.p, b: reflected + Vec3::random_in_unit_sphere() * self.fuzz};
        *attenuation = self.albedo;
        return scattered.direction().dot(rec.normal) > 0.0;
    }
}
