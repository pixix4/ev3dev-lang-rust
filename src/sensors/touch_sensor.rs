use crate::sensors::Sensor;
use crate::{Attribute, Device, Driver, Ev3Result, Findable};

/// Button state
pub const MODE_TOUCH: &str = "TOUCH";

/// Touch Sensor
#[derive(Debug, Clone, Device, Sensor, Findable)]
#[class_name = "lego-sensor"]
#[driver_name = "lego-ev3-touch"]
#[port = "crate::sensors::SensorPort"]
pub struct TouchSensor {
    driver: Driver,
}

impl TouchSensor {
    /// A boolean indicating whether the current touch sensor is being pressed.
    pub fn get_pressed_state(&self) -> Ev3Result<bool> {
        Ok(self.get_value0()? != 0)
    }
}
