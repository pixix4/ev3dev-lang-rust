//! EV3 specific features

use std::fs;

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use std::rc::Rc;

use crate::driver::DRIVER_PATH;
use crate::utils::OrErr;
use crate::{Attribute, Ev3Result};

/// Color type.
pub type Color = (u8, u8);

/// The led's on top of the EV3 brick.
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

        let paths = fs::read_dir(Path::new(DRIVER_PATH).join("leds"))?;

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

        let left_red = Attribute::from_sys_class("leds", left_red_name.as_str(), "brightness")?;
        let left_green = Attribute::from_sys_class("leds", left_green_name.as_str(), "brightness")?;
        let right_red = Attribute::from_sys_class("leds", right_red_name.as_str(), "brightness")?;
        let right_green =
            Attribute::from_sys_class("leds", right_green_name.as_str(), "brightness")?;

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

const KEY_BUF_LEN: usize = 96;
const EVIOCGKEY: u32 = 2_153_792_792;

/// Helper struct for ButtonFileHandler.
struct FileMapEntry {
    pub file: File,
    pub buffer_cache: [u8; KEY_BUF_LEN],
}
// Manually implement Debug cause `buffer_cache` does not implement Debug.
impl fmt::Debug for FileMapEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileMapEntry")
            .field("file", &self.file)
            .finish()
    }
}

/// Helper struct for ButtonFileHandler.
#[derive(Debug)]
struct ButtonMapEntry {
    pub file_name: String,
    pub key_code: u32,
}

/// This implementation depends on the availability of the EVIOCGKEY ioctl
/// to be able to read the button state buffer. See Linux kernel source
/// in /include/uapi/linux/input.h for details.
#[derive(Debug)]
struct ButtonFileHandler {
    file_map: HashMap<String, FileMapEntry>,
    button_map: HashMap<String, ButtonMapEntry>,
    pressed_buttons: HashSet<String>,
}

impl ButtonFileHandler {
    /// Create a new instance.
    fn new() -> Self {
        ButtonFileHandler {
            file_map: HashMap::new(),
            button_map: HashMap::new(),
            pressed_buttons: HashSet::new(),
        }
    }

    /// Add a button the the file handler.
    fn add_button(&mut self, name: &str, file_name: &str, key_code: u32) -> Ev3Result<()> {
        if !self.file_map.contains_key(file_name) {
            let file = File::open(file_name)?;
            let buffer_cache = [0u8; KEY_BUF_LEN];

            self.file_map
                .insert(file_name.to_owned(), FileMapEntry { file, buffer_cache });
        }

        self.button_map.insert(
            name.to_owned(),
            ButtonMapEntry {
                file_name: file_name.to_owned(),
                key_code,
            },
        );

        Ok(())
    }

    fn get_pressed_buttons(&self) -> HashSet<String> {
        self.pressed_buttons.clone()
    }

    /// Check if a button is pressed.
    fn get_button_state(&self, name: &str) -> bool {
        self.pressed_buttons.contains(name)
    }

    /// Check for currently pressed buttons. If the new state differs from the
    /// old state, call the appropriate button event handlers.
    fn process(&mut self) {
        for entry in self.file_map.values_mut() {
            unsafe {
                libc::ioctl(
                    entry.file.as_raw_fd(),
                    (EVIOCGKEY as i32).try_into().unwrap(),
                    &mut entry.buffer_cache,
                );
            }
        }

        self.pressed_buttons.clear();

        for (
            btn_name,
            ButtonMapEntry {
                file_name,
                key_code,
            },
        ) in self.button_map.iter()
        {
            let buffer = &self.file_map[file_name].buffer_cache;

            if (buffer[(key_code / 8) as usize] & 1 << (key_code % 8)) != 0 {
                self.pressed_buttons.insert(btn_name.to_owned());
            }
        }
    }
}

/// Ev3 brick button handler. Opens the corresponding `/dev/input` file handlers.
///
/// This implementation depends on the availability of the EVIOCGKEY ioctl
/// to be able to read the button state buffer. See Linux kernel source
/// in /include/uapi/linux/input.h for details.
///
/// ```no_run
/// use ev3dev_lang_rust::Button;
/// use std::thread;
/// use std::time::Duration;
///
/// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
/// let button = Button::new()?;
///
/// loop {
///     button.process();
///
///     println!("Is 'up' pressed: {}", button.is_up());
///     println!("Pressed buttons: {:?}", button.get_pressed_buttons());
///
///     thread::sleep(Duration::from_millis(100));
/// }
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Button {
    button_handler: Rc<RefCell<ButtonFileHandler>>,
}

impl Button {
    /// Ev3 brick button handler. Opens the corresponding `/dev/input` file handlers.
    pub fn new() -> Ev3Result<Self> {
        let mut handler = ButtonFileHandler::new();

        handler.add_button("up", "/dev/input/by-path/platform-gpio_keys-event", 103)?;
        handler.add_button("down", "/dev/input/by-path/platform-gpio_keys-event", 108)?;
        handler.add_button("left", "/dev/input/by-path/platform-gpio_keys-event", 105)?;
        handler.add_button("right", "/dev/input/by-path/platform-gpio_keys-event", 106)?;
        handler.add_button("enter", "/dev/input/by-path/platform-gpio_keys-event", 28)?;
        handler.add_button(
            "backspace",
            "/dev/input/by-path/platform-gpio_keys-event",
            14,
        )?;

        Ok(Self {
            button_handler: Rc::new(RefCell::new(handler)),
        })
    }

    /// Check for currently pressed buttons. If the new state differs from the
    /// old state, call the appropriate button event handlers.
    pub fn process(&self) {
        self.button_handler.borrow_mut().process()
    }

    /// Get all pressed buttons by name.
    pub fn get_pressed_buttons(&self) -> HashSet<String> {
        self.button_handler.borrow().get_pressed_buttons()
    }

    /// Check if 'up' button is pressed.
    pub fn is_up(&self) -> bool {
        self.button_handler.borrow().get_button_state("up")
    }

    /// Check if 'down' button is pressed.
    pub fn is_down(&self) -> bool {
        self.button_handler.borrow().get_button_state("down")
    }

    /// Check if 'left' button is pressed.
    pub fn is_left(&self) -> bool {
        self.button_handler.borrow().get_button_state("left")
    }

    /// Check if 'right' button is pressed.
    pub fn is_right(&self) -> bool {
        self.button_handler.borrow().get_button_state("right")
    }

    /// Check if 'enter' button is pressed.
    pub fn is_enter(&self) -> bool {
        self.button_handler.borrow().get_button_state("enter")
    }

    /// Check if 'backspace' button is pressed.
    pub fn is_backspace(&self) -> bool {
        self.button_handler.borrow().get_button_state("backspace")
    }
}
