use super::{Vec3, Color};

pub fn write_color(pixel_color : Color) {
  println!("{} {} {}", 
    (pixel_color.x() * 259.999) as i32,
    (pixel_color.y() * 259.999) as i32, 
    (pixel_color.z() * 259.999) as i32)
}