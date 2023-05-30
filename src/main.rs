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
use crate::material::{ Lambertian, Metal };
use crate::sphere::Sphere;
use crate::camera::Camera;

use std::rc::Rc;

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

fn main() {
    let mut rng = rand::thread_rng();

    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    //World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.));

    world.add(Rc::new(Sphere::new(Point3::new(0., -100.5, -1.), 100., material_ground)));
    world.add(Rc::new(Sphere::new(Point3::new(0., 0., -1.), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(Point3::new(-1., 0., -1.), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(Point3::new(1., 0., -1.), 0.5, material_right)));

    // Camera
    let camera = Camera::default_camera();

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
