use nalgebra::{Vector3, Vector2};

use std::ops::{Deref, DerefMut};
use image::{ImageBuffer, Pixel};

pub type Triangle = [Vector2<f64>; 3];

// this is nice:
// https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/barycentric-coordinates
pub fn barycentric(p: Vector2<f64>, triangle: Triangle) -> Vector3<f64> {
  /* Vec3f u = cross(
         Vec3f(pts[2].x-pts[0].x, pts[1].x-pts[0].x, pts[0].x-p.x)
       , Vec3f(pts[2].y-pts[0].y, pts[1].y-pts[0].y, pts[0].y-P.y)
     );
   */
  let ab = triangle[2] - triangle[0];
  let ac = triangle[1] - triangle[0];
  let pa = triangle[0] - p;

  // I don't understand what's going on
  let c = Vector3::new(ab.x, ac.x, pa.x);
  let d = Vector3::new(ab.y, ac.y, pa.y);

  let u = c.cross(&d);

  if u.z.abs() < 1.0 {
    return Vector3::new(-1.0, 1.0, 1.0);
  }

  return Vector3::new(1.0 - (u.x+u.y)/u.z, u.y/u.z, u.x/u.z); 
}

pub fn triangle<P, Container>(
  triangle: Triangle, 
  buf: &mut ImageBuffer<P, Container>,
  val: P)
  where
    P: Pixel + 'static,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
  // TODO: calculate bounding box, don't iterate through whole image!

  for x in 0..(buf.width() - 1) {
    for y in 0..(buf.height() - 1) {
      let p = Vector2::new(x as f64, y as f64);
      let bc_screen = barycentric(p, triangle);
      if bc_screen.x < 0. || bc_screen.y < 0. || bc_screen.z < 0. {
          continue
      }; 
      buf.put_pixel(p.x as u32, p.y as u32, val); 
    }
  }
}

// Version of bresenham's from here: https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham%E2%80%99s-Line-Drawing-Algorithm
// And this optimization: https://github.com/ssloy/tinyrenderer/issues/28
// Except instead of a branch, we just use multiply by steep cast to int.
//
// Also it's probably really buggy because I didn't think about much very hard...
// Passing mutable references around: https://stackoverflow.com/questions/23574416/
pub fn bresenham<P, Container>(
  x0: i32,
  y0: i32,
  x1: i32,
  y1: i32,
  buf: &mut ImageBuffer<P, Container>,
  val: P)
  where
    P: Pixel + 'static,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
  let (x0, y0, x1, y1, steep) =
    if (x0-x1).abs() < (y0-y1).abs() {
      (y0, x0, y1, x1, true)
    } else {
      (x0, y0, x1, y1, false)
    };

  let (x0, y0, x1, y1) =
    if x0 > x1 {
      (x1, y1, x0, y0)
    } else {
      (x0, y0, x1, y1)
    };

  let dx: i32 = (x1 - x0) as i32;
  let dy: i32 = (y1 - y0) as i32;
  let derror2: i32 = dy.abs()*2;

  let mut error2: i32 = 0;
  let mut y: i32 = y0;

  let steep_x = if steep { 1 } else { 0 };
  let steep_y = 1 - steep_x;

  for x in x0..x1 {
    // if(steep) { put_pixel(y, x) } else { put_pixel(x, y) }
    let cx = steep_y * x + steep_x * y;
    let cy = steep_x * x + steep_y * y;
    buf.put_pixel(cx as u32, cy as u32, val); // 

    error2 += derror2;
    if error2 > dx {
      y += if y1 > y0 { 1 } else { -1 };
      error2 -= dx*2;
    }
  }

}
