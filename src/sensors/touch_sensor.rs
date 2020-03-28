//! Touch Sensor

use super::SensorPort;
use crate::{Attribute, Device, Driver, Ev3Result};

/// Touch Sensor
#[derive(Debug, Clone, Device)]
pub struct TouchSensor {
    driver: Driver,
}

impl TouchSensor {
    findable!("lego-sensor", "lego-ev3-touch", SensorPort);

    /// Button state
    pub const MODE_TOUCH: &'static str = "TOUCH";

    sensor!();

    /// A boolean indicating whether the current touch sensor is being pressed.
    pub fn get_pressed_state(&self) -> Ev3Result<bool> {
        Ok(self.get_value0()? != 0)
    }
}
