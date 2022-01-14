use crate::Color;

pub fn write_color(pixel_color: Color, sample_per_pixels: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / (sample_per_pixels as f64);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    println!(
        "{} {} {}",
        (256. * f64::clamp(r, 0.0, 0.999)) as i32,
        (256. * f64::clamp(g, 0.0, 0.999)) as i32,
        (256. * f64::clamp(b, 0.0, 0.999)) as i32
    )
}
