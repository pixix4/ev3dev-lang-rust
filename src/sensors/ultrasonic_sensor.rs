//! LEGO EV3 ultrasonic sensor

use super::SensorPort;
use crate::{Attribute, Device, Driver, Ev3Error, Ev3Result};
use std::cell::Cell;

/// LEGO EV3 ultrasonic sensor.
#[derive(Debug, Clone, Device)]
pub struct UltrasonicSensor {
    driver: Driver,
    cm_scale: Cell<Option<f32>>,
    in_scale: Cell<Option<f32>>,
}

impl UltrasonicSensor {
    fn new(driver: Driver) -> Self {
        Self {
            driver,
            cm_scale: Cell::new(None),
            in_scale: Cell::new(None),
        }
    }

    findable!(
        "lego-sensor",
        "lego-ev3-us",
        SensorPort,
        "UltrasonicSensor",
        "in"
    );

    /// Continuous measurement - sets LEDs on, steady.
    /// Units in centimeters. Distance (0-2550)
    pub const MODE_US_DIST_CM: &'static str = "US-DIST-CM";

    /// Continuous measurement - sets LEDs on, steady.
    /// Units in inches. Distance (0-1003)
    pub const MODE_US_DIST_IN: &'static str = "US-DIST-IN";

    /// Listen - sets LEDs on, blinking. Presence (0-1)
    pub const MODE_US_LISTEN: &'static str = "US-LISTEN";

    /// Single measurement - LEDs on momentarily when mode is set, then off.
    /// Units in centimeters. Distance (0-2550)
    pub const MODE_US_SI_CM: &'static str = "US-SI-CM";

    /// Single measurement - LEDs on momentarily when mode is set, then off.
    /// Units in inches. Distance (0-1003)
    pub const MODE_US_SI_IN: &'static str = "US-SI-IN";

    /// ??? - sets LEDs on, steady.
    /// Units in centimeters. Distance (0-2550)
    pub const MODE_US_DC_CM: &'static str = "US-DC-CM";

    /// ??? - sets LEDs on, steady.
    /// Units in inches. Distance (0-1003)
    pub const MODE_US_DC_IN: &'static str = "US-DC-IN";

    sensor!();

    /// Continuous measurement - sets LEDs on, steady. Units in centimeters. Distance (0-2550)
    pub fn set_mode_us_dist_cm(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_US_DIST_CM)
    }

    /// Continuous measurement - sets LEDs on, steady. Units in inches. Distance (0-1003)
    pub fn set_mode_us_dist_in(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_US_DIST_IN)
    }

    /// Listen - sets LEDs on, blinking. Presence (0-1)
    pub fn set_mode_us_listen(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_US_LISTEN)
    }

    /// Single measurement - LEDs on momentarily when mode is set, then off. Units in centimeters. Distance (0-2550)
    pub fn set_mode_us_si_cm(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_US_SI_CM)
    }

    /// Single measurement - LEDs on momentarily when mode is set, then off. Units in inches. Distance (0-1003)
    pub fn set_mode_us_si_in(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_US_SI_IN)
    }

    /// ??? - sets LEDs on, steady . Units in centimeters. Distance (0-2550)
    pub fn set_mode_us_dc_cm(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_US_DC_CM)
    }

    /// ??? - sets LEDs on, steady . Units in inches. Distance (0-1003)
    pub fn set_mode_us_dc_in(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_US_DC_IN)
    }

    /// Measurement of the distance detected by the sensor, unscaled.
    pub fn get_distance(&self) -> Ev3Result<i32> {
        self.get_value0()
    }

    /// Measurement of the distance detected by the sensor, in centimeters.
    pub fn get_distance_centimeters(&self) -> Ev3Result<f32> {
        let scale_field = self.cm_scale.get();
        let scale = match scale_field {
            Some(s) => s,
            None => {
                let decimals = self.get_decimals()?;
                let s = 10f32.powi(-decimals);
                self.cm_scale.set(Some(s));
                s
            }
        };

        Ok((self.get_value0()? as f32) * scale)
    }

    /// Measurement of the distance detected by the sensor, in centimeters.
    pub fn get_distance_inches(&self) -> Ev3Result<f32> {
        let scale_field = self.in_scale.get();
        let scale = match scale_field {
            Some(s) => s,
            None => {
                let decimals = self.get_decimals()?;
                let s = 10f32.powi(-decimals);
                self.in_scale.set(Some(s));
                s
            }
        };

        Ok((self.get_value0()? as f32) * scale)
    }
}
