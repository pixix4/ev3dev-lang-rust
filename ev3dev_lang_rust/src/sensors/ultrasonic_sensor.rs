use crate::sensors::{Sensor, SensorPort};
use crate::{Attribute, Device, Driver, Ev3Result};

/// Continuous measurement - sets LEDs on, steady. Units in centimeters. Distance (0-2550)
pub const MODE_US_DIST_CM: &str = "US-DIST-CM";

/// Continuous measurement - sets LEDs on, steady. Units in inches. Distance (0-1003)
pub const MODE_US_DIST_IN: &str = "US-DIST-IN";

/// Listen - sets LEDs on, blinking. Presence (0-1)
pub const MODE_US_LISTEN: &str = "US-LISTEN";

/// Single measurement - LEDs on momentarily when mode is set, then off. Units in centimeters. Distance (0-2550)
pub const MODE_US_SI_CM: &str = "US-SI-CM";

/// Single measurement - LEDs on momentarily when mode is set, then off. Units in inches. Distance (0-1003)
pub const MODE_US_SI_IN: &str = "US-SI-IN";

/// ??? - sets LEDs on, steady . Units in centimeters. Distance (0-2550)
pub const MODE_US_DC_CM: &str = "US-DC-CM";

/// ??? - sets LEDs on, steady . Units in inches. Distance (0-1003)
pub const MODE_US_DC_IN: &str = "US-DC-IN";

#[derive(Debug, Clone, Device)]
pub struct UltrasonicSensor {
    driver: Driver,
}

impl Sensor for UltrasonicSensor {}

impl UltrasonicSensor {
    /// Try to get a `UltrasonicSensor` on the given port. Returns `None` if port is not used or another device is connected.
    pub fn new(port: SensorPort) -> Ev3Result<UltrasonicSensor> {
        let name = Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-us")?;

        Ok(UltrasonicSensor {
            driver: Driver::new("lego-sensor", &name),
        })
    }

    /// Try to find a `UltrasonicSensor`. Only returns a sensor if their is exactly one connected, `None` otherwise.
    pub fn find() -> Ev3Result<UltrasonicSensor> {
        let name = Driver::find_name_by_driver("lego-sensor", "lego-ev3-us")?;

        Ok(UltrasonicSensor {
            driver: Driver::new("lego-sensor", &name),
        })
    }

    pub fn set_mode_us_dist_cm(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_DIST_CM)
    }

    pub fn set_mode_us_dist_in(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_DIST_IN)
    }

    pub fn set_mode_us_listen(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_LISTEN)
    }

    pub fn set_mode_us_si_cm(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_SI_CM)
    }

    pub fn set_mode_us_si_in(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_SI_IN)
    }

    pub fn set_mode_us_dc_cm(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_DC_CM)
    }

    pub fn set_mode_us_dc_in(&self) -> Ev3Result<()> {
        self.set_mode(MODE_US_DC_IN)
    }

    pub fn get_distance(&self) -> Ev3Result<i32> {
        self.get_value0()
    }
}
