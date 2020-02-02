mod ppm_struct;
pub mod extern_ppm_lib;
#[cfg(test)]
mod tests;

use crate::pixel::Pixel;
use std::path::Path;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;
use std::collections::VecDeque;
use ppm_struct::{PpmStringElement, PpmIntElement, PpmValue};

#[derive(Debug, Clone)]
pub struct Image {
    pub buffer: Vec<Pixel>,
    pub width: usize,
    pub height: usize,
    pub max_color: usize,
    pub ppm_type: String
}


impl Image {

    pub fn new_with_file(filename: &Path) -> Result<Image, String> {

        let f = match File::open(filename){
            Ok(file) => file,
            Err(_e) => return Err(String::from("Can't open the file")),
        };
        let mut f = BufReader::new(f);

        let mut ppm_type = PpmStringElement{name: String::from("ppm type"), value: String::new()};
        let mut ppm_width = PpmIntElement{name: String::from("ppm width"), value: 0};
        let mut ppm_height = PpmIntElement{name: String::from("ppm height"), value: 0};
        let mut ppm_max_color = PpmIntElement{name: String::from("ppm max color"), value: 0};

        let mut pixel_vector: Vec<Pixel> = Vec::new();
        

        {
            let mut settings: VecDeque<&mut dyn PpmValue> = VecDeque::new();
            settings.push_back(&mut ppm_type);
            settings.push_back(&mut ppm_width);
            settings.push_back(&mut ppm_height);
            settings.push_back(&mut ppm_max_color);

            Image::read_header(&mut settings, &mut f)?;
        }

        Image::read_data(&mut pixel_vector, &mut f)?;
        
        if pixel_vector.len() != ppm_width.value * ppm_height.value {

            return Err(String::from("Invalid file data, pixel number don't respect width and height"))
        }

        Ok(Image{buffer: pixel_vector, width: ppm_width.value, height: ppm_height.value, max_color: ppm_max_color.value, ppm_type: ppm_type.value})
    }

    pub fn new_with_file_bin(filename: &Path) -> Result<Image, String> {

        let f = match File::open(filename){
            Ok(file) => file,
            Err(_e) => return Err(String::from("Can't open the file")),
        };
        let mut buff = Vec::<u8>::new();
        let mut f = BufReader::new(f);

        let mut ppm_type = PpmStringElement{name: String::from("ppm type"), value: String::new()};
        let mut ppm_width = PpmIntElement{name: String::from("ppm width"), value: 0};
        let mut ppm_height = PpmIntElement{name: String::from("ppm height"), value: 0};
        let mut ppm_max_color = PpmIntElement{name: String::from("ppm max color"), value: 0};

        let mut pixel_vector: Vec<Pixel> = Vec::new();
        

        {
            let mut settings: VecDeque<&mut dyn PpmValue> = VecDeque::new();
            settings.push_back(&mut ppm_type);
            settings.push_back(&mut ppm_width);
            settings.push_back(&mut ppm_height);
            settings.push_back(&mut ppm_max_color);

            Image::read_header(&mut settings, &mut f)?;
        }

        loop {

            match f.read_until(255, &mut buff) {
                Ok(read_size) => {

                    if read_size == 0 {

                        break;
                    }
                    read_size
                },
                Err(_e) => break,
            };

            while buff.len() >= 3 {

                let r = buff.remove(0);
                let g = buff.remove(0);
                let b = buff.remove(0);

                pixel_vector.push(Pixel::new(r, g, b));
            }
        }

        Ok(Image{buffer: pixel_vector, width: ppm_width.value, height: ppm_height.value, max_color: ppm_max_color.value, ppm_type: ppm_type.value})
    }

    fn read_header(settings: &mut VecDeque<&mut dyn PpmValue>, buffer: &mut BufReader<File>) -> Result<(), String>{

        let mut temp_string = String::from("");

        while settings.len() != 0 {

            let _read_size = match buffer.read_line(&mut temp_string) {
                Ok(read_size) => {
                    if read_size == 0 && settings.len() != 0 {

                        return Err(String::from("Invalid file data"));
                    }

                    read_size
                },
                Err(_e) => return Err(String::from("Invalid file data"))
            };

            if temp_string.get(..1) == Some("#") {

                temp_string.clear();
                continue;
            }

            for element in temp_string.split_ascii_whitespace() {

                let current_param = match settings.pop_front() {
                    Some(param) => param,
                    None => return Err(String::from("Invalid file data")),
                };

                current_param.set_value(element.to_string());
            }

            temp_string.clear();
        }

        Ok(())
    }

    fn read_data(pixel_vector: &mut Vec<Pixel>, buffer: &mut BufReader<File>) -> Result<(), String>{

        let mut temp_string = String::from("");

        loop {

            let _read_size = match buffer.read_line(&mut temp_string) {
                Ok(read_size) => {
                    if read_size == 0 {

                        break;
                    }

                    read_size
                },
                Err(_e) => return Err(String::from("Invalid file data"))
            };

            let mut inside_line_iter = temp_string.split_ascii_whitespace();

            while let Some(line_pixel) = inside_line_iter.next() {

                let first_value = Image::get_pixel_from_string(Some(line_pixel))?;
                let second_value = Image::get_pixel_from_string(inside_line_iter.next())?;
                let third_value = Image::get_pixel_from_string(inside_line_iter.next())?;

                pixel_vector.push(Pixel::new(first_value, second_value, third_value));
            }

            temp_string.clear();
        }

        Ok(())
    }

    fn get_pixel_from_string(current_value: Option<&str>) -> Result<u8, String> {

        match current_value {
            Some(value) => match value.parse::<u8>() {
                Ok(value) => Ok(value),
                Err(_err) => Err(String::from("Invalid file data"))
            },
            None => Err(String::from("Invalid file data"))
        }
    }

    pub fn save(&self, filename: &Path) -> std::io::Result<()> {

        let file = File::create(filename)?;
        let mut file = BufWriter::new(file);

        file.write("P3".as_bytes())?;
        file.write(format!("\n{} {}\n{}\n", self.width, self.height, self.max_color).as_bytes())?;

        let mut current_width = 0;
        let mut current_line_size = 0;

        for pixel in &self.buffer {

            if current_width == self.width || current_line_size > 70 {

                file.write(b"\n")?;
                current_width = 0;
                current_line_size = 0;
            }

            file.write(format!("{} {} {}  ", pixel.red(), pixel.green(), pixel.blue()).as_bytes())?;

            current_width += 1;
            current_line_size += 7;
        }

        file.flush()
    }

    pub fn save_bin(&self, filename: &Path) -> std::io::Result<()> {

        let file = File::create(filename)?;
        let mut file = BufWriter::new(file);

        file.write("P6".as_bytes())?;
        file.write(format!("\n{} {}\n{}\n", self.width, self.height, self.max_color).as_bytes())?;

        for pixel in &self.buffer {

            file.write(&[pixel.red(), pixel.green(), pixel.blue()])?;
        }

        file.flush()
    }

    pub fn invert(&mut self){

        for pixel in &mut self.buffer {

            pixel.invert();
        }
    }

    pub fn grayscale(&mut self){

        for pixel in &mut self.buffer {

            pixel.grayscale();
        }
    }
}