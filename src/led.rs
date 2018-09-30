use driver::Attribute;
use driver::AttributeResult;
use std::fs;

pub const COLOR_OFF: (u8, u8) = (0, 0);
pub const COLOR_RED: (u8, u8) = (255, 0);
pub const COLOR_GREEN: (u8, u8) = (0, 255);
pub const COLOR_AMBER: (u8, u8) = (255, 255);
pub const COLOR_ORANGE: (u8, u8) = (255, 128);
pub const COLOR_YELLOW: (u8, u8) = (25, 255);

pub struct Led {
    left_red: Attribute,
    left_green: Attribute,
    right_red: Attribute,
    right_green: Attribute,
}

impl Led {
    pub fn new() -> Option<Led> {
        let mut left_red_name = String::new();
        let mut left_green_name = String::new();
        let mut right_red_name = String::new();
        let mut right_green_name = String::new();

        let paths = fs::read_dir("/sys/class/leds").unwrap();

        for path in paths {
            let file_name = path.unwrap().file_name();
            let name = String::from(file_name.to_str().unwrap());

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

        return Some(Led { left_red, left_green, right_red, right_green });
    }

    pub fn get_left_red(&mut self) -> AttributeResult<u8> {
        let brightness = self.left_red.get_int()?;
        Ok(brightness as u8)
    }
    pub fn set_left_red(&mut self, brightness: u8) -> AttributeResult<()> {
        self.left_red.set_int(brightness as isize)
    }

    pub fn get_left_green(&mut self) -> AttributeResult<u8> {
        let brightness = self.left_green.get_int()?;
        Ok(brightness as u8)
    }
    pub fn set_left_green(&mut self, brightness: u8) -> AttributeResult<()> {
        self.left_green.set_int(brightness as isize)
    }


    pub fn get_right_red(&mut self) -> AttributeResult<u8> {
        let brightness = self.right_red.get_int()?;
        Ok(brightness as u8)
    }
    pub fn set_right_red(&mut self, brightness: u8) -> AttributeResult<()> {
        self.right_red.set_int(brightness as isize)
    }

    pub fn get_right_green(&mut self) -> AttributeResult<u8> {
        let brightness = self.right_green.get_int()?;
        Ok(brightness as u8)
    }
    pub fn set_right_green(&mut self, brightness: u8) -> AttributeResult<()> {
        self.right_green.set_int(brightness as isize)
    }


    pub fn get_left_color(&mut self) -> AttributeResult<(u8, u8)> {
        let red = self.get_left_red()?;
        let green = self.get_left_green()?;
        return Ok((red, green));
    }
    pub fn set_left_color(&mut self, color: (u8, u8)) -> AttributeResult<()> {
        self.set_left_red(color.0)?;
        self.set_left_green(color.1)
    }

    pub fn get_right_color(&mut self) -> AttributeResult<(u8, u8)> {
        let red = self.get_right_red()?;
        let green = self.get_right_green()?;
        return Ok((red, green));
    }
    pub fn set_right_color(&mut self, color: (u8, u8)) -> AttributeResult<()> {
        self.set_right_red(color.0)?;
        self.set_right_green(color.1)
    }

    /// Returns None if left and right colors are different.
    pub fn get_color(&mut self) -> AttributeResult<Option<(u8, u8)>> {
        let left = self.get_left_color()?;
        let right = self.get_right_color()?;
        if left.0 == right.0 && left.1 == right.1 {
            return Ok(Some(left));
        } else {
            return Ok(None);
        }
    }
    pub fn set_color(&mut self, color: (u8, u8)) -> AttributeResult<()> {
        self.set_left_color(color)?;
        self.set_right_color(color)
    }
}