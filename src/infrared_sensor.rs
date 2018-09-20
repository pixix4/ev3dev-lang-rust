use core::Sensor;
use core::Device;
use core::SensorPort;
use driver::Driver;
use driver::Attribute;
use driver::AttributeResult;

/// Proximity
pub const MODE_IR_PROX: &'static str = "IR-PROX";

/// IR Seeker
pub const MODE_IR_SEEK: &'static str = "IR-SEEK";

/// IR Remote Control
pub const MODE_IR_REMOTE: &'static str = "IR-REMOTE";

/// IR Remote Control
pub const MODE_IR_REM_A: &'static str = "IR-REM-A";

/// Alternate IR Seeker ???
pub const MODE_IR_S_ALT: &'static str = "IR-S-ALT";

/// Calibration ???
pub const MODE_IR_CAL: &'static str = "IR-CAL";

pub struct InfraredSensor {
    driver: Driver
}

impl Sensor for InfraredSensor {}

impl Device for InfraredSensor {
    fn get_attribute(&mut self, name: &str) -> &Attribute {
        self.driver.get_attribute(name)
    }
}

impl InfraredSensor {

    /// Try to get a `InfraredSensor` on the given port. Returns `None` if port is not used or another device is connected.
    pub fn new(port: SensorPort) -> Option<InfraredSensor> {
        if let Some(name) = Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-ir") {
            return Some(InfraredSensor {
                driver: Driver::new(String::from("lego-sensor"), name)
            });
        }

        None
    }

    /// Try to find a `InfraredSensor`. Only returns a sensor if their is exactly one connected, `None` otherwise.
    pub fn find() -> Option<InfraredSensor> {
        if let Some(name) = Driver::find_name_by_driver("lego-sensor", "lego-ev3-ir") {
            return Some(InfraredSensor {
                driver: Driver::new(String::from("lego-sensor"), name)
            });
        }

        None
    }

    pub fn set_mode_ir_prox(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_IR_PROX))
    }

    pub fn set_mode_ir_seek(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_IR_SEEK))
    }

    pub fn set_mode_ir_remote(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_IR_REMOTE))
    }

    pub fn set_mode_ir_rem_a(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_IR_REM_A))
    }

    pub fn set_mode_ir_s_alt(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_IR_S_ALT))
    }

    pub fn set_mode_ir_cal(&mut self) -> AttributeResult<()> {
        self.set_mode(String::from(MODE_IR_CAL))
    }
}