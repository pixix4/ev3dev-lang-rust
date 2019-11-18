use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::fs;
use std::string::String;

use crate::{utils::OrErr, Attribute, Ev3Error, Ev3Result, Port};

pub const ROOT_PATH: &str = "/sys/class/";

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
    ) -> Ev3Result<String> {
        let port_address = port.address();

        let mut filename = ROOT_PATH.to_owned();
        filename.push_str(class_name);
        let paths = fs::read_dir(filename)?;

        for path in paths {
            let file_name = path?.file_name();
            let name = file_name.to_str().or_err()?;

            let address = Attribute::new(class_name, name, "address")?;

            if address.get::<String>()?.contains(port_address.as_str()) {
                let driver = Attribute::new(class_name, name, "driver_name")?;

                if driver.get::<String>()? == driver_name {
                    return Ok(name.to_owned());
                }
            }
        }

        Err(Ev3Error::NotFound)
    }

    pub fn find_name_by_port(class_name: &str, port: &dyn Port) -> Ev3Result<String> {
        let port_address = port.address();

        let mut filename = ROOT_PATH.to_owned();
        filename.push_str(class_name);
        let paths = fs::read_dir(filename)?;

        for path in paths {
            let file_name = path?.file_name();
            let name = file_name.to_str().or_err()?;

            let address = Attribute::new(class_name, name, "address")?;

            if address.get::<String>()?.contains(port_address.as_str()) {
                return Ok(name.to_owned());
            }
        }

        Err(Ev3Error::NotFound)
    }

    pub fn find_name_by_driver(class_name: &str, driver_name: &str) -> Ev3Result<String> {
        let mut filename = ROOT_PATH.to_owned();
        filename.push_str(class_name);
        let paths = fs::read_dir(filename)?;

        let mut found_name: Option<String> = None;

        for path in paths {
            let file_name = path?.file_name();
            let name = file_name.to_str().ok_or(Ev3Error::InternalError {
                msg: "Cannot convert filename to str".to_owned(),
            })?;

            let driver = Attribute::new(class_name, name, "driver_name")?;

            if driver.get::<String>()? == driver_name {
                match found_name {
                    Some(_) => return Err(Ev3Error::MultipleMatches),
                    None => found_name = Some(name.to_owned()),
                }
            }
        }

        found_name.ok_or(Ev3Error::NotFound)
    }

    pub fn find_names_by_driver(class_name: &str, driver_name: &str) -> Ev3Result<Vec<String>> {
        let mut filename = ROOT_PATH.to_owned();
        filename.push_str(class_name);
        let paths = fs::read_dir(filename)?;

        let mut found_names = Vec::new();
        for path in paths {
            let file_name = path?.file_name();
            let name = file_name.to_str().or_err()?;

            let driver = Attribute::new(class_name, name, "driver_name")?;

            if driver.get::<String>()? == driver_name {
                found_names.push(name.to_owned());
            }
        }

        Ok(found_names)
    }

    pub fn get_attribute(&self, attribute_name: &str) -> Attribute {
        let mut attributes = self.attributes.borrow_mut();

        if !attributes.contains_key(attribute_name) {
            if let Ok(v) =
                Attribute::new(self.class_name.as_ref(), self.name.as_ref(), attribute_name)
            {
                attributes.insert(attribute_name.to_owned(), v);
            };
        };

        attributes
            .get(attribute_name)
            .expect("Internal error in the attribute map")
            .clone()
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
