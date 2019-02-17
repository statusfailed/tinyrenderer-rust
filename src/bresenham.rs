use std::ops::{Deref, DerefMut};
use image::{ImageBuffer, Pixel};

// Version of bresenham's from here: https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham%E2%80%99s-Line-Drawing-Algorithm
// And this optimization: https://github.com/ssloy/tinyrenderer/issues/28
// Except instead of a branch, we just use multiply by steep cast to int.
//
// Also it's probably really buggy because I didn't think about much very hard...
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
    if (x0 > x1) {
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
    if(error2 > dx) {
      y += if y1 > y0 { 1 } else { -1 };
      error2 -= dx*2;
    }
  }

}
