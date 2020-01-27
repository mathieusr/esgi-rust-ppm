mod ppm_struct;
#[cfg(test)]
pub mod tests;

use super::pixel::Pixel;
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
        let f = BufReader::new(f);
        let mut lines = f.lines();

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

            while settings.len() != 0 {

                let line = match lines.next() {
                    Some(temp_line) => match temp_line {
                        Ok(value) => value,
                        Err(_err) => return Err(String::from("Invalid file data"))
                    },
                    None => return Err(String::from("Invalid file data"))
                }.to_string();

                if line.get(..1) == Some("#") {

                    continue;
                }

                for element in line.split_ascii_whitespace() {

                    let current_param = match settings.pop_front() {
                        Some(param) => param,
                        None => return Err(String::from("Invalid file data")),
                    };

                    current_param.set_value(element.to_string());
                }
            }
        }

        {
            for line in lines {

                let line = match line {
                    Ok(value) => value,
                    Err(_err) => return Err(String::from("Invalid file data"))
                };

                let mut inside_line_iter = line.split_ascii_whitespace();

                while let Some(line_pixel) = inside_line_iter.next() {

                    let first_value = Image::get_pixel_from_string(Some(line_pixel))?;
                    let second_value = Image::get_pixel_from_string(inside_line_iter.next())?;
                    let third_value = Image::get_pixel_from_string(inside_line_iter.next())?;

                    pixel_vector.push(Pixel::new(first_value, second_value, third_value));
                }
            }
        }
        
        if pixel_vector.len() != ppm_width.value * ppm_height.value {

            return Err(String::from("Invalid file data, pixel number don't respect width and height"))
        }

        Ok(Image{buffer: pixel_vector, width: ppm_width.value, height: ppm_height.value, max_color: ppm_max_color.value, ppm_type: ppm_type.value})
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

        file.write(self.ppm_type.as_bytes())?;
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