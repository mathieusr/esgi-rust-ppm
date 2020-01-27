use super::{Image, Pixel};
use std::path::Path;

pub fn get_image() -> Image {

    let image_data = (0..4).map(|_i| Pixel::new(128, 255, 128)).collect();

    Image{buffer: image_data, height: 2, width: 2, max_color: 255, ppm_type: String::from("P3")}
}

#[test]
fn check_image_invert(){

    let mut image = get_image();
    image.invert();

    let pixel = image.buffer.pop().unwrap();
    assert_eq!(pixel.red(), 127);
    assert_eq!(pixel.green(), 0);
    assert_eq!(pixel.blue(), 127);
}

#[test]
fn check_image_grayscale(){

    let mut image = get_image();
    image.grayscale();

    let pixel = image.buffer.pop().unwrap();
    assert_eq!(pixel.red(), 170);
    assert_eq!(pixel.green(), 170);
    assert_eq!(pixel.blue(), 170);
}

#[test]
fn check_read_and_write(){

    let image = get_image();
    let path = Path::new("unit_test.ppm");
    image.save(path).expect("Try to write to file");
    let image_read = Image::new_with_file(path).expect("Try to read write file in test");
    let first_pixel = image_read.buffer.first().expect("Try to get one pixel");
    let last_pixel = image_read.buffer.last().expect("Try to get one pixel");

    assert_eq!(image_read.height, 2);
    assert_eq!(image_read.width, 2);
    assert_eq!(image_read.max_color, 255);
    assert_eq!(image_read.ppm_type, "P3");
    assert_eq!(image_read.buffer.len(), 4);
    assert_eq!(first_pixel.red(), 128);
    assert_eq!(first_pixel.green(), 255);
    assert_eq!(first_pixel.blue(), 128);
    assert_eq!(last_pixel.red(), 128);
    assert_eq!(last_pixel.green(), 255);
    assert_eq!(last_pixel.blue(), 128);

    std::fs::remove_file(path).expect("Finally try to delete the file");
}