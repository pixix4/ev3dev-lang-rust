//! LEGO EV3 ultrasonic sensor

use crate::sensors::Sensor;
use crate::{Attribute, Device, Driver, Ev3Result, Findable};

/// Continuous measurement - sets LEDs on, steady.
/// Units in centimeters. Distance (0-2550)
pub const MODE_US_DIST_CM: &str = "US-DIST-CM";

/// Continuous measurement - sets LEDs on, steady.
/// Units in inches. Distance (0-1003)
pub const MODE_US_DIST_IN: &str = "US-DIST-IN";

/// Listen - sets LEDs on, blinking. Presence (0-1)
pub const MODE_US_LISTEN: &str = "US-LISTEN";

/// Single measurement - LEDs on momentarily when mode is set, then off.
/// Units in centimeters. Distance (0-2550)
pub const MODE_US_SI_CM: &str = "US-SI-CM";

/// Single measurement - LEDs on momentarily when mode is set, then off.
/// Units in inches. Distance (0-1003)
pub const MODE_US_SI_IN: &str = "US-SI-IN";

/// ??? - sets LEDs on, steady.
/// Units in centimeters. Distance (0-2550)
pub const MODE_US_DC_CM: &str = "US-DC-CM";

/// ??? - sets LEDs on, steady.
/// Units in inches. Distance (0-1003)
pub const MODE_US_DC_IN: &str = "US-DC-IN";

/// LEGO EV3 ultrasonic sensor.
#[derive(Debug, Clone, Device, Sensor, Findable)]
#[class_name = "lego-sensor"]
#[driver_name = "lego-ev3-us"]
#[port = "crate::sensors::SensorPort"]
pub struct UltrasonicSensor {
    driver: Driver,
}

impl UltrasonicSensor {
    /// Continuous measurement - sets LEDs on, steady. Units in centimeters. Distance (0-2550)
    pub fn set_mode_us_dist_cm(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_DIST_CM)
    }

    /// Continuous measurement - sets LEDs on, steady. Units in inches. Distance (0-1003)
    pub fn set_mode_us_dist_in(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_DIST_IN)
    }

    /// Listen - sets LEDs on, blinking. Presence (0-1)
    pub fn set_mode_us_listen(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_LISTEN)
    }

    /// Single measurement - LEDs on momentarily when mode is set, then off. Units in centimeters. Distance (0-2550)
    pub fn set_mode_us_si_cm(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_SI_CM)
    }

    /// Single measurement - LEDs on momentarily when mode is set, then off. Units in inches. Distance (0-1003)
    pub fn set_mode_us_si_in(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_SI_IN)
    }

    /// ??? - sets LEDs on, steady . Units in centimeters. Distance (0-2550)
    pub fn set_mode_us_dc_cm(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_DC_CM)
    }

    /// ??? - sets LEDs on, steady . Units in inches. Distance (0-1003)
    pub fn set_mode_us_dc_in(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_DC_IN)
    }

    /// Measurement of the distance detected by the sensor
    pub fn get_distance(&self) -> Ev3Result<i32> {
        self.get_value0()
    }
}
