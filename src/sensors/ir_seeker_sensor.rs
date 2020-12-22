//! LEGO EV3 light sensor.

use super::SensorPort;
use crate::{Attribute, Device, Driver, Ev3Error, Ev3Result};

/// LEGO EV3 light sensor.
#[derive(Debug, Clone, Device)]
pub struct IrSeekerSensor {
    driver: Driver,
}

impl IrSeekerSensor {

    fn new(driver: Driver) -> Self {
        Self {
            driver,
        }
    }

    findable!(
        "lego-sensor",
        "ht-nxt-ir-seek-v2",
        SensorPort,
        "IrSeeker",
        "in"
    );

    /// Sensor mode alternating current -> filters the infrared signal of the hitechnic ball -> only shows direction
    pub const AC: &'static str = "AC";

    /// Sensor mode Direct current -> reacts on all infrared signals, sun infrared signal included -> only shows direction
    pub const DC: &'static str = "DC";

    /// Sensor mode alternating current -> shows direction (value0) and values of each of the five sensors
    pub const AC_ALL: &'static str = "AC-ALL";

    /// Sensor mode Direct current -> shows direction (value0) and values of each of the five sensors
    pub const DC_ALL: &'static str = "DC-ALL";

    sensor!();

    /// sets mode to AC
    pub fn set_mode_ac(&self) -> Ev3Result<()> {
        self.set_mode(Self::AC)
    }

    /// sets mode to DC
    pub fn set_mode_dc(&self) -> Ev3Result<()> {
        self.set_mode(Self::DC)
    }

    /// sets mode to AC_ALL
    pub fn set_mode_ac_all(&self) -> Ev3Result<()> {
        self.set_mode(Self::AC_ALL)
    }

    /// sets mode to DC_ALL
    pub fn set_mode_dc_all(&self) -> Ev3Result<()> {
        self.set_mode(Self::DC_ALL)
    }

    /// gets direction of incoming ir light (calculated by the sensor)
    pub fn get_ir_direction(&self) -> Ev3Result<i32> {
        self.get_value0()
    }

    /// gets the values of the five sensors of the HiTechnic IR Seeker (only works if dc_all or ac_all mode is activated)
    pub fn get_raw_values(&self) -> Ev3Result<[i32; 5]> {
        let val1 = self.get_value1()?;
        let val2 = self.get_value2()?;
        let val3 = self.get_value3()?;
        let val4 = self.get_value4()?;
        let val5 = self.get_value5()?;
        Ok([val1, val2, val3, val4, val5])
    }

}
