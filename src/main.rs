extern crate ppm;

use std::path::Path;

fn main(){

    let mut image = ppm::image_binary::ImageBinary::new_with_file(Path::new("output_4.ppm")).unwrap();

    image.grayscale(Path::new("test_result_gray.txt")).unwrap();
    //image.grayscale(Path::new("test_result_invert.txt")).unwrap();
}