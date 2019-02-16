extern crate image;

mod test_image;

fn main() {
  test_image::create_test_image().save("out.png");
  println!("saved image");
}
