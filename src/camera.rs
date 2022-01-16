use crate::{Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    u: Vec3,
    v: Vec3, 
    lens_radius: f64
}

impl Camera {
    pub fn new(lookfrom : Point3, lookat : Point3, vup : Vec3, vfov : f64, aspect_ratio : f64, aperature : f64, focus_dist : f64) -> Self {
        let theta = f64::to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (&lookfrom - lookat).unit_vec();
        let u = Vec3::cross(&vup, &w).unit_vec();
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * &u;
        let vertical = focus_dist * viewport_height * &v;
        let lower_left_corner =
            &origin - &horizontal / 2. - &vertical / 2. - focus_dist * &w;

        let lens_radius = aperature / 2.;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u, v,
            lens_radius
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = &self.u * rd.x() + &self.v * rd.y();

        let to_screen =
            &self.lower_left_corner + s * &self.horizontal + t * &self.vertical - &self.origin - &offset;
        Ray::new(&self.origin + offset, to_screen)
    }
}
