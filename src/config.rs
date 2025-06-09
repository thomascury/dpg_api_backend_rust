use core::fmt;

pub struct Configuration {
    word_count: u8,
    separator: String,
    capitalize: bool,
    use_lower: bool,
    use_upper: bool,
    use_digit: bool,
    use_special: bool,
    passphrase_minimum_length: u8,
    pub field_list: Vec<&'static str>,
}

impl Configuration {
    pub fn set_word_count (&mut self, opt: Option<&str>) {
        match opt {
            Some(value) => {
                self.word_count = value.to_string().parse::<u8>().unwrap();
                debug!("value for word_count set to '{}'", self.word_count)
            },
            None => (),
        }
    }
    pub fn set_separator (&mut self, opt: Option<&str>) {
        match opt {
            Some(value) => {
                self.separator = value.to_string();
                debug!("value for separator set to '{}'", self.separator)
            },
            None => (),
        }
    }
    pub fn set_capitalize (&mut self, opt: Option<&str>) {
        match opt {
            Some(value) => {
                self.capitalize = value.to_string().parse::<bool>().unwrap();
                debug!("value for capitalize set to '{}'", self.capitalize)
            },
            None => (),
        }
    }
    pub fn set_use_lower (&mut self, opt: Option<&str>) {
        match opt {
            Some(value) => {
                self.use_lower = value.to_string().parse::<bool>().unwrap();
                debug!("value for use_lower set to '{}'", self.use_lower)
            },
            None => (),
        }
    }
    pub fn set_use_upper (&mut self, opt: Option<&str>) {
        match opt {
            Some(value) => {
                self.use_upper = value.to_string().parse::<bool>().unwrap();
                debug!("value for use_upper set to '{}'", self.use_upper)
            },
            None => (),
        }
    }
    pub fn set_use_digit (&mut self, opt: Option<&str>) {
        match opt {
            Some(value) => {
                self.use_digit = value.to_string().parse::<bool>().unwrap();
                debug!("value for use_digit set to '{}'", self.use_digit)
            },
            None => (),
        }
    }
    pub fn set_use_special (&mut self, opt: Option<&str>) {
        match opt {
            Some(value) => {
                self.use_special = value.to_string().parse::<bool>().unwrap();
                debug!("value for use_special set to '{}'", self.use_special)
            },
            None => (),
        }
    }
    pub fn set_passphrase_minimum_length (&mut self, opt: Option<&str>) {
        match opt {
            Some(value) => {
                self.passphrase_minimum_length = value.to_string().parse::<u8>().unwrap();
                debug!("value for passphrase_minimum_length set to '{}'", self.passphrase_minimum_length)
            },
            None => (),
        }
    }
    pub fn get_word_count(&self) -> u8 {
        return self.word_count;
    }
    pub fn get_separator(&self) -> &String {
        return &self.separator;
    }
    pub fn get_capitalize(&self) -> bool {
        return self.capitalize;
    }
    pub fn get_use_lower(&self) -> bool {
        return self.use_lower;
    }
    pub fn get_use_upper(&self) -> bool {
        return self.use_upper;
    }
    pub fn get_use_digit(&self) -> bool {
        return self.use_digit;
    }
    pub fn get_use_special(&self) -> bool {
        return self.use_special;
    }
    pub fn get_passphrase_minimum_length(&self) -> u8 {
        return self.passphrase_minimum_length;
    }

    pub fn init() -> Configuration {
        let field_list_vec = Vec::from([
            "word_count",
            "separator",
            "capitalize",
            "use_lower",
            "use_upper",
            "use_digit",
            "use_special",
            "passphrase_minimum_length",
        ]);
        let config = Configuration {
            word_count: 5,
            separator: " ".to_string(),
            capitalize: true,
            use_lower: true,
            use_upper: true,
            use_digit: true,
            use_special: true,
            passphrase_minimum_length: 14,
            field_list: field_list_vec,
        };
        return config;
    }
}

impl fmt::Display for Configuration {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Configuration: ")?;
        for field in self.field_list.clone() {
            match field {
                "word_count" => write!(f, "word_count: {}, ", self.get_word_count())?,
                "separator" => write!(f, "separator: '{}', ", self.get_separator())?,
                "capitalize" => write!(f, "capitalize: {}, ", self.get_capitalize())?,
                "use_lower" => write!(f, "use_lower: {}, ", self.get_use_lower())?,
                "use_upper" => write!(f, "use_upper: {}, ", self.get_use_upper())?,
                "use_digit" => write!(f, "use_digit: {}, ", self.get_use_digit())?,
                "use_special" => write!(f, "use_special: {}", self.get_use_special())?,
                "passphrase_minimum_length" => write!(f, "passphrase_minimum_length: {}", self.get_passphrase_minimum_length())?,
                _ => ()
            }
        }
        Ok(())
    }
}