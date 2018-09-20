use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::string::String;

use core::Port;

pub type AttributeResult<T> = Result<T, io::Error>;

const ROOT_PATH: &str = "/sys/class/";

pub struct Attribute {
    file: File
}

impl Attribute {
    pub fn new(class_name: &str, name: &str, attribute_name: &str) -> Option<Attribute> {
        let file = Attribute::open_file(class_name, name, attribute_name);
        if file.is_some() {
            Some(Attribute {
                file: file.unwrap(),
            })
        } else {
            None
        }
    }

    fn open_file(class_name: &str, name: &str, attribute_name: &str) -> Option<File> {
        let mut filename = String::from(ROOT_PATH);
        filename.push_str(class_name);
        filename.push_str("/");
        filename.push_str(name);
        filename.push_str("/");
        filename.push_str(attribute_name);

        let stat = fs::metadata(&filename);

        if stat.is_ok() {
            let mode = stat.unwrap().permissions().mode();

            let readable = mode & 256 == 256;
            let writeable = mode & 128 == 128;

            let file = OpenOptions::new()
                .read(readable)
                .write(writeable)
                .open(&filename);

            file.ok()
        } else {
            None
        }
    }

    pub fn get_str(&self) -> AttributeResult<String> {
        let mut value = String::new();
        (&self.file).seek(SeekFrom::Start(0))?;
        (&self.file).read_to_string(&mut value)?;
        Ok(String::from(value.trim_right()))
    }

    pub fn set_str(&self, value: String) -> AttributeResult<()> {
        (&self.file).seek(SeekFrom::Start(0))?;
        (&self.file).write_all(value.as_bytes())?;
        Ok(())
    }

    pub fn get_int(&self) -> AttributeResult<isize> {
        let value = self.get_str()?;
        Ok(value.parse::<isize>().unwrap())
    }

    pub fn set_int(&self, value: isize) -> AttributeResult<()> {
        self.set_str(value.to_string())
    }

    pub fn get_float(&self) -> AttributeResult<f32> {
        let value = self.get_str()?;
        Ok(value.parse::<f32>().unwrap())
    }

    pub fn set_float(&self, value: f32) -> AttributeResult<()> {
        self.set_str(value.to_string())
    }

    pub fn get_vec(&self) -> AttributeResult<Vec<String>> {
        let value = self.get_str()?;
        let vec = value
            .split_whitespace()
            .map(|word| word.to_string())
            .collect();
        Ok(vec)
    }
}

pub struct Driver {
    class_name: String,
    name: String,
    attributes: HashMap<String, Attribute>,
}

impl Driver {
    pub fn new(class_name: String, name: String) -> Driver {
        Driver {
            class_name,
            name,
            attributes: HashMap::new(),
        }
    }

    pub fn find_name_by_port_and_driver(class_name: &str, port: &Port, driver_name: &str) -> Option<String> {
        let port_address = port.address();

        let mut filename = String::from(ROOT_PATH);
        filename.push_str(class_name);
        let paths = fs::read_dir(filename).unwrap();

        for path in paths {
            let file_name = path.unwrap().file_name();
            let name = file_name.to_str().unwrap();

            let address = Attribute::new(class_name, name, "address").unwrap();

            if address.get_str().unwrap() == port_address {
                let driver = Attribute::new(class_name, name, "driver_name").unwrap();

                if driver.get_str().unwrap() == driver_name {
                    return Some(String::from(name));
                }
            }
        }
        None
    }

    pub fn find_name_by_port(class_name: &str, port: &Port) -> Option<String> {
        let port_address = port.address();

        let mut filename = String::from(ROOT_PATH);
        filename.push_str(class_name);
        let paths = fs::read_dir(filename).unwrap();

        for path in paths {
            let file_name = path.unwrap().file_name();
            let name = file_name.to_str().unwrap();

            let address = Attribute::new(class_name, name, "address").unwrap();

            if address.get_str().unwrap() == port_address {
                return Some(String::from(name));
            }
        }
        None
    }

    pub fn find_name_by_driver(class_name: &str, driver_name: &str) -> Option<String> {
        let mut filename = String::from(ROOT_PATH);
        filename.push_str(class_name);
        let paths = fs::read_dir(filename).unwrap();

        let mut found_name:Option<String> = None;

        for path in paths {
            let file_name = path.unwrap().file_name();
            let name = file_name.to_str().unwrap();

            let driver = Attribute::new(class_name, name, "driver_name").unwrap();

            if driver.get_str().unwrap() == driver_name {
                match found_name {
                    Some(_) => return None,
                    None => found_name = Some(String::from(name)),
                }
            }
        }
        return found_name;
    }

    pub fn get_attribute(&mut self, attribute_name: &str) -> &Attribute {
        if !self.attributes.contains_key(attribute_name) {
            if let Some(v) = Attribute::new(self.class_name.as_ref(), self.name.as_ref(), attribute_name) {
                self.attributes.insert(String::from(attribute_name), v);
            };
        };

        return self.attributes.get(attribute_name).unwrap();
    }
}