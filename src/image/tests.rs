use super::Image;
use std::path::Path;

#[test]
fn check_header(){

    let image = Image::new_with_file(Path::new("./test.txt")).unwrap();

    assert_eq!(image.width, 4);
    assert_eq!(image.height, 4);
    assert_eq!(image.max_color, 15);
    assert_eq!(image.ppm_type, "P3");
}