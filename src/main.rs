mod vec3;
mod color;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod rtweekend;
mod camera;
mod material;

use crate::vec3::{ Color, Point3, Vec3 };
use crate::color::write_color;
use crate::ray::Ray;
use crate::hittable_list::HittableList;
use crate::hittable::{ Hittable, HitRecord };
use crate::material::{ Dielectric, Lambertian, Material, Metal };
use crate::sphere::Sphere;
use crate::camera::Camera;

use std::rc::Rc;
use std::f32;

extern crate rand;
use rand::Rng;

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    let mut rec = HitRecord::new_empty(); 
    if world.hit(r, 0.001, f32::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Point3::ZERO, Vec3::ZERO);
        let mut attenuation = Color::ZERO;

        let mat_ptr = rec.mat_ptr.clone();

        if mat_ptr.scatter(&r, &mut rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Color::ZERO;
    }

    let unit_direction = r.direction.clone().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0., -1000., 0.), 1000., ground_material)));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Point3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = 0.5 * rng.gen::<f32>();
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Point3::new(0., 1., 0.), 1., material1)));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Point3::new(-4., 1., 0.), 1., material2)));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    world.add(Rc::new(Sphere::new(Point3::new(4., 1., 0.), 1., material3)));
    
    return world;
}

fn main() {
    let mut rng = rand::thread_rng();

    // Image

    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const MAX_DEPTH: i32 = 50;

    //World
    
    let world = random_scene();

    // Camera

    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;

    let camera = Camera::new(lookfrom, lookat, vup, 20., ASPECT_RATIO, aperture, dist_to_focus);

    let header = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    print!("{}", header);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0., 0., 0.);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (IMAGE_WIDTH as f32 - 1.);
                let v = (j as f32 + rng.gen::<f32>()) / (IMAGE_HEIGHT as f32 - 1.);
                
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Done");
}
