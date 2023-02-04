//! Helper struct that manages attributes.
//! It creates an `Attribute` instance if it does not exists or uses a cached one.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::fs;
use std::path::Path;
use std::string::String;

use crate::utils::OrErr;
use crate::{Attribute, Ev3Error, Ev3Result, Port};

/// The driver path `/sys/class/`.
#[cfg(not(feature = "override-driver-path"))]
pub const DRIVER_PATH: &str = "/sys/class/";

/// The driver path that was set with the env variable `EV3DEV_DRIVER_PATH` (default value: `/sys/class/`).
#[cfg(feature = "override-driver-path")]
pub const DRIVER_PATH: &str = get_driver_path();

#[cfg(feature = "override-driver-path")]
const fn get_driver_path() -> &'static str {
    let path = std::option_env!("EV3DEV_DRIVER_PATH");
    if let Some(path) = path {
        path
    } else {
        "/sys/class/"
    }
}

/// Helper struct that manages attributes.
/// It creates an `Attribute` instance if it does not exists or uses a cached one.
#[derive(Clone)]
pub struct Driver {
    class_name: String,
    name: String,
    attributes: RefCell<HashMap<String, Attribute>>,
}

impl Driver {
    /// Returns a new `Driver`.
    /// All attributes created by this driver will use the path `/sys/class/{class_name}/{name}`.
    pub fn new(class_name: &str, name: &str) -> Driver {
        Driver {
            class_name: class_name.to_owned(),
            name: name.to_owned(),
            attributes: RefCell::new(HashMap::new()),
        }
    }

    /// Returns the name of the device with the given `class_name`, `driver_name` and at the given `port`.
    ///
    /// Returns `Ev3Error::NotFound` if no such device exists.
    pub fn find_name_by_port_and_driver(
        class_name: &str,
        port: &dyn Port,
        driver_name_vec: &[&str],
    ) -> Ev3Result<String> {
        let port_address = port.address();

        let paths = fs::read_dir(Path::new(DRIVER_PATH).join(class_name))?;

        for path in paths {
            let file_name = path?.file_name();
            let name = file_name.to_str().or_err()?;

            let address = Attribute::from_sys_class(class_name, name, "address")?;

            if address.get::<String>()?.contains(&port_address) {
                let driver = Attribute::from_sys_class(class_name, name, "driver_name")?;
                let driver_name = driver.get::<String>()?;
                if driver_name_vec.iter().any(|n| &driver_name == n) {
                    return Ok(name.to_owned());
                }
            }
        }

        Err(Ev3Error::NotConnected {
            device: format!("{driver_name_vec:?}"),
            port: Some(port_address),
        })
    }

    /// Returns the name of the device with the given `class_name`.
    ///
    /// Returns `Ev3Error::NotFound` if no such device exists.
    /// Returns `Ev3Error::MultipleMatches` if more then one matching device exists.
    pub fn find_name_by_driver(class_name: &str, driver_name_vec: &[&str]) -> Ev3Result<String> {
        let mut names = Driver::find_names_by_driver(class_name, driver_name_vec)?;

        match names.len() {
            0 => Err(Ev3Error::NotConnected {
                device: format!("{driver_name_vec:?}"),
                port: None,
            }),
            1 => Ok(names
                .pop()
                .expect("Name vector should contains exactly one element")),
            _ => Err(Ev3Error::MultipleMatches {
                device: format!("{driver_name_vec:?}"),
                ports: names,
            }),
        }
    }

    /// Returns the names of the devices with the given `class_name`.
    pub fn find_names_by_driver(
        class_name: &str,
        driver_name_vec: &[&str],
    ) -> Ev3Result<Vec<String>> {
        let paths = fs::read_dir(Path::new(DRIVER_PATH).join(class_name))?;

        let mut found_names = Vec::new();
        for path in paths {
            let file_name = path?.file_name();
            let name = file_name.to_str().or_err()?;

            let driver = Attribute::from_sys_class(class_name, name, "driver_name")?;

            let driver_name = driver.get::<String>()?;
            if driver_name_vec.iter().any(|n| &driver_name == n) {
                found_names.push(name.to_owned());
            }
        }

        Ok(found_names)
    }

    /// Return the `Attribute` wrapper for the given `attribute_name`.
    /// Creates a new one if it does not exist.
    pub fn get_attribute(&self, attribute_name: &str) -> Attribute {
        let mut attributes = self.attributes.borrow_mut();

        if !attributes.contains_key(attribute_name) {
            if let Ok(v) = Attribute::from_sys_class(
                self.class_name.as_ref(),
                self.name.as_ref(),
                attribute_name,
            ) {
                attributes.insert(attribute_name.to_owned(), v);
            };
        };

        attributes
            .get(attribute_name)
            .expect("Internal error in the attribute map")
            .clone()
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
