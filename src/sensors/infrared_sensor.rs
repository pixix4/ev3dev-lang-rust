//! LEGO EV3 infrared sensor.

use super::{Sensor, SensorPort};
use crate::{sensor_mode, Attribute, Device, Driver, Ev3Error, Ev3Result};
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

/// LEGO EV3 infrared sensor.
#[derive(Debug, Clone, Device, Sensor)]
pub struct InfraredSensor {
    driver: Driver,
}

impl InfraredSensor {
    fn new(driver: Driver) -> Self {
        Self { driver }
    }

    findable!(
        "lego-sensor",
        ["lego-ev3-ir"],
        SensorPort,
        "InfraredSensor",
        "in"
    );

    sensor_mode!(
        "IR-PROX",
        MODE_IR_PROX,
        "Proximity",
        set_mode_ir_prox,
        is_mode_ir_prox
    );
    sensor_mode!(
        "IR-SEEK",
        MODE_IR_SEEK,
        "IR Seeker",
        set_mode_ir_seek,
        is_mode_ir_seek
    );
    sensor_mode!(
        "IR-REMOTE",
        MODE_IR_REMOTE,
        "IR Remote Control",
        set_mode_ir_remote,
        is_mode_ir_remote
    );
    sensor_mode!(
        "IR-REM-A",
        MODE_IR_REM_A,
        "IR Remote Control",
        set_mode_ir_rem_a,
        is_mode_ir_rem_a
    );
    sensor_mode!(
        "IR-S-ALT",
        MODE_IR_S_ALT,
        "Alternate IR Seeker ???",
        set_mode_ir_s_alt,
        is_mode_ir_s_alt
    );
    sensor_mode!(
        "IR-CAL",
        MODE_IR_CAL,
        "Calibration ???",
        set_mode_ir_cal,
        is_mode_ir_cal
    );

    /// Get the proximity distance, in the range 0-100 (pct).
    pub fn get_distance(&self) -> Ev3Result<i32> {
        self.get_value0()
    }
}

struct RemoteControlHelper {
    last_buttons: i32,
    pressed_buttons: HashSet<String>,
}

impl RemoteControlHelper {
    fn new() -> RemoteControlHelper {
        RemoteControlHelper {
            last_buttons: 0,
            pressed_buttons: HashSet::new(),
        }
    }

    fn contains(&self, button: &str) -> bool {
        self.pressed_buttons.contains(button)
    }
}

/// Seeks EV3 Remote Controller in beacon mode.
#[derive(Clone)]
pub struct RemoteControl {
    sensor: InfraredSensor,
    channel: u8,
    helper: Rc<RefCell<RemoteControlHelper>>,
}

// Manually implement Debug cause `buffer_cache` does not implement Debug.
impl fmt::Debug for RemoteControl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RemoteControl")
            .field("sensor", &self.sensor)
            .field("channel", &self.channel)
            .finish()
    }
}

impl RemoteControl {
    /// Wrap a InfraredSensor into a BeaconSeeker
    pub fn new(sensor: InfraredSensor, channel: u8) -> Ev3Result<RemoteControl> {
        sensor.set_mode_ir_remote()?;

        Ok(RemoteControl {
            sensor,
            channel: u8::max(1, u8::min(4, channel)) - 1,
            helper: Rc::new(RefCell::new(RemoteControlHelper::new())),
        })
    }

    /// Checks if `red_up` button is pressed.
    pub fn is_red_up(&self) -> bool {
        self.helper.borrow().contains("red_up")
    }

    /// Checks if `red_down` button is pressed.
    pub fn is_red_down(&self) -> bool {
        self.helper.borrow().contains("red_down")
    }

    /// Checks if `blue_up` button is pressed.
    pub fn is_blue_up(&self) -> bool {
        self.helper.borrow().contains("blue_up")
    }

    /// Checks if `blue_down` button is pressed.
    pub fn is_blue_down(&self) -> bool {
        self.helper.borrow().contains("blue_down")
    }

    /// Checks if `beacon` button is pressed.
    pub fn is_beacon(&self) -> bool {
        self.helper.borrow().contains("beacon")
    }

    /// Check for currently pressed buttons. If the new state differs from the
    /// old state, call the appropriate button event handlers.
    pub fn process(&self) -> Ev3Result<()> {
        let buttons = self.sensor.get_value(self.channel)?;

        let mut helper = self.helper.borrow_mut();

        if helper.last_buttons != buttons {
            helper.last_buttons = buttons;

            helper.pressed_buttons.clear();

            match buttons {
                1 => {
                    helper.pressed_buttons.insert("red_up".to_owned());
                }
                2 => {
                    helper.pressed_buttons.insert("red_down".to_owned());
                }
                3 => {
                    helper.pressed_buttons.insert("blue_up".to_owned());
                }
                4 => {
                    helper.pressed_buttons.insert("blue_down".to_owned());
                }
                5 => {
                    helper.pressed_buttons.insert("red_up".to_owned());
                    helper.pressed_buttons.insert("blue_up".to_owned());
                }
                6 => {
                    helper.pressed_buttons.insert("red_up".to_owned());
                    helper.pressed_buttons.insert("blue_down".to_owned());
                }
                7 => {
                    helper.pressed_buttons.insert("red_down".to_owned());
                    helper.pressed_buttons.insert("blue_up".to_owned());
                }
                8 => {
                    helper.pressed_buttons.insert("red_down".to_owned());
                    helper.pressed_buttons.insert("blue_down".to_owned());
                }
                9 => {
                    helper.pressed_buttons.insert("beacon".to_owned());
                }
                10 => {
                    helper.pressed_buttons.insert("red_up".to_owned());
                    helper.pressed_buttons.insert("red_down".to_owned());
                }
                11 => {
                    helper.pressed_buttons.insert("blue_up".to_owned());
                    helper.pressed_buttons.insert("blue_down".to_owned());
                }
                _ => {}
            }
        }
        Ok(())
    }
}

/// Seeks EV3 Remote Controller in beacon mode.
#[derive(Debug, Clone)]
pub struct BeaconSeeker {
    sensor: InfraredSensor,
    channel: u8,
}

impl BeaconSeeker {
    /// Wrap a InfraredSensor into a BeaconSeeker
    pub fn new(sensor: InfraredSensor, channel: u8) -> Ev3Result<BeaconSeeker> {
        sensor.set_mode_ir_seek()?;

        Ok(BeaconSeeker {
            sensor,
            channel: u8::max(1, u8::min(4, channel)) - 1,
        })
    }

    /// Returns heading (-25, 25) to the beacon on the given channel.
    pub fn get_heading(&self) -> Ev3Result<i32> {
        self.sensor.get_value(self.channel * 2)
    }

    /// Returns distance (0, 100) to the beacon on the given channel.
    /// Returns -128 when beacon is not found.
    pub fn get_distance(&self) -> Ev3Result<i32> {
        self.sensor.get_value(self.channel * 2 + 1)
    }

    /// Returns heading and distance to the beacon on the given channel as a
    /// tuple.
    pub fn get_heading_and_distance(&self) -> Ev3Result<(i32, i32)> {
        Ok((
            self.sensor.get_value(self.channel * 2)?,
            self.sensor.get_value(self.channel * 2 + 1)?,
        ))
    }
}
