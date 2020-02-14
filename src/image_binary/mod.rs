use std::io::prelude::*;
use std::path::Path;
use std::io::{BufReader, BufWriter, SeekFrom};
use std::fs::File;
use std::collections::VecDeque;
use crate::image::ppm_struct::{PpmIntElement, PpmStringElement, PpmValue};
use crate::pixel::Pixel;

#[derive(Debug)]
pub struct ImageBinary {
    pub buffer: BufReader<File>,
    pub width: usize,
    pub height: usize,
    pub max_color: usize,
    pub ppm_type: String
}

impl ImageBinary {

    pub fn new_with_file(filename: &Path) -> Result<ImageBinary, String> {

        let f = match File::open(filename){
            Ok(file) => file,
            Err(_e) => return Err(String::from("Can't open the file")),
        };
        let mut f = BufReader::new(f);

        let mut ppm_type = PpmStringElement{name: String::from("ppm type"), value: String::new()};
        let mut ppm_width = PpmIntElement{name: String::from("ppm width"), value: 0};
        let mut ppm_height = PpmIntElement{name: String::from("ppm height"), value: 0};
        let mut ppm_max_color = PpmIntElement{name: String::from("ppm max color"), value: 0};
        

        {
            let mut settings: VecDeque<&mut dyn PpmValue> = VecDeque::new();
            settings.push_back(&mut ppm_type);
            settings.push_back(&mut ppm_width);
            settings.push_back(&mut ppm_height);
            settings.push_back(&mut ppm_max_color);

            ImageBinary::read_header(&mut settings, &mut f)?;
        }

        Ok(ImageBinary{buffer: f, width: ppm_width.value, height: ppm_height.value, max_color: ppm_max_color.value, ppm_type: ppm_type.value})
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

    pub fn grayscale(&mut self, filename: &Path) -> std::io::Result<()>{

        self.transform(filename, &GrayscaleTransform{})
    }

    pub fn invert(&mut self, filename: &Path) -> std::io::Result<()>{

        self.transform(filename, &InvertTransform{})
    }

    pub fn transform(&mut self, filename: &Path, transform_method: &dyn TransformImage) -> std::io::Result<()>{

        let file = File::create(filename)?;
        let mut file = BufWriter::new(file);
        let max: u64 = 1024 * 1024 * 3;
        let mut read_buffer: [u8; 1024 * 1024 * 3] = [0; 1024 * 1024 * 3];
        let current = self.buffer.seek(SeekFrom::Current(0))?;
        let mut temp_current = current;
        let end = self.buffer.seek(SeekFrom::End(0))?;

        self.buffer.seek(SeekFrom::Start(current))?;

        file.write("P6".as_bytes())?;
        file.write(format!("\n{} {}\n{}\n", self.width, self.height, self.max_color).as_bytes())?;

        loop {

            if temp_current == end {
                
                println!("Je break");
                break;
            }

            let buff_size = if temp_current + max > end {

                end - temp_current

            }else{

                max
            };

            let mut slice_reader = &mut read_buffer[1..((buff_size + 1) as usize)];

            self.buffer.read_exact(&mut slice_reader)?;

            for i in 0..(slice_reader.len() / 3) {

                let mut temp_pixel = Pixel::new(slice_reader[i * 3], slice_reader[(i * 3) + 1], slice_reader[(i * 3) + 2]);

                transform_method.transform(&mut temp_pixel);

                file.write(&[temp_pixel.red(), temp_pixel.green(), temp_pixel.blue()])?;
            }

            self.buffer.seek(SeekFrom::Current(buff_size as i64))?;
            temp_current += buff_size;
        }

        file.flush()?;

        Ok(())
    }
}

pub trait TransformImage {

    fn transform(&self, pixel: &mut Pixel);
}

struct GrayscaleTransform {}

impl TransformImage for GrayscaleTransform {

    fn transform(&self, pixel: &mut Pixel){

        pixel.grayscale();
    }
}

struct InvertTransform {}

impl TransformImage for InvertTransform {

    fn transform(&self, pixel: &mut Pixel){

        pixel.invert();
    }
}