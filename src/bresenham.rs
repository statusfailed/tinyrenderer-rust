use std::ops::{Deref, DerefMut, Index, IndexMut};
use image::{ImageBuffer, Pixel};

pub fn bresenham<P, Container>(
  x0: u32,
  y0: u32,
  x1: u32,
  y1: u32,
  buf: &mut ImageBuffer<P, Container>,
  val: P)
  where
    P: Pixel + 'static,
    Container: Deref<Target = [P::Subpixel]> + DerefMut,
{
  let deltax = (x1 - x0) as f64;
  let deltay = (y1 - y0) as f64;
  let deltaerr = (deltay / deltax).abs();

  let mut error = 0.0;
  let mut y = y0;

  for x in x0..x1 {
    buf.put_pixel(x, y, val);
    error = error + deltaerr;
    if error >= 0.5 {
      y = y + (deltay.signum() * 1.) as u32;
      error = error - 1.0;
    }

  }
}
