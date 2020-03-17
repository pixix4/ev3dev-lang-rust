//! EV3 Buttons
//! 
//! ```no_run
//! use ev3dev_lang_rust::Ev3Button;
//! 
//! 
//! # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
//! let button = Ev3Button::new()?;
//! 
//! loop {
//!     button.process();
//! 
//!     println!("Is 'up' pressed: {}", button.is_up());
//!     println!("Pressed buttons: {:?}", button.get_pressed_buttons());
//! 
//!     std::thread::sleep_ms(100);
//! }
//! # }
//! ```

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::fmt;
use std::os::unix::io::AsRawFd;

use crate::Ev3Result;

const KEY_BUF_LEN: usize = 96;
const EVIOCGKEY: u32 = 2153792792;

/// Helper struct for ButtonFileHandler.
struct FileMapEntry {
    pub file: File,
    pub buffer_cache: [u8; KEY_BUF_LEN],
}
// Manuelly implement Debug cause `buffer_cache` does not implement Debug.
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
            pressed_buttons: HashSet::new()
        }
    }

    /// Add a button the the file handler.
    fn add_button(&mut self, name: &str, file_name: &str, key_code: u32) -> Ev3Result<()> {
        if !self.file_map.contains_key(file_name) {
            let file = File::open(file_name)?;
            let buffer_cache = [0u8; KEY_BUF_LEN];

            self.file_map.insert(file_name.to_owned(), FileMapEntry{
                file,
                buffer_cache,
            });
        }

        self.button_map.insert(name.to_owned(), ButtonMapEntry{
            file_name: file_name.to_owned(),
            key_code,
        });

        Ok(())
    }

    fn get_pressed_buttons(&self) -> HashSet<String> {
        self.pressed_buttons.clone()
    }

    /// Check if a button is pressed.
    fn get_button_state(&self, name: &str) -> bool {
        self.pressed_buttons.contains(name)
    }

    /// Check for currenly pressed buttons. If the new state differs from the
    /// old state, call the appropriate button event handlers.
    fn process(&mut self) {
        for entry in self.file_map.values_mut() {
            unsafe {
                libc::ioctl(entry.file.as_raw_fd(), EVIOCGKEY.into(), &mut entry.buffer_cache);
            }
        }

        self.pressed_buttons.clear();

        for (btn_name, ButtonMapEntry {file_name, key_code}) in self.button_map.iter() {
            let buffer = &self.file_map[file_name].buffer_cache;

            if (buffer[(key_code / 8) as usize] & 1 << key_code % 8) != 0 {
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
/// use ev3dev_lang_rust::Ev3Button;
/// 
/// # fn main() -> ev3dev_lang_rust::Ev3Result<()> {
/// let button = Ev3Button::new()?;
/// 
/// loop {
///     button.process();
/// 
///     println!("Is 'up' pressed: {}", button.is_up());
///     println!("Pressed buttons: {:?}", button.get_pressed_buttons());
/// 
///     std::thread::sleep_ms(100);
/// }
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Ev3Button {
    button_handler: Rc<RefCell<ButtonFileHandler>>
}

impl Ev3Button {

    /// Ev3 brick button handler. Opens the corresponding `/dev/input` file handlers.
    pub fn new() -> Ev3Result<Self> {
        let mut handler = ButtonFileHandler::new();

        handler.add_button("up", "/dev/input/by-path/platform-gpio_keys-event", 103)?;
        handler.add_button("down", "/dev/input/by-path/platform-gpio_keys-event", 108)?;
        handler.add_button("left", "/dev/input/by-path/platform-gpio_keys-event", 105)?;
        handler.add_button("right", "/dev/input/by-path/platform-gpio_keys-event", 106)?;
        handler.add_button("enter", "/dev/input/by-path/platform-gpio_keys-event", 28)?;
        handler.add_button("backspace", "/dev/input/by-path/platform-gpio_keys-event", 14)?;

        Ok(Self {
            button_handler: Rc::new(RefCell::new(handler))
        })
    }

    /// Check for currenly pressed buttons. If the new state differs from the
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