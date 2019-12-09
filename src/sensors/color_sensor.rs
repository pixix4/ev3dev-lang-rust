//! LEGO EV3 color sensor.

use crate::sensors::Sensor;
use crate::{Attribute, Device, Driver, Ev3Result, Findable};

/// Reflected light - sets LED color to red
pub const MODE_COL_REFLECT: &str = "COL-REFLECT";

/// Ambient light - sets LED color to blue (dimly lit)
pub const MODE_COL_AMBIENT: &str = "COL-AMBIENT";

/// Color - sets LED color to white (all LEDs rapidly cycling)
pub const MODE_COL_COLOR: &str = "COL-COLOR";

/// Raw Reflected - sets LED color to red
pub const MODE_REF_RAW: &str = "REF-RAW";

/// Raw Color Components - sets LED color to white (all LEDs rapidly cycling)
pub const MODE_RGB_RAW: &str = "RGB-RAW";

/// Calibration ??? - sets LED color to red, flashing every 4 seconds, then goes continuous
pub const MODE_COL_CAL: &str = "COL-CAL";

/// LEGO EV3 color sensor.
#[derive(Debug, Clone, Device, Sensor, Findable)]
#[class_name = "lego-sensor"]
#[driver_name = "lego-ev3-color"]
#[port = "crate::sensors::SensorPort"]
pub struct ColorSensor {
    driver: Driver,
}

impl ColorSensor {
    /// Reflected light - sets LED color to red
    pub fn set_mode_col_reflect(&self) -> Ev3Result<()> {
        self.set_mode(MODE_COL_REFLECT)
    }

    /// Ambient light - sets LED color to blue (dimly lit)
    pub fn set_mode_col_ambient(&self) -> Ev3Result<()> {
        self.set_mode(MODE_COL_AMBIENT)
    }

    /// Color - sets LED color to white (all LEDs rapidly cycling)
    pub fn set_mode_col_color(&self) -> Ev3Result<()> {
        self.set_mode(MODE_COL_COLOR)
    }

    /// Raw Reflected - sets LED color to red
    pub fn set_mode_ref_raw(&self) -> Ev3Result<()> {
        self.set_mode(MODE_REF_RAW)
    }

    /// Raw Color Components - sets LED color to white (all LEDs rapidly cycling)
    pub fn set_mode_rgb_raw(&self) -> Ev3Result<()> {
        self.set_mode(MODE_RGB_RAW)
    }

    /// Calibration ??? - sets LED color to red, flashing every 4 seconds, then goes continuous
    pub fn set_mode_col_cal(&self) -> Ev3Result<()> {
        self.set_mode(MODE_COL_CAL)
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

    /// Red, green and blue componets of the detected color, each in the range 0-1020
    pub fn get_rgb(&self) -> Ev3Result<(i32, i32, i32)> {
        let red = self.get_red()?;
        let green = self.get_green()?;
        let blue = self.get_blue()?;

        Ok((red, green, blue))
    }
}
