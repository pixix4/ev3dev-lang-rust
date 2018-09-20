use core::Sensor;
use core::Device;
use core::SensorPort;
use driver::Driver;
use driver::Attribute;
use driver::AttributeResult;

/// Continuous measurement - sets LEDs on, steady. Units in centimeters. Distance (0-2550)
pub const MODE_US_DIST_CM: &'static str = "US-DIST-CM";

/// Continuous measurement - sets LEDs on, steady. Units in inches. Distance (0-1003)
pub const MODE_US_DIST_IN: &'static str = "US-DIST-IN";

/// Listen - sets LEDs on, blinking. Presence (0-1)
pub const MODE_US_LISTEN: &'static str = "US-LISTEN";

/// Single measurement - LEDs on momentarily when mode is set, then off. Units in centimeters. Distance (0-2550)
pub const MODE_US_SI_CM: &'static str = "US-SI-CM";

/// Single measurement - LEDs on momentarily when mode is set, then off. Units in inches. Distance (0-1003)
pub const MODE_US_SI_IN: &'static str = "US-SI-IN";

/// ??? - sets LEDs on, steady . Units in centimeters. Distance (0-2550)
pub const MODE_US_DC_CM: &'static str = "US-DC-CM";

/// ??? - sets LEDs on, steady . Units in inches. Distance (0-1003)
pub const MODE_US_DC_IN: &'static str = "US-DC-IN";

pub struct UltrasonicSensor {
    driver: Driver
}

impl Sensor for UltrasonicSensor {}

impl Device for UltrasonicSensor {
    fn get_attribute(&mut self, name: &str) -> &Attribute {
        self.driver.get_attribute(name)
    }
}

impl UltrasonicSensor {

    /// Try to get a `UltrasonicSensor` on the given port. Returns `None` if port is not used or another device is connected.
    pub fn new(port: SensorPort) -> Option<UltrasonicSensor> {
        if let Some(name) = Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-us") {
            return Some(UltrasonicSensor {
                driver: Driver::new(String::from("lego-sensor"), name)
            });
        }

        None
    }

    /// Try to find a `UltrasonicSensor`. Only returns a sensor if their is exactly one connected, `None` otherwise.
    pub fn find() -> Option<UltrasonicSensor> {
        if let Some(name) = Driver::find_name_by_driver("lego-sensor", "lego-ev3-us") {
            return Some(UltrasonicSensor {
                driver: Driver::new(String::from("lego-sensor"), name)
            });
        }

        None
    }

    pub fn set_mode_us_dist_cm(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_US_DIST_CM))
    }

    pub fn set_mode_us_dist_in(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_US_DIST_IN))
    }

    pub fn set_mode_us_listen(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_US_LISTEN))
    }

    pub fn set_mode_us_si_cm(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_US_SI_CM))
    }

    pub fn set_mode_us_si_in(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_US_SI_IN))
    }

    pub fn set_mode_us_dc_cm(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_US_DC_CM))
    }

    pub fn set_mode_us_dc_in(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_US_DC_IN))
    }

    pub fn get_distance(&mut self) -> AttributeResult<isize> {
        self.get_value0()
    }
}