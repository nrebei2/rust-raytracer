use crate::{Point3, Ray, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Point3,
}

impl Camera {
    pub fn new(args: Option<(f64, f64, f64, Point3)>) -> Self {
        let (aspect_ratio, viewport_height, focal_length, origin) = if let Some(vals) = args {
            vals
        } else {
            (16.0 / 9.0, 2.0, 1.0, Point3::new(0, 0, 0))
        };

        let viewport_width = aspect_ratio * viewport_height;
        let horizontal = Vec3::new(viewport_width, 0, 0);
        let vertical = Vec3::new(0, viewport_height, 0);
        let lower_left_corner =
            &origin - &horizontal / 2. - &vertical / 2. - Vec3::new(0, 0, focal_length);

        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let to_screen =
            &self.lower_left_corner + u * &self.horizontal + v * &self.vertical - &self.origin;
        Ray::new(self.origin.clone(), to_screen)
    }
}
