extern crate ppm;

//use ppm::pixel::Pixel;
use ppm::image::Image;
use std::path::Path;

fn main(){

    // let mut base = Pixel::new(1, 2, 3);

    // println!("{}", base);

    // base.grayscale();

    // println!("{}", base);

    let path = Path::new("./test.txt");

    match Image::new_with_file(path) {
        Ok(_result) => println!("It's ok {} {}", _result.width, _result.height),
        Err(err) => println!("Error: {}", err)
    };
}