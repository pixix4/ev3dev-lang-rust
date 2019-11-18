use crate::sensors::{Sensor, SensorPort};
use crate::{Attribute, Device, Driver, Ev3Result};

/// Button state
pub const MODE_TOUCH: &str = "TOUCH";

#[derive(Debug, Clone, Device)]
pub struct TouchSensor {
    driver: Driver,
}

impl Sensor for TouchSensor {}

impl TouchSensor {
    /// Try to get a `TouchSensor` on the given port. Returns `None` if port is not used or another device is connected.
    pub fn new(port: SensorPort) -> Ev3Result<TouchSensor> {
        let name = Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-touch")?;

        Ok(TouchSensor {
            driver: Driver::new("lego-sensor", &name),
        })
    }

    /// Try to find a `TouchSensor`. Only returns a sensor if their is exactly one connected, `None` otherwise.
    pub fn find() -> Ev3Result<TouchSensor> {
        let name = Driver::find_name_by_driver("lego-sensor", "lego-ev3-touch")?;

        Ok(TouchSensor {
            driver: Driver::new("lego-sensor", &name),
        })
    }

    pub fn get_pressed_state(&self) -> Ev3Result<bool> {
        Ok(self.get_value0()? != 0)
    }
}
