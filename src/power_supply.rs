//! An interface to read data from the system’s power_supply class.
//! Uses the built-in legoev3-battery if none is specified.

use std::{fs, path::Path};

use crate::driver::DRIVER_PATH;
use crate::utils::OrErr;
use crate::{Attribute, Device, Driver, Ev3Error, Ev3Result};

/// An interface to read data from the system’s power_supply class.
/// Uses the built-in legoev3-battery if none is specified.
#[derive(Debug, Clone, Device)]
pub struct PowerSupply {
    driver: Driver,
}

impl PowerSupply {
    /// Create a new instance of `PowerSupply`.
    pub fn new() -> Ev3Result<PowerSupply> {
        let paths = fs::read_dir(Path::new(DRIVER_PATH).join("power_supply"))?;

        for path in paths {
            let file_name = path?.file_name();
            let name = file_name.to_str().or_err()?;

            if name.contains("ev3-battery") {
                return Ok(PowerSupply {
                    driver: Driver::new("power_supply", name),
                });
            }
        }

        Err(Ev3Error::NotConnected {
            device: "power_supply".to_owned(),
            port: None,
        })
    }

    /// Returns the battery current in microamps
    pub fn get_current_now(&self) -> Ev3Result<i32> {
        self.get_attribute("current_now").get()
    }

    /// Always returns System.
    pub fn get_scope(&self) -> Ev3Result<String> {
        self.get_attribute("zscope").get()
    }

    /// Returns Unknown or Li-ion depending on if the rechargeable battery is present.
    pub fn get_technology(&self) -> Ev3Result<String> {
        self.get_attribute("technology").get()
    }

    /// Always returns Battery.
    pub fn get_type(&self) -> Ev3Result<String> {
        self.get_attribute("type").get()
    }

    /// Returns the nominal “full” battery voltage. The value returned depends on technology.
    pub fn get_voltage_max_design(&self) -> Ev3Result<i32> {
        self.get_attribute("voltage_max_design").get()
    }

    /// Returns the nominal “empty” battery voltage. The value returned depends on technology.
    pub fn get_voltage_min_design(&self) -> Ev3Result<i32> {
        self.get_attribute("voltage_min_design").get()
    }

    /// Returns the battery voltage in microvolts.
    pub fn get_voltage_now(&self) -> Ev3Result<i32> {
        self.get_attribute("voltage_now").get()
    }
}
