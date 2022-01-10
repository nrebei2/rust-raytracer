mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;

#[macro_use] extern crate impl_ops;

use vec3::{Vec3, Color, Point3};
use ray::Ray;
use hittable_list::HittableList;
use std::rc::Rc;
use sphere::Sphere;
use hittable::{HitRecord, Hittable};

fn ray_color(ray : &Ray, world : &HittableList) -> Color {
  let mut rec = HitRecord::new();

  if world.hit(ray, 0., f64::INFINITY, &mut rec) {
    return 0.5 * (rec.normal + Color::create(1, 1, 1))
  }

  let unit_direction = ray.direction().unit_vec();
  let t = 0.5 * (unit_direction.y() + 1.0);
  (1.0 - t) * Color::create(1, 1, 1) + t * Color::create(0.5,0.7, 1.0)
}

fn main() {

  // Image Dimensions
  const ASPECT_RATIO : f64 = 16.0 / 9.0;
  const IMAGE_WIDTH : i32 = 400;
  const IMAGE_HEIGHT : i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;


  // World

  let mut world = HittableList::new();
  world.add(Rc::new(Sphere::new(Point3::create(0,0,-1), 0.5)));
  world.add(Rc::new(Sphere::new(Point3::create(0,-100.5,-1), 100.)));

  // Camera

  let viewport_height = 2.0;
  let viewport_width = ASPECT_RATIO * viewport_height;
  let focal_length = 1.0;

  let origin = Point3::create(0, 0, 0);
  let horizontal = Vec3::create(viewport_width, 0, 0);
  let vertical = Vec3::create(0, viewport_height, 0);
  let lower_left_corner = &origin - &horizontal/2. - &vertical/2. - Vec3::create(0, 0, focal_length);

  // Render

  println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

  for j in (0..IMAGE_HEIGHT).rev() {
    eprintln!("{} lines remaining!", j);
    for i in 0..IMAGE_WIDTH {
      let u = i as f64 / (IMAGE_WIDTH-1) as f64;
      let v = j as f64 / (IMAGE_HEIGHT-1) as f64;

      let to_screen = &lower_left_corner + u*&horizontal + v*&vertical - &origin;

      let r = Ray::new(&origin, &to_screen);

      let pixel_color = ray_color(&r, &world);

      color::write_color(pixel_color)
    }
  }

}