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
    led: Attribute,
}

impl Led {
    /// Led off.
    pub const COLOR_OFF: Color = 0;

    /// Led color blue
    pub const COLOR_AMBER: Color = 255;

    /// Create a new instance of the `Led` struct.
    pub fn new() -> Ev3Result<Led> {
        let mut led_name = String::new();

        let paths = fs::read_dir(Path::new(DRIVER_PATH).join("leds"))?;

        for path in paths {
            let file_name = path?.file_name();
            let name = file_name.to_str().or_err()?.to_owned();

            if name.contains(":brick-status") && name.contains("led0:") {
                led_name = name;
            }
        }

        let led = Attribute::from_sys_class("leds", led_name.as_str(), "brightness")?;

        Ok(Led { led })
    }

    /// Returns the current brightness value of led.
    pub fn get_led(&self) -> Ev3Result<Color> {
        self.led.get()
    }

    /// Sets the brightness value of led.
    pub fn set_led(&self, brightness: Color) -> Ev3Result<()> {
        self.led.set(brightness)
    }
}
