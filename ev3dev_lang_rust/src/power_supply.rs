use std::fs;

use core::Device;
use driver::{Attribute, AttributeResult, Driver};

#[derive(Debug, Clone, Device)]
pub struct PowerSupply {
    driver: Driver,
}

impl PowerSupply {
    pub fn new() -> Option<PowerSupply> {
        let paths = fs::read_dir("/sys/class/power_supply").unwrap();

        for path in paths {
            let file_name = path.unwrap().file_name();
            let name = file_name.to_str().unwrap();

            if name.contains("ev3-battery") {
                return Some(PowerSupply {
                    driver: Driver::new("power_supply", name),
                });
            }
        }
        None
    }

    /// Returns the battery current in microamps
    pub fn get_current_now(&self) -> AttributeResult<i32> {
        self.get_attribute("current_now").get()
    }

    ///	Always returns System.
    pub fn get_scope(&self) -> AttributeResult<String> {
        self.get_attribute("zscope").get()
    }

    ///	Returns Unknown or Li-ion depending on if the rechargeable battery is present.
    pub fn get_technology(&self) -> AttributeResult<String> {
        self.get_attribute("technology").get()
    }

    /// Always returns Battery.
    pub fn get_type(&self) -> AttributeResult<String> {
        self.get_attribute("type").get()
    }

    ///	Returns the nominal “full” battery voltage. The value returned depends on technology.
    pub fn get_voltage_max_design(&self) -> AttributeResult<i32> {
        self.get_attribute("voltage_max_design").get()
    }

    /// Returns the nominal “empty” battery voltage. The value returned depends on technology.
    pub fn get_voltage_min_design(&self) -> AttributeResult<i32> {
        self.get_attribute("voltage_min_design").get()
    }

    /// Returns the battery voltage in microvolts.
    pub fn get_voltage_now(&self) -> AttributeResult<i32> {
        self.get_attribute("voltage_now").get()
    }
}
