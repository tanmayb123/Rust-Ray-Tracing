use crate::{
    vec3::Vec3,
    hitable::HitRecord,
    ray::Ray,
    dialectric::Dialectric,
    lambertian::Lambertian,
    metal::Metal,
};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

#[derive(Clone)]
pub enum MaterialType {
    LambertianMat(Lambertian),
    DialectricMat(Dialectric),
    MetalMat(Metal),
}

impl MaterialType {
    pub fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        match self {
            MaterialType::LambertianMat(l) => l.scatter(r, rec, attenuation, scattered),
            MaterialType::DialectricMat(d) => d.scatter(r, rec, attenuation, scattered),
            MaterialType::MetalMat(m) => m.scatter(r, rec, attenuation, scattered),
        }
    }
}
