//! Touch Sensor

use super::{Sensor, SensorPort};
use crate::{Attribute, Device, Driver, Ev3Error, Ev3Result};

/// Touch Sensor
#[derive(Debug, Clone, Device, Sensor)]
pub struct TouchSensor {
    driver: Driver,
}

impl TouchSensor {
    fn new(driver: Driver) -> Self {
        Self { driver }
    }

    findable!(
        "lego-sensor",
        ["lego-ev3-touch", "lego-nxt-touch"],
        SensorPort,
        "TouchSensor",
        "in"
    );

    /// Button state
    pub const MODE_TOUCH: &'static str = "TOUCH";

    /// A boolean indicating whether the current touch sensor is being pressed.
    pub fn get_pressed_state(&self) -> Ev3Result<bool> {
        Ok(self.get_value0()? != 0)
    }
}
