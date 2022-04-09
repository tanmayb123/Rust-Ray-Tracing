use crate::{
    hitable::{HitRecord, Hitable},
    vec3::Vec3,
    material::MaterialType,
    ray::Ray,
};

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: MaterialType,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: MaterialType) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.mat = self.material.clone();
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.mat = self.material.clone();
                return true;
            }
        }
        false
    }
}
