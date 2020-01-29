extern crate libc;
use libc::{c_char, c_uint};
use super::Image;
use crate::pixel::Pixel;
use std::ffi::CString;

#[link(name = "ppma_io")]
extern "C"{

    fn ppma_read (input_name: *const c_char, xsize: *const c_uint, ysize: *const c_uint, rgb_max: *const c_uint, r: *const *const c_uint, g: *const *const c_uint, b: *const *const c_uint);

    fn ppma_write ( input_name: *const c_char, xsize: c_uint, ysize: c_uint, r: *const c_uint, g: *const c_uint, b: *const c_uint) -> c_uint;
}

pub fn read(file_name: &str) -> Image {

    unsafe {

        let xsize: c_uint = 0;
        let ysize: c_uint = 0;
        let rgb_max: c_uint = 0;
        let r_pointer: *const c_uint = std::ptr::null();
        let g_pointer: *const c_uint = std::ptr::null();
        let b_pointer: *const c_uint = std::ptr::null();

        ppma_read(CString::new(file_name).unwrap().as_ptr(), &xsize, &ysize, &rgb_max, &r_pointer, &g_pointer, &b_pointer);

        let r_slice = std::slice::from_raw_parts(r_pointer, (xsize * ysize) as usize);
        let g_slice = std::slice::from_raw_parts(g_pointer, (xsize * ysize) as usize);
        let b_slice = std::slice::from_raw_parts(b_pointer, (xsize * ysize) as usize);
        let mut pixel_vector: Vec<Pixel> = Vec::new();

        for i in 0..(xsize * ysize) {

            pixel_vector.push(Pixel::new(r_slice[i as usize] as u8, g_slice[i as usize] as u8, b_slice[i as usize] as u8));
        }

        Image{buffer: pixel_vector, width: xsize as usize, height: ysize as usize, max_color: rgb_max as usize, ppm_type: String::new()}
    }
}

pub fn write(file_name: &str, image: &Image){

    let mut r_pixel = Vec::<u32>::new();
    let mut g_pixel = Vec::<u32>::new();
    let mut b_pixel = Vec::<u32>::new();

    for i in 0..(image.width * image.height) {

        let pixel = image.buffer.get(i).unwrap();
        r_pixel.push(pixel.red() as u32);
        g_pixel.push(pixel.green() as u32);
        b_pixel.push(pixel.blue() as u32);
    }

    unsafe {

        ppma_write(CString::new(file_name).unwrap().as_ptr(), image.width as u32, image.height as u32, (&r_pixel).as_ptr(), (&g_pixel).as_ptr(), (&b_pixel).as_ptr());
    }
}