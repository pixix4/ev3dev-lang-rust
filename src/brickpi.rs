//! EV3 specific features

use std::fs;
use std::path::Path;

use crate::driver::DRIVER_PATH;
use crate::utils::OrErr;
use crate::{Attribute, Ev3Result};

/// Color type.
pub type Color = u8;

/// The led's on top of the EV3 brick.
#[derive(Debug, Clone)]
pub struct Led {
    led1: Attribute,
    led2: Attribute,
}

impl Led {
    /// Led off.
    pub const COLOR_OFF: Color = 0;

    /// Led color blue
    pub const COLOR_BLUE: Color = 255;

    /// Create a new instance of the `Led` struct.
    pub fn new() -> Ev3Result<Led> {
        let mut led1_name = String::new();
        let mut led2_name = String::new();

        let paths = fs::read_dir(Path::new(DRIVER_PATH).join("leds"))?;

        for path in paths {
            let file_name = path?.file_name();
            let name = file_name.to_str().or_err()?.to_owned();

            if name.contains("led1") {
                led1_name = name;
            } else if name.contains("led2") {
                led2_name = name;
            }
        }

        let led1 = Attribute::from_sys_class("leds", led1_name.as_str(), "brightness")?;
        let led2 = Attribute::from_sys_class("leds", led2_name.as_str(), "brightness")?;

        Ok(Led { led1, led2 })
    }

    /// Returns the current brightness value of led1.
    pub fn get_led1(&self) -> Ev3Result<Color> {
        self.led1.get()
    }

    /// Sets the brightness value of led1.
    pub fn set_led1(&self, brightness: Color) -> Ev3Result<()> {
        self.led1.set(brightness)
    }

    /// Returns the current brightness value of led2.
    pub fn get_led2(&self) -> Ev3Result<Color> {
        self.led2.get()
    }

    /// Sets the brightness value of led2.
    pub fn set_led2(&self, brightness: Color) -> Ev3Result<()> {
        self.led2.set(brightness)
    }
}
