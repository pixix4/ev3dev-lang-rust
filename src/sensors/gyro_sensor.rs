//! LEGO EV3 gyro sensor.

use super::SensorPort;
use crate::{Attribute, Device, Driver, Ev3Error, Ev3Result};

/// LEGO EV3 gyro sensor.
#[derive(Debug, Clone, Device)]
pub struct GyroSensor {
    driver: Driver,
}

impl GyroSensor {
    findable!(
        "lego-sensor",
        "lego-ev3-gyro",
        SensorPort,
        "GyroSensor",
        "in"
    );

    /// Angle
    pub const MODE_GYRO_ANG: &'static str = "GYRO-ANG";

    /// Rotational Speed
    pub const MODE_GYRO_RATE: &'static str = "GYRO-RATE";

    /// Raw sensor value ???
    pub const MODE_GYRO_FAS: &'static str = "GYRO-FAS";

    /// Angle and Rotational Speed
    pub const MODE_GYRO_G_AND_A: &'static str = "GYRO-G&A";

    /// Calibration ???
    pub const MODE_GYRO_CAL: &'static str = "GYRO-CAL";

    sensor!();

    /// Angle
    pub fn set_mode_col_ang(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_GYRO_ANG)
    }

    /// Rotational Speed
    pub fn set_mode_col_rate(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_GYRO_RATE)
    }

    /// Raw sensor value ???
    pub fn set_mode_col_fas(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_GYRO_FAS)
    }

    /// Angle and Rotational Speed
    pub fn set_mode_gyro_g_and_a(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_GYRO_G_AND_A)
    }

    /// Calibration ???
    pub fn set_mode_gyro_cal(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_GYRO_CAL)
    }
}
