extern crate ppm;

use ppm::pixel::Pixel;

fn main(){

    let mut base = Pixel::new(1, 2, 3);

    println!("{}", base);

    base.grayscale();

    println!("{}", base);
}