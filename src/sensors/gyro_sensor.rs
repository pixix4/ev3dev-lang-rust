//! LEGO EV3 gyro sensor.

use super::SensorPort;
use crate::{Attribute, Device, Driver, Ev3Error, Ev3Result};

/// LEGO EV3 gyro sensor.
#[derive(Debug, Clone, Device)]
pub struct GyroSensor {
    driver: Driver,
}

impl GyroSensor {
    fn new(driver: Driver) -> Self {
        Self { driver }
    }

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

    /// Gets the angle, ranging from -32768 to 32767
    /// Fails if it has been set in the wrong mode
    pub fn get_angle(&self) -> Ev3Result<i32> {
        if self.get_mode()? == GyroSensor::MODE_GYRO_G_AND_A
            || self.get_mode()? == GyroSensor::MODE_GYRO_ANG
        {
            return self.get_value0();
        }
        Ev3Result::Err(Ev3Error::InternalError {
            msg: format!("Cannot get angle while in {}", self.get_mode()?),
        })
    }

    /// Gets the rotational speed value, ranging from -440 to 440
    /// Fails is it has been set in the wrong mode:
    /// for example, fails if we ask for rotational speed while in MODE_GYRO_ANG mode
    pub fn get_rotational_speed(&self) -> Ev3Result<i32> {
        if self.get_mode()? == GyroSensor::MODE_GYRO_RATE {
            return self.get_value0();
        } else if self.get_mode()? == GyroSensor::MODE_GYRO_G_AND_A {
            return self.get_value1();
        }

        Ev3Result::Err(Ev3Error::InternalError {
            msg: format!("Cannot get rotational speed while in {}", self.get_mode()?),
        })
    }
}
