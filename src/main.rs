mod vec3;
mod material;
mod sphere;
mod hitable;
mod hitable_list;
mod ray;
mod lambertian;
mod metal;
mod dialectric;
mod camera;

use vec3::Vec3;
use material::MaterialType;
use sphere::Sphere;
use hitable::Hitable;
use hitable_list::HitableList;
use ray::Ray;
use lambertian::Lambertian;
use metal::Metal;
use dialectric::Dialectric;
use camera::Camera;

fn color<T: Hitable>(r: &Ray, world: T, depth: i32) -> Vec3 {
    let mut rec = hitable::HitRecord::new();
    if world.hit(r, 0.001, std::f32::MAX, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::default();
        if depth < 50 && rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * color(&scattered, world, depth+1);
        }
        return Vec3::default();
    }

    let unit_direction = r.direction().unit_vector();
    let t = (unit_direction.y() + 1.0) * 0.5;
    return (Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Vec3::new(0.5, 0.7, 1.0) * t);
}

fn random_scene() -> HitableList<Sphere> {
    let mut list: Vec<Sphere> = Vec::new();

    let mat1 = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
    list.push(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, MaterialType::LambertianMat(mat1)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f32>();
            let center = Vec3::new(a as f32 + 0.9 * rand::random::<f32>(), 0.2, b as f32 + 0.9 * rand::random::<f32>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let mat = Lambertian::new(Vec3::new(
                        rand::random::<f32>() * rand::random::<f32>(),
                        rand::random::<f32>() * rand::random::<f32>(),
                        rand::random::<f32>() * rand::random::<f32>()
                    ));
                    list.push(Sphere::new(center, 0.2, MaterialType::LambertianMat(mat)));
                } else if choose_mat < 0.95 {
                    let mat = Metal::new(
                        Vec3::new(
                            0.5 * (1.0 + rand::random::<f32>()),
                            0.5 * (1.0 + rand::random::<f32>()),
                            0.5 * (1.0 + rand::random::<f32>())
                        ),
                        0.5 * rand::random::<f32>()
                    );
                    list.push(Sphere::new(center, 0.2, MaterialType::MetalMat(mat)));
                } else {
                    let mat = Dialectric::new(1.5);
                    list.push(Sphere::new(center, 0.2, MaterialType::DialectricMat(mat)));
                }
            }
        }
    }

    let mat2 = Dialectric::new(1.5);
    list.push(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, MaterialType::DialectricMat(mat2)));

    let mat3 = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
    list.push(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, MaterialType::LambertianMat(mat3)));

    let mat4 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    list.push(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, MaterialType::MetalMat(mat4)));

    return HitableList::new(list);
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let world = random_scene();

    let lookfrom = Vec3::new(12.0, 2.0, 4.0);
    let lookat = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.2;
    let cam = Camera::new(lookfrom, lookat, Vec3::new(0.0, 1.0, 0.0), 20.0, nx as f32 / ny as f32, aperture, dist_to_focus);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::default();
            for _s in 0..ns {
                let u = (i as f32 + rand::random::<f32>()) / nx as f32;
                let v = (j as f32 + rand::random::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                col = col + color(&r, world.clone(), 0);
            }
            col = col / ns as f32;
            col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
            let ir = (255.99 * col.x()) as i32;
            let ig = (255.99 * col.y()) as i32;
            let ib = (255.99 * col.z()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
