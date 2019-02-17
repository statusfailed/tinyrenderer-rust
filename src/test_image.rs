use image::{ImageBuffer, Rgb};

const DX: usize = 512;
const DY: usize = 512;
const SIZE: usize = DX * DY;

// confusing! https://stackoverflow.com/questions/53212611/
pub fn create_test_image() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  let mut buf = vec![128 as u8; SIZE * 3];
  for i in 0..(SIZE*3) {
    let x = i % DX;
    let y = i / DY;
    buf[i] = (x * y) as u8;
  }

  ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(DX as u32, DY as u32, buf).unwrap()
}

pub fn create_blank_image() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  let buf = vec![0 as u8; SIZE * 3];
  ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(DX as u32, DY as u32, buf).unwrap()
}
