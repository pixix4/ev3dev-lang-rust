use std::fs;

use crate::{Attribute, Ev3Result, utils::OrErr};

pub const COLOR_OFF: (u8, u8) = (0, 0);
pub const COLOR_RED: (u8, u8) = (255, 0);
pub const COLOR_GREEN: (u8, u8) = (0, 255);
pub const COLOR_AMBER: (u8, u8) = (255, 255);
pub const COLOR_ORANGE: (u8, u8) = (255, 128);
pub const COLOR_YELLOW: (u8, u8) = (25, 255);

#[derive(Clone)]
pub struct Led {
    left_red: Attribute,
    left_green: Attribute,
    right_red: Attribute,
    right_green: Attribute,
}

impl Led {
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

    pub fn get_left_red(&self) -> Ev3Result<u8> {
        self.left_red.get()
    }
    pub fn set_left_red(&self, brightness: u8) -> Ev3Result<()> {
        self.left_red.set(brightness)
    }

    pub fn get_left_green(&self) -> Ev3Result<u8> {
        self.left_green.get()
    }
    pub fn set_left_green(&self, brightness: u8) -> Ev3Result<()> {
        self.left_green.set(brightness)
    }

    pub fn get_right_red(&self) -> Ev3Result<u8> {
        self.right_red.get()
    }
    pub fn set_right_red(&self, brightness: u8) -> Ev3Result<()> {
        self.right_red.set(brightness)
    }

    pub fn get_right_green(&self) -> Ev3Result<u8> {
        self.right_green.get()
    }
    pub fn set_right_green(&self, brightness: u8) -> Ev3Result<()> {
        self.right_green.set(brightness)
    }

    pub fn get_left_color(&self) -> Ev3Result<(u8, u8)> {
        let red = self.get_left_red()?;
        let green = self.get_left_green()?;

        Ok((red, green))
    }
    pub fn set_left_color(&self, color: (u8, u8)) -> Ev3Result<()> {
        self.set_left_red(color.0)?;
        self.set_left_green(color.1)
    }

    pub fn get_right_color(&self) -> Ev3Result<(u8, u8)> {
        let red = self.get_right_red()?;
        let green = self.get_right_green()?;

        Ok((red, green))
    }
    pub fn set_right_color(&self, color: (u8, u8)) -> Ev3Result<()> {
        self.set_right_red(color.0)?;
        self.set_right_green(color.1)
    }

    /// Returns None if left and right colors are different.
    pub fn get_color(&self) -> Ev3Result<Option<(u8, u8)>> {
        let left = self.get_left_color()?;
        let right = self.get_right_color()?;

        if left.0 == right.0 && left.1 == right.1 {
            Ok(Some(left))
        } else {
            Ok(None)
        }
    }
    pub fn set_color(&self, color: (u8, u8)) -> Ev3Result<()> {
        self.set_left_color(color)?;
        self.set_right_color(color)
    }
}
