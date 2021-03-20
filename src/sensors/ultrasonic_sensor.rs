//! LEGO EV3 ultrasonic sensor

use super::{Sensor, SensorPort};
use crate::{sensor_mode, Attribute, Device, Driver, Ev3Error, Ev3Result};
use std::cell::Cell;

/// LEGO EV3 ultrasonic sensor.
#[derive(Debug, Clone, Device, Sensor)]
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
        ["lego-ev3-us", "lego-nxt-us"],
        SensorPort,
        "UltrasonicSensor",
        "in"
    );

    sensor_mode!(
        "US-DIST-CM",
        MODE_US_DIST_CM,
        "Continuous measurement - sets LEDs on, steady. Units in centimeters. Distance (0-2550)",
        set_mode_us_dist_cm,
        is_mode_us_dist_cm
    );
    sensor_mode!(
        "US-DIST-IN",
        MODE_US_DIST_IN,
        "Continuous measurement - sets LEDs on, steady. Units in inches. Distance (0-1003)",
        set_mode_us_dist_in,
        is_mode_us_dist_in
    );
    sensor_mode!(
        "US-LISTEN",
        MODE_US_LISTEN,
        "Listen - sets LEDs on, blinking. Presence (0-1) #",
        set_mode_us_listen,
        is_mode_us_listen
    );
    sensor_mode!(
        "US-SI-CM",
        MODE_US_SI_CM,
        "Single measurement - LEDs on momentarily when mode is set, then off. Units in centimeters. Distance (0-2550)",
        set_mode_us_si_cm,
        is_mode_us_si_cm
    );
    sensor_mode!(
        "US-SI-IN",
        MODE_US_SI_IN,
        "Single measurement - LEDs on momentarily when mode is set, then off. Units in inches. Distance (0-1003)",
        set_mode_us_si_in,
        is_mode_us_si_in
    );
    sensor_mode!(
        "US-DC-CM",
        MODE_US_DC_CM,
        "??? - sets LEDs on, steady. Units in centimeters. Distance (0-2550)",
        set_mode_us_dc_cm,
        is_mode_us_dc_cm
    );
    sensor_mode!(
        "US-DC-IN",
        MODE_US_DC_IN,
        "??? - sets LEDs on, steady. Units in inches. Distance (0-1003)",
        set_mode_us_dc_in,
        is_mode_us_dc_in
    );

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
