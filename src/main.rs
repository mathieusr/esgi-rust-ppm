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
    let path_output = Path::new("./test_result.txt");

    match Image::new_with_file(path).as_mut() {
        Ok(result) => {

            result.invert();

            match result.save(path_output) {
                Ok(()) => println!("Write succeded"),
                Err(err) => println!("{:?}", err)
            }
        },
        Err(err) => println!("Error: {}", err)
    };
}