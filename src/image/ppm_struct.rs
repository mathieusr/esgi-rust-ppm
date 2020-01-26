pub trait PpmValue {

    fn set_value(&mut self, value: String);
}

pub struct PpmStringElement {
    pub name: String,
    pub value: String
}

impl PpmValue for PpmStringElement {

    fn set_value(&mut self, value: String) {

        self.value = value;
    }
}

pub struct PpmIntElement {
    pub name: String,
    pub value: usize
}

impl PpmValue for PpmIntElement {

    fn set_value(&mut self, value: String) {

        let new_value = value.parse::<usize>();

        match new_value {

            Ok(new_cert_value) => self.value = new_cert_value,
            Err(_e) => println!("Error when parsing your int ppm param")
        }
    }
}
