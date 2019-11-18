use crate::sensors::{Sensor, SensorPort};
use core::Device;
use driver::Attribute;
use driver::AttributeResult;
use driver::Driver;

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

#[derive(Debug, Clone)]
pub struct UltrasonicSensor {
    driver: Driver,
}

impl Sensor for UltrasonicSensor {}

impl Device for UltrasonicSensor {
    fn get_attribute(&self, name: &str) -> Attribute {
        self.driver.get_attribute(name)
    }
}

impl UltrasonicSensor {
    /// Try to get a `UltrasonicSensor` on the given port. Returns `None` if port is not used or another device is connected.
    pub fn new(port: SensorPort) -> Option<UltrasonicSensor> {
        if let Some(name) =
            Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-us")
        {
            return Some(UltrasonicSensor {
                driver: Driver::new("lego-sensor", &name),
            });
        }

        None
    }

    /// Try to find a `UltrasonicSensor`. Only returns a sensor if their is exactly one connected, `None` otherwise.
    pub fn find() -> Option<UltrasonicSensor> {
        if let Some(name) = Driver::find_name_by_driver("lego-sensor", "lego-ev3-us") {
            return Some(UltrasonicSensor {
                driver: Driver::new("lego-sensor", &name),
            });
        }

        None
    }

    pub fn set_mode_us_dist_cm(&self) -> AttributeResult<()> {
        self.set_mode(MODE_US_DIST_CM)
    }

    pub fn set_mode_us_dist_in(&self) -> AttributeResult<()> {
        self.set_mode(MODE_US_DIST_IN)
    }

    pub fn set_mode_us_listen(&self) -> AttributeResult<()> {
        self.set_mode(MODE_US_LISTEN)
    }

    pub fn set_mode_us_si_cm(&self) -> AttributeResult<()> {
        self.set_mode(MODE_US_SI_CM)
    }

    pub fn set_mode_us_si_in(&self) -> AttributeResult<()> {
        self.set_mode(MODE_US_SI_IN)
    }

    pub fn set_mode_us_dc_cm(&self) -> AttributeResult<()> {
        self.set_mode(MODE_US_DC_CM)
    }

    pub fn set_mode_us_dc_in(&self) -> AttributeResult<()> {
        self.set_mode(MODE_US_DC_IN)
    }

    pub fn get_distance(&self) -> AttributeResult<i32> {
        self.get_value0()
    }
}
