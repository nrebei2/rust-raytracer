mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

#[macro_use]
extern crate impl_ops;

use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use std::time::Instant;
use vec3::{Color, Point3, Vec3};

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::new();

    if world.hit(ray, 0., f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1, 1, 1));
    }

    let unit_direction = ray.direction().unit_vec();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1, 1, 1) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image Dimensions
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    // World

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0, 0, -1), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0, -100.5, -1), 100.)));

    // Camera

    let cam = Camera::new(None);

    // Render

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    let now = Instant::now();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("{} lines remaining!", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0, 0, 0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::thread_rng().gen_range(0.0..1.0)) as f64
                    / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand::thread_rng().gen_range(0.0..1.0)) as f64
                    / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            color::write_color(pixel_color, SAMPLES_PER_PIXEL)
        }
    }
    let elapsed = now.elapsed();
    eprintln!("Render elapsed: {:.2?}", elapsed);
}
