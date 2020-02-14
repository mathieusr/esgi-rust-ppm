# PPM Library

## Author

Library write by:
* Mathieu SERRA
* Kevin Bourmier
* Hugo Terral

## Functionalities

### Read

You can easily read different type of PPM with the library. Below you will find all the posibilities to read a ppm file. All this examples return an image object

#### ASCII file

To read an ASCII file:
```
extern crate ppm;

use std::path::Path;

let image = ppm::image::Image::new_with_file(Path::new("test.txt")).unwrap();
```

#### Binary file
```
extern crate ppm;

use std::path::Path;

let image = ppm::image::Image::new_with_file_bin(Path::new("test.txt")).unwrap();
```

#### Use an external library
```
extern crate ppm;

use std::path::Path;

let image = ppm::image::extern_ppm_lib::read("test.txt");
```

### Write

After reading your file and getting your Image object you can rewrite it. The library offer your three way to write it.

#### ASCII File
```
image.save(Path::new("test_result.txt"))
```

#### Binary file
```
image.save_bin(Path::new("test_result.txt"))
```

#### Via an external libray
```
ppm::image::extern_ppm_lib::write("test_result.txt", myImage)
```

### Update your image object

Before rewrite your image you can update it

#### Grayscale
You can easily grayscale your image object
```
image.grayscale()
```

#### Invert color
You can easily invert the color or your image
```
image.invert()
```

## Tests

The library includes different type of tests

### Unit test 
The library implement unit tests. To run them:
```
cargo test
```

### Benchmark test
The library implement benchmark tests. To run them:
```
cargo bench
``` 