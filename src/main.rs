mod vec3;
mod color;

use crate::vec3::Color;
use crate::color::write_color;

use std::io;

fn main() {
    const image_width: i32 = 256;
    const image_height: i32 = 256;

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    print!("{}", header);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let pixel_color = Color::new(i as f32 / (image_width as f32 - 1.0), j as f32 / (image_height as f32 - 1.0), 0.25);
            write_color(pixel_color);
        }
    }

    eprintln!("Done");
}
