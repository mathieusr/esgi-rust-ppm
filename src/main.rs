extern crate ppm;

use std::path::Path;

fn main(){

    let image = ppm::image::Image::new_with_file(Path::new("test.txt")).unwrap();

    //image.save_bin(Path::new("test_bin_result.ppm"));

    //let image_bin = ppm::image::Image::new_with_file_bin(Path::new("test_bin_result.ppm"));

    println!("{:?}", image);
}