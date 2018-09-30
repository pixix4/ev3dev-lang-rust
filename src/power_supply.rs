use driver::Driver;
use driver::Attribute;
use driver::AttributeResult;
use std::fs;

pub struct PowerSupply {
    driver: Driver
}

impl PowerSupply {
    pub fn new() -> Option<PowerSupply> {
        let paths = fs::read_dir("/sys/class/power_supply").unwrap();

        for path in paths {
            let file_name = path.unwrap().file_name();
            let name = String::from(file_name.to_str().unwrap());

            if name.contains("ev3-battery") {
                return Some(PowerSupply {
                    driver: Driver::new(String::from("power_supply"), name)
                });
            }
        }
        None
    }


    fn get_attribute(&mut self, name: &str) -> &Attribute {
        self.driver.get_attribute(name)
    }

    /// Returns the battery current in microamps
    pub fn get_current_now(&mut self) -> AttributeResult<isize> {
        self.get_attribute("current_now").get_int()
    }

    ///	Always returns System.
    pub fn get_scope(&mut self) -> AttributeResult<String> {
        self.get_attribute("scope").get_str()
    }

    ///	Returns Unknown or Li-ion depending on if the rechargeable battery is present.
    pub fn get_technology(&mut self) -> AttributeResult<String> {
        self.get_attribute("technology").get_str()
    }

    /// Always returns Battery.
    pub fn get_type(&mut self) -> AttributeResult<String> {
        self.get_attribute("type").get_str()
    }

    ///	Returns the nominal “full” battery voltage. The value returned depends on technology.
    pub fn get_voltage_max_design(&mut self) -> AttributeResult<isize> {
        self.get_attribute("voltage_max_design").get_int()
    }

    /// Returns the nominal “empty” battery voltage. The value returned depends on technology.
    pub fn get_voltage_min_design(&mut self) -> AttributeResult<isize> {
        self.get_attribute("voltage_min_design").get_int()
    }

    /// Returns the battery voltage in microvolts.
    pub fn get_voltage_now(&mut self) -> AttributeResult<isize> {
        self.get_attribute("voltage_now").get_int()
    }
}