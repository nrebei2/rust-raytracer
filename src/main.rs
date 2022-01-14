mod camera;
mod hittable;
mod vec3;
mod utility;
mod material;

#[macro_use]
extern crate impl_ops;

use camera::Camera;
use hittable::{HitRecord, Hittable, sphere::Sphere, hittable_list::HittableList};
use std::rc::Rc;
use std::time::Instant;
use vec3::{Color, Point3, Vec3, color, ray::*};
use utility::*;
use material::*;

fn ray_color(ray: Ray, world: &HittableList, depth : i32) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {return Color::new(0, 0, 0)}

    if world.hit(&ray, 0.001, f64::INFINITY, &mut rec) {
        if !rec.front_face {eprintln!("Inside sphere!")};
        let mut scattered = Ray::create();
        let mut attenuation = Color::create();

        if rec.mat_ptr.scatter(&ray, &rec, &mut attenuation, &mut scattered) {     
            return attenuation * ray_color(scattered, world, depth - 1)
        }
        return Color::new(0, 0, 0)
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
    const SAMPLES_PER_PIXEL: i32 = 10;
    const MAX_DEPTH : i32 = 20;

    // World
        
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));


    world.add(Rc::new(Sphere::new(Point3::new(0, -100.5, -1), 100., material_ground)));
    world.add(Rc::new(Sphere::new(Point3::new(0, 0, -1), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(Point3::new(-1, 0, -1), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(Point3::new(1, 0, -1), 0.5, material_right)));

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
                let u = (i as f64 + random_float()) as f64
                    / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_float()) as f64
                    / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, MAX_DEPTH);
            }

            color::write_color(pixel_color, SAMPLES_PER_PIXEL)
        }
    }
    let elapsed = now.elapsed();
    eprintln!("Render elapsed: {:.2?}", elapsed);

}
