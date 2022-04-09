use crate::{
    vec3::Vec3,
    ray::Ray,
};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let lens_radius = aperture / 2.0;

        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = lookfrom;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let lower_left_corner = origin
            - u * half_width * focus_dist
            - v * half_height * focus_dist
            - w * focus_dist;

        let horizontal = u * 2.0 * half_width * focus_dist;
        let vertical = v * 2.0 * half_height * focus_dist;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = Vec3::random_in_unit_sphere() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner
                + self.horizontal * u
                + self.vertical * v
                - self.origin
                - offset,
        )
    }
}
