use super::wavefront;
use super::drawing::{bresenham, triangle, Triangle};
use super::test_image;

use std::fs::File;

use image;
use nalgebra::{Vector3, Vector2};

pub fn chapter1() {
  let file = File::open("data/chapter_1_head.obj").unwrap(); // TODO: nasty unwrap
  let mut model = wavefront::Model::from_file(file);

  let nvertices = model.vertices.len();
  let nfaces = model.faces.len();

  println!("chapter1: read {} vertices and {} faces",
           nvertices,
           nfaces);

  let white = image::Rgb([255, 255, 255]);
  let mut image = test_image::create_blank_image();

  let (width, height) = image.dimensions();

  for face in model.faces {

    // iterate through pairs of vertices in the face
    let facelen = face.len();
    for i in 0..(facelen - 1) {
      // NOTE: vertices are 1-indexed!?
      let v_idx_0 = face[i]                 as usize - 1; // TODO: handle the - 1 in the file format reader.
      let v_idx_1 = face[(i + 1) % facelen] as usize - 1;

      let v0: Vector3<f64> = model.vertices[v_idx_0];
      let v1: Vector3<f64> = model.vertices[v_idx_1];

      // NOTE: invert height because of upside-down y coordinates
      let x0: f64 = (v0[0] + 1.0) * width  as f64/2.;
      let y0: f64 = height as f64 - (v0[1] + 1.0) * height as f64/2.;

      let x1: f64 = (v1[0] + 1.0) * width  as f64/2.;
      let y1: f64 = height as f64 - (v1[1] + 1.0) * height as f64/2.;

      //println!("{} {} {} {}", x0 as i32, y0 as i32, x1 as i32, y1 as i32);
      let w = width as i32;
      let h = height as i32;
      bresenham(x0 as i32 % w, y0 as i32 % h, x1 as i32 % w, y1 as i32 % h, &mut image, white);
    }
  }

  //bresenham(0, 0, 250, 250, &mut image, white);
  bresenham(223, 226, 223, 226, &mut image, white);
  let filename = "images/chapter1.png";
  image.save(filename);
  println!("saved to {}", filename);
}

pub fn chapter2() {
	// 500x500
  let white = image::Rgb([255, 255, 255]);
  let mut image = test_image::create_blank_image();
	let triangle_points: Triangle = [
		Vector2::new(10., 10.),
    Vector2::new(100., 30.),
    Vector2::new(190., 160.),
  ];
  triangle(triangle_points, &mut image, white);
  let filename = "images/chapter2.png";
  image::imageops::flip_vertical(&mut image).save(filename);
  println!("saved to {}", filename);
}
