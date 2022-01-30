//! LEGO EV3 color sensor.

use super::{Sensor, SensorPort};
use crate::{sensor_mode, Attribute, Device, Driver, Ev3Error, Ev3Result};

/// LEGO EV3 color sensor.
#[derive(Debug, Clone, Device, Sensor)]
pub struct ColorSensor {
    driver: Driver,
}

impl ColorSensor {
    fn new(driver: Driver) -> Self {
        Self { driver }
    }

    findable!(
        "lego-sensor",
        ["lego-ev3-color"],
        SensorPort,
        "ColorSensor",
        "in"
    );

    sensor_mode!(
        "COL-REFLECT",
        MODE_COL_REFLECT,
        "Reflected light - sets LED color to red",
        set_mode_col_reflect,
        is_mode_col_reflect
    );
    sensor_mode!(
        "COL-AMBIENT",
        MODE_COL_AMBIENT,
        "Ambient light - sets LED color to blue (dimly lit)",
        set_mode_col_ambient,
        is_mode_col_ambient
    );
    sensor_mode!(
        "COL-COLOR",
        MODE_COL_COLOR,
        "Color - sets LED color to white (all LEDs rapidly cycling)",
        set_mode_col_color,
        is_mode_col_color
    );
    sensor_mode!(
        "REF-RAW",
        MODE_REF_RAW,
        "Raw Reflected - sets LED color to red",
        set_mode_ref_raw,
        is_mode_ref_raw
    );
    sensor_mode!(
        "RGB-RAW",
        MODE_RGB_RAW,
        "Raw Color Components - sets LED color to white (all LEDs rapidly cycling)",
        set_mode_rgb_raw,
        is_mode_rgb_raw
    );
    sensor_mode!(
        "COL-CAL",
        MODE_COL_CAL,
        "Calibration ??? - sets LED color to red, flashing every 4 seconds, then goes continuous",
        set_mode_col_cal,
        is_mode_col_cal
    );

    /// Get the color value for the modes `COL-REFLECT`, `COL-AMBIENT`, `COL-COLOR` and `REF-RAW`.
    pub fn get_color(&self) -> Ev3Result<i32> {
        self.get_value0()
    }

    /// Red component of the detected color, in the range 0-1020.
    pub fn get_red(&self) -> Ev3Result<i32> {
        self.get_value0()
    }

    /// Green component of the detected color, in the range 0-1020.
    pub fn get_green(&self) -> Ev3Result<i32> {
        self.get_value1()
    }

    /// Blue component of the detected color, in the range 0-1020.
    pub fn get_blue(&self) -> Ev3Result<i32> {
        self.get_value2()
    }

    /// Red, green and blue components of the detected color, each in the range 0-1020
    pub fn get_rgb(&self) -> Ev3Result<(i32, i32, i32)> {
        let red = self.get_red()?;
        let green = self.get_green()?;
        let blue = self.get_blue()?;

        Ok((red, green, blue))
    }
}
