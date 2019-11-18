use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fs::{self, File, OpenOptions};
use std::io::{Error as IoError, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::PermissionsExt;
use std::rc::Rc;
use std::string::String;

use core::Port;

pub type AttributeResult<T> = Result<T, Ev3Error>;

#[derive(Debug)]
pub enum Ev3Error {
    IoError { msg: String },
    ParseError { msg: String },
}
impl From<IoError> for Ev3Error {
    fn from(err: IoError) -> Ev3Error {
        Ev3Error::IoError {
            msg: err.description().to_owned(),
        }
    }
}

const ROOT_PATH: &str = "/sys/class/";

#[derive(Clone)]
pub struct Attribute {
    file: Rc<RefCell<File>>,
}

impl Attribute {
    pub fn new(class_name: &str, name: &str, attribute_name: &str) -> Option<Attribute> {
        let file = Attribute::open_file(class_name, name, attribute_name);
        if let Some(file) = file {
            Some(Attribute {
                file: Rc::new(RefCell::new(file)),
            })
        } else {
            None
        }
    }

    fn open_file(class_name: &str, name: &str, attribute_name: &str) -> Option<File> {
        let mut filename = ROOT_PATH.to_owned();
        filename.push_str(class_name);
        filename.push_str("/");
        filename.push_str(name);
        filename.push_str("/");
        filename.push_str(attribute_name);

        // println!("Open file: {}", &filename);

        let stat = fs::metadata(&filename);

        if let Ok(stat) = stat {
            let mode = stat.permissions().mode();

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

    fn get_str(&self) -> AttributeResult<String> {
        let mut value = String::new();
        let mut file = self.file.borrow_mut();
        file.seek(SeekFrom::Start(0))?;
        file.read_to_string(&mut value)?;
        Ok(value.trim_end().to_owned())
    }

    fn set_str(&self, value: &str) -> AttributeResult<()> {
        let mut file = self.file.borrow_mut();
        file.seek(SeekFrom::Start(0))?;
        file.write_all(value.as_bytes())?;
        Ok(())
    }

    pub fn get<T>(&self) -> AttributeResult<T>
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: Error,
    {
        let value = self.get_str()?;
        match value.parse::<T>() {
            Ok(value) => Ok(value),
            Err(e) => Err(Ev3Error::ParseError {
                msg: e.description().to_owned(),
            }),
        }
    }

    pub fn set<T>(&self, value: T) -> AttributeResult<()>
    where
        T: std::string::ToString,
    {
        self.set_str(&value.to_string())
    }

    #[inline]
    pub fn set_str_slice(&self, value: &str) -> AttributeResult<()> {
        self.set_str(value)
    }

    pub fn get_vec(&self) -> AttributeResult<Vec<String>> {
        let value = self.get_str()?;
        let vec = value
            .split_whitespace()
            .map(|word| word.to_owned())
            .collect();
        Ok(vec)
    }
}

pub struct Driver {
    class_name: String,
    name: String,
    attributes: RefCell<HashMap<String, Attribute>>,
}

impl Driver {
    pub fn new(class_name: &str, name: &str) -> Driver {
        Driver {
            class_name: class_name.to_owned(),
            name: name.to_owned(),
            attributes: RefCell::new(HashMap::new()),
        }
    }

    pub fn find_name_by_port_and_driver(
        class_name: &str,
        port: &dyn Port,
        driver_name: &str,
    ) -> Option<String> {
        let port_address = port.address();

        let mut filename = ROOT_PATH.to_owned();
        filename.push_str(class_name);
        let paths = fs::read_dir(filename).unwrap();

        for path in paths {
            let file_name = path.unwrap().file_name();
            let name = file_name.to_str().unwrap();

            let address = Attribute::new(class_name, name, "address").unwrap();

            if address
                .get::<String>()
                .unwrap()
                .contains(port_address.as_str())
            {
                let driver = Attribute::new(class_name, name, "driver_name").unwrap();

                if driver.get::<String>().unwrap() == driver_name {
                    return Some(name.to_owned());
                }
            }
        }
        None
    }

    pub fn find_name_by_port(class_name: &str, port: &dyn Port) -> Option<String> {
        let port_address = port.address();

        let mut filename = ROOT_PATH.to_owned();
        filename.push_str(class_name);
        let paths = fs::read_dir(filename).unwrap();

        for path in paths {
            let file_name = path.unwrap().file_name();
            let name = file_name.to_str().unwrap();

            let address = Attribute::new(class_name, name, "address").unwrap();

            if address
                .get::<String>()
                .unwrap()
                .contains(port_address.as_str())
            {
                return Some(name.to_owned());
            }
        }
        None
    }

    pub fn find_name_by_driver(class_name: &str, driver_name: &str) -> Option<String> {
        let mut filename = ROOT_PATH.to_owned();
        filename.push_str(class_name);
        let paths = fs::read_dir(filename).unwrap();

        let mut found_name: Option<String> = None;

        for path in paths {
            let file_name = path.unwrap().file_name();
            let name = file_name.to_str().unwrap();

            let driver = Attribute::new(class_name, name, "driver_name").unwrap();

            if driver.get::<String>().unwrap() == driver_name {
                match found_name {
                    Some(_) => return None,
                    None => found_name = Some(name.to_owned()),
                }
            }
        }

        found_name
    }

    pub fn find_names_by_driver(class_name: &str, driver_name: &str) -> Vec<String> {
        let mut filename = ROOT_PATH.to_owned();
        filename.push_str(class_name);
        let paths = fs::read_dir(filename).unwrap();

        let mut found_names = Vec::new();
        for path in paths {
            let file_name = path.unwrap().file_name();
            let name = file_name.to_str().unwrap();

            let driver = Attribute::new(class_name, name, "driver_name").unwrap();

            if driver.get::<String>().unwrap() == driver_name {
                found_names.push(name.to_owned());
            }
        }

        found_names
    }

    pub fn get_attribute(&self, attribute_name: &str) -> Attribute {
        let mut attributes = self.attributes.borrow_mut();

        if !attributes.contains_key(attribute_name) {
            if let Some(v) =
                Attribute::new(self.class_name.as_ref(), self.name.as_ref(), attribute_name)
            {
                attributes.insert(attribute_name.to_owned(), v);
            };
        };

        attributes.get(attribute_name).unwrap().clone()
    }
}

impl Clone for Driver {
    fn clone(&self) -> Self {
        Driver {
            class_name: self.class_name.clone(),
            name: self.name.clone(),
            attributes: RefCell::new(HashMap::new()),
        }
    }
}

impl Debug for Driver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Driver {{ class_name: {}, name: {} }}",
            self.class_name, self.name
        )
    }
}
