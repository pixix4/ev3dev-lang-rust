//! HiTechnic EV3 / NXT Compass Sensor. (<https://www.generationrobots.com/en/401186-hitechnic-compass-sensor-for-lego-mindstorms-nxt-and-ev3.html>)

use super::{Sensor, SensorPort};
use crate::{Attribute, Device, Driver, Ev3Error, Ev3Result};

/// HiTechnic EV3 / NXT Compass Sensor.
#[derive(Debug, Clone, Device, Sensor)]
pub struct CompassSensor {
    driver: Driver,
    origin: i32, // zero point
}

impl CompassSensor {
    fn new(driver: Driver) -> Self {
        Self { driver, origin: 0 }
    }

    findable!(
        "lego-sensor",
        ["ht-nxt-compass"],
        SensorPort,
        "Compass",
        "in"
    );

    /// Command for starting the calibration
    pub const COMMAND_START_CALIBRATION: &'static str = "BEGIN-CAL";

    /// Command for stopping the calibration
    pub const COMMAND_STOP_CALIBRATION: &'static str = "END-CAL";

    // Sensor only have one mode (COMPASS), so setting the mode is not necessary

    /// gets rotation (in degree) from the compass sensor
    pub fn get_rotation(&self) -> Ev3Result<i32> {
        self.get_value0()
    }

    /// sets the origin
    pub fn set_zero(&mut self) -> Ev3Result<()> {
        self.origin = self.get_rotation()?;
        Ok(())
    }

    /// calculates the rotation to the origin / the zero point
    pub fn get_relative_rotation(&self) -> Ev3Result<i32> {
        let pos = self.get_rotation()?;
        let mut rel_rot = pos - self.origin;
        if rel_rot < 0 {
            rel_rot += 360;
        }
        Ok(rel_rot)
    }

    /// calibration:
    /// start the calibration by start_calibration()
    /// turn the robot 360 degrees
    /// end the calibration by stop_calibration()
    /// attention: if calibration has not finished, the get_rotation method always returns -258

    /// starts the calibration
    pub fn start_calibration(&self) -> Ev3Result<()> {
        self.set_command(Self::COMMAND_START_CALIBRATION)
    }

    /// stops the calibration
    pub fn stop_calibration(&self) -> Ev3Result<()> {
        self.set_command(Self::COMMAND_STOP_CALIBRATION)
    }
}
