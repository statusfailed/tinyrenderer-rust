extern crate image;
extern crate nalgebra;

mod test_image;
mod wavefront;
mod drawing;

//use image::{ImageBuffer, Rgb};

mod chapters;

fn main() {
  //chapters::chapter1();
  chapters::chapter2();
  println!("Done!")
}

/*
use std::fs::File;
use std::path::Path;

fn old_main() {
  let mut image = test_image::create_blank_image();
  let white = image::Rgb([255, 255, 255]);
  drawing::bresenham(10, 10, 20, 500, &mut image, white);
  image.save("out.png");
  println!("saved image");

  println!("Reading model...");
  let file = File::open(Path::new("data/chapter_1_head.obj"));
  let mut model = wavefront::Model::from_file(file.unwrap());
  println!("Done! read {} vertices and {} faces.", model.vertices.len(), model.faces.len());
}
*/
