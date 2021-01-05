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

    /// Rotational Speed (2nd axis)
    pub const MODE_TILT_RATE: &'static str = "TILT-RATE";

    /// Angle (2nd axis)
    pub const MODE_TILT_ANG: &'static str = "TILT-ANG";

    sensor!();

    /// Angle
    pub fn set_mode_gyro_ang(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_GYRO_ANG)
    }

    /// Rotational Speed
    pub fn set_mode_gyro_rate(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_GYRO_RATE)
    }

    /// Raw sensor value ???
    pub fn set_mode_gyro_fas(&self) -> Ev3Result<()> {
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

    /// Calibration ???
    pub fn set_mode_tilt_rate(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_TILT_RATE)
    }

    /// Calibration ???
    pub fn set_mode_tilt_ang(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_TILT_ANG)
    }

    /// Gets the angle, ranging from -32768 to 32767
    /// Fails if it has been set in the wrong mode
    pub fn get_angle(&self) -> Ev3Result<i32> {
        match self.get_mode()?.as_ref() {
            GyroSensor::MODE_GYRO_G_AND_A => self.get_value0(),
            GyroSensor::MODE_GYRO_ANG => self.get_value0(),
            mode => Ev3Result::Err(Ev3Error::InternalError {
                msg: format!("Cannot get angle while in {} mode", mode),
                // Returns error
            }),
        }
    }

    /// Gets the rotational speed value, ranging from -440 to 440
    /// Fails is it has been set in the wrong mode:
    /// for example, fails if we ask for rotational speed while in MODE_GYRO_ANG mode
    pub fn get_rotational_speed(&self) -> Ev3Result<i32> {
        match self.get_mode()?.as_ref() {
            GyroSensor::MODE_GYRO_RATE => self.get_value0(),
            GyroSensor::MODE_GYRO_G_AND_A => self.get_value1(),
            mode => Ev3Result::Err(Ev3Error::InternalError {
                msg: format!("Cannot get rotational speed while in {} mode", mode),
                // Returns error
            }),
        }
    }
}
