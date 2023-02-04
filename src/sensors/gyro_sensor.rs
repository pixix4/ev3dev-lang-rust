//! LEGO EV3 gyro sensor.

use super::{Sensor, SensorPort};
use crate::{sensor_mode, Attribute, Device, Driver, Ev3Error, Ev3Result};

/// LEGO EV3 gyro sensor.
#[derive(Debug, Clone, Device, Sensor)]
pub struct GyroSensor {
    driver: Driver,
}

impl GyroSensor {
    fn new(driver: Driver) -> Self {
        Self { driver }
    }

    findable!(
        "lego-sensor",
        ["lego-ev3-gyro"],
        SensorPort,
        "GyroSensor",
        "in"
    );
    sensor_mode!(
        "GYRO-ANG",
        MODE_GYRO_ANG,
        "Angle",
        set_mode_gyro_ang,
        is_mode_gyro_ang
    );
    sensor_mode!(
        "GYRO-RATE",
        MODE_GYRO_RATE,
        "Rotational Speed",
        set_mode_gyro_rate,
        is_mode_gyro_rate
    );
    sensor_mode!(
        "GYRO-FAS",
        MODE_GYRO_FAS,
        "Raw sensor value ???",
        set_mode_gyro_fas,
        is_mode_gyro_fas
    );
    sensor_mode!(
        "GYRO-G&A",
        MODE_GYRO_G_AND_A,
        "Angle and Rotational Speed",
        set_mode_gyro_g_and_a,
        is_mode_gyro_g_and_a
    );
    sensor_mode!(
        "GYRO-CAL",
        MODE_GYRO_CAL,
        "Calibration ???",
        set_mode_gyro_cal,
        is_mode_gyro_cal
    );
    sensor_mode!(
        "TILT-RATE",
        MODE_TILT_RATE,
        "Rotational Speed (2nd axis)",
        set_mode_tilt_rate,
        is_mode_tilt_rate
    );
    sensor_mode!(
        "TILT-ANG",
        MODE_TILT_ANG,
        "Angle (2nd axis)",
        set_mode_tilt_ang,
        is_mode_tilt_ang
    );

    /// Gets the angle, ranging from -32768 to 32767
    /// Fails if it has been set in the wrong mode
    pub fn get_angle(&self) -> Ev3Result<i32> {
        match self.get_mode()?.as_ref() {
            GyroSensor::MODE_GYRO_G_AND_A => self.get_value0(),
            GyroSensor::MODE_GYRO_ANG => self.get_value0(),
            mode => Ev3Result::Err(Ev3Error::InternalError {
                msg: format!("Cannot get angle while in {mode} mode"),
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
                msg: format!("Cannot get rotational speed while in {mode} mode"),
                // Returns error
            }),
        }
    }
}
