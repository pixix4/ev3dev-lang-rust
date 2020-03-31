//! The leds on top of the EV3 brick.

use std::fs;

use crate::{utils::OrErr, Attribute, Ev3Result};

/// Color type.
pub type Color = (u8, u8);

/// The leds on top of the EV3 brick.
#[derive(Debug, Clone)]
pub struct Led {
    left_red: Attribute,
    left_green: Attribute,
    right_red: Attribute,
    right_green: Attribute,
}

impl Led {

    /// Led off.
    pub const COLOR_OFF: Color = (0, 0);

    /// Led color red
    pub const COLOR_RED: Color = (255, 0);

    /// Led color green.
    pub const COLOR_GREEN: Color = (0, 255);

    /// Led color amber.
    pub const COLOR_AMBER: Color = (255, 255);

    /// Led color orange.
    pub const COLOR_ORANGE: Color = (255, 128);

    /// LED color yellow.
    pub const COLOR_YELLOW: Color = (25, 255);

    /// Create a new instance of the `Led` struct.
    pub fn new() -> Ev3Result<Led> {
        let mut left_red_name = String::new();
        let mut left_green_name = String::new();
        let mut right_red_name = String::new();
        let mut right_green_name = String::new();

        let paths = fs::read_dir("/sys/class/leds")?;

        for path in paths {
            let file_name = path?.file_name();
            let name = file_name.to_str().or_err()?.to_owned();

            if name.contains(":brick-status") || name.contains(":ev3dev") {
                if name.contains("led0:") || name.contains("left:") {
                    if name.contains("red:") {
                        left_red_name = name;
                    } else if name.contains("green:") {
                        left_green_name = name
                    }
                } else if name.contains("led1:") || name.contains("right:") {
                    if name.contains("red:") {
                        right_red_name = name
                    } else if name.contains("green:") {
                        right_green_name = name
                    }
                }
            }
        }

        let left_red = Attribute::new("leds", left_red_name.as_str(), "brightness")?;
        let left_green = Attribute::new("leds", left_green_name.as_str(), "brightness")?;
        let right_red = Attribute::new("leds", right_red_name.as_str(), "brightness")?;
        let right_green = Attribute::new("leds", right_green_name.as_str(), "brightness")?;

        Ok(Led {
            left_red,
            left_green,
            right_red,
            right_green,
        })
    }

    /// Returns the current red value of the left led.
    fn get_left_red(&self) -> Ev3Result<u8> {
        self.left_red.get()
    }

    /// Sets the red value of the left led.
    fn set_left_red(&self, brightness: u8) -> Ev3Result<()> {
        self.left_red.set(brightness)
    }

    /// Returns the current green value of the left led.
    fn get_left_green(&self) -> Ev3Result<u8> {
        self.left_green.get()
    }

    /// Sets the green value of the left led.
    fn set_left_green(&self, brightness: u8) -> Ev3Result<()> {
        self.left_green.set(brightness)
    }

    /// Returns the current red value of the right led.
    fn get_right_red(&self) -> Ev3Result<u8> {
        self.right_red.get()
    }

    /// Sets the red value of the right led.
    fn set_right_red(&self, brightness: u8) -> Ev3Result<()> {
        self.right_red.set(brightness)
    }

    /// Returns the current green value of the right led.
    fn get_right_green(&self) -> Ev3Result<u8> {
        self.right_green.get()
    }

    /// Sets the green value of the right led.
    fn set_right_green(&self, brightness: u8) -> Ev3Result<()> {
        self.right_green.set(brightness)
    }

    /// Returns the current color value of the left led.
    pub fn get_left_color(&self) -> Ev3Result<Color> {
        let red = self.get_left_red()?;
        let green = self.get_left_green()?;

        Ok((red, green))
    }

    /// Sets the color value of the left led.
    pub fn set_left_color(&self, color: Color) -> Ev3Result<()> {
        self.set_left_red(color.0)?;
        self.set_left_green(color.1)
    }

    /// Returns the current color value of the right led.
    pub fn get_right_color(&self) -> Ev3Result<Color> {
        let red = self.get_right_red()?;
        let green = self.get_right_green()?;

        Ok((red, green))
    }

    /// Sets the color value of the right led.
    pub fn set_right_color(&self, color: Color) -> Ev3Result<()> {
        self.set_right_red(color.0)?;
        self.set_right_green(color.1)
    }

    /// Returns the color value of both leds or `None` if they are different.
    pub fn get_color(&self) -> Ev3Result<Option<Color>> {
        let left = self.get_left_color()?;
        let right = self.get_right_color()?;

        if left.0 == right.0 && left.1 == right.1 {
            Ok(Some(left))
        } else {
            Ok(None)
        }
    }

    /// Sets the color value of both leds.
    pub fn set_color(&self, color: Color) -> Ev3Result<()> {
        self.set_left_color(color)?;
        self.set_right_color(color)
    }
}
