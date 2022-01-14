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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0, -1000, 0), 1000., material_ground)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_float();
            let center = Point3::new(a as f64 + 0.9*random_float(), 0.2, b as f64 + 0.9*random_float());

            if (&center - Point3::new(4, 0.2, 0)).length() > 0.9 {
                let sphere_material : Rc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)))
                } else if choose_mat < 0.95 {
                    // metal 
                    let albedo = Color::random_rng(0.5, 1.);
                    let fuzz = random_float_rng(0., 0.5);

                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)))
                } else {
                    // glass 
                    sphere_material = Rc::new(Dielectic::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }
        }
    }

    let material1 = Rc::new(Dielectic::new(1.5));
    world.add(Rc::new(Sphere::new(Point3::new(0, 1, 0), 1.0, material1)));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Point3::new(-4, 1, 0), 1.0, material2)));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.));
    world.add(Rc::new(Sphere::new(Point3::new(4, 1, 0), 1.0, material3)));


    world
}

fn ray_color(ray: Ray, world: &HittableList, depth : i32) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {return Color::new(0, 0, 0)}

    if world.hit(&ray, 0.001, f64::INFINITY, &mut rec) {
        // if !rec.front_face {eprintln!("Inside sphere!")};
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
    const ASPECT_RATIO: f64 = 3. / 2.;
    const IMAGE_WIDTH: i32 = 300;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 5;
    const MAX_DEPTH : i32 = 5;

    // World
        
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13., 2., 3.);
    let lookat = Point3::new(0, 0, -1);
    let vup = Vec3::new(0, 1, 0);
    let dist_to_focus = 10.;
    let aperature = 0.1;
    let cam = Camera::new(lookfrom, lookat, vup, 20., ASPECT_RATIO, aperature, dist_to_focus);

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
