use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl fmt::Display for Pixel {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "Pixel: ({},{},{})", self.r, self.g, self.b)
    }
}

impl Pixel {

    pub fn new(red: u8, green: u8, blue: u8) -> Pixel {

        Pixel{r: red, g: green, b: blue}
    }

    pub fn red(&self) -> u8 {

        self.r
    } 

    pub fn green(&self) -> u8 {

        self.g
    } 

    pub fn blue(&self) -> u8 {

        self.b
    }

    pub fn invert(&mut self) {

        self.r = !self.r;
        self.g = !self.g;
        self.b = !self.b;
    }

    pub fn grayscale(&mut self){

        let average = (self.r / 3) + (self.g / 3) + (self.b / 3) + (((self.r % 2) + (self.g % 2) + (self.b % 2)) / 2);

        self.r = average;
        self.g = average;
        self.b = average;
    }
}

#[cfg(test)]
mod tests {

    use super::Pixel;

    fn get_pixel() -> Pixel{

        Pixel::new(1, 2, 3)
    }

    #[test]
    fn check_pixel_red() {

        assert_eq!(get_pixel().red(), 1);
    }

    #[test]
    fn check_pixel_green() {

        assert_eq!(get_pixel().green(), 2);
    }

    #[test]
    fn check_pixel_blue() {

        assert_eq!(get_pixel().blue(), 3);
    }

    #[test]
    fn check_display() {

        assert_eq!(format!("{}", get_pixel()), "Pixel: (1,2,3)");
    }

    #[test]
    fn check_invert() {

        let mut temp = get_pixel();
        temp.invert();

        assert_eq!(temp.red(), 255 - 1);
        assert_eq!(temp.green(), 255 - 2);
        assert_eq!(temp.blue(), 255 - 3);
    }

    #[test]
    fn check_equality() {

        let temp = get_pixel();
        let mut temp1 = get_pixel();

        assert!(temp == temp1);

        temp1.r = 255;

        assert_eq!(temp == temp1, false);
    }

    #[test]
    fn check_grascayle() {

        let mut temp = get_pixel();
        temp.grayscale();

        assert_eq!(temp.r, 2);
        assert_eq!(temp.g, 2);
        assert_eq!(temp.b, 2);
    }
}