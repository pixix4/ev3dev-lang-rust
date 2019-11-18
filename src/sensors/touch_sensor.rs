use crate::sensors::{Sensor, SensorPort};
use core::Device;
use driver::Attribute;
use driver::AttributeResult;
use driver::Driver;

/// Button state
pub const MODE_TOUCH: &str = "TOUCH";

#[derive(Debug, Clone)]
pub struct TouchSensor {
    driver: Driver,
}

impl Sensor for TouchSensor {}

impl Device for TouchSensor {
    fn get_attribute(&self, name: &str) -> Attribute {
        self.driver.get_attribute(name)
    }
}

impl TouchSensor {
    /// Try to get a `TouchSensor` on the given port. Returns `None` if port is not used or another device is connected.
    pub fn new(port: SensorPort) -> Option<TouchSensor> {
        if let Some(name) =
            Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-touch")
        {
            return Some(TouchSensor {
                driver: Driver::new("lego-sensor", &name),
            });
        }

        None
    }

    /// Try to find a `TouchSensor`. Only returns a sensor if their is exactly one connected, `None` otherwise.
    pub fn find() -> Option<TouchSensor> {
        if let Some(name) = Driver::find_name_by_driver("lego-sensor", "lego-ev3-touch") {
            return Some(TouchSensor {
                driver: Driver::new("lego-sensor", &name),
            });
        }

        None
    }

    pub fn get_pressed_state(&self) -> AttributeResult<bool> {
        Ok(self.get_value0()? != 0)
    }
}
