//! LEGO EV3 gyro sensor.

use crate::sensors::Sensor;
use crate::{Attribute, Device, Driver, Ev3Result, Findable};

/// Angle
pub const MODE_GYRO_ANG: &str = "GYRO-ANG";

/// Rotational Speed
pub const MODE_GYRO_RATE: &str = "GYRO-RATE";

/// Raw sensor value ???
pub const MODE_GYRO_FAS: &str = "GYRO-FAS";

/// Angle and Rotational Speed
pub const MODE_GYRO_G_AND_A: &str = "GYRO-G&A";

/// Calibration ???
pub const MODE_GYRO_CAL: &str = "GYRO-CAL";

/// LEGO EV3 gyro sensor.
#[derive(Debug, Clone, Device, Sensor, Findable)]
#[class_name = "lego-sensor"]
#[driver_name = "lego-ev3-gyro"]
#[port = "crate::sensors::SensorPort"]
pub struct GyroSensor {
    driver: Driver,
}

impl GyroSensor {
    /// Angle
    pub fn set_mode_col_ang(&self) -> Ev3Result<()> {
        self.set_mode(MODE_GYRO_ANG)
    }

    /// Rotational Speed
    pub fn set_mode_col_rate(&self) -> Ev3Result<()> {
        self.set_mode(MODE_GYRO_RATE)
    }

    /// Raw sensor value ???
    pub fn set_mode_col_fas(&self) -> Ev3Result<()> {
        self.set_mode(MODE_GYRO_FAS)
    }

    /// Angle and Rotational Speed
    pub fn set_mode_gyro_g_and_a(&self) -> Ev3Result<()> {
        self.set_mode(MODE_GYRO_G_AND_A)
    }

    /// Calibration ???
    pub fn set_mode_gyro_cal(&self) -> Ev3Result<()> {
        self.set_mode(MODE_GYRO_CAL)
    }
}
