extern crate image;
extern crate nalgebra;

use std::fs::File;
use std::path::Path;

mod test_image;
mod bresenham;
mod wavefront;

//use image::{ImageBuffer, Rgb};

mod chapters;

fn main() {
  chapters::chapter1();
  println!("Done!")
}

fn old_main() {
  let mut image = test_image::create_blank_image();
  let white = image::Rgb([255, 255, 255]);
  bresenham::bresenham(10, 10, 20, 500, &mut image, white);
  image.save("out.png");
  println!("saved image");

  println!("Reading model...");
  let file = File::open(Path::new("data/chapter_1_head.obj"));
  let mut model = wavefront::Model::from_file(file.unwrap());
  println!("Done! read {} vertices and {} faces.", model.vertices.len(), model.faces.len());
}
