extern crate image;

mod test_image;
mod bresenham;

use image::{ImageBuffer, Rgb};

// Passing mutable references around: https://stackoverflow.com/questions/23574416/
fn main() {
  let mut image = test_image::create_blank_image();
  let white = image::Rgb([255, 255, 255]);
  bresenham::bresenham(10, 10, 100, 100, &mut image, white);
  image.save("out.png");
  println!("saved image");
}
