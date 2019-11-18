use crate::sensors::{Sensor, SensorPort};
use crate::{Attribute, Device, Driver, Ev3Result};

/// Proximity
pub const MODE_IR_PROX: &str = "IR-PROX";

/// IR Seeker
pub const MODE_IR_SEEK: &str = "IR-SEEK";

/// IR Remote Control
pub const MODE_IR_REMOTE: &str = "IR-REMOTE";

/// IR Remote Control
pub const MODE_IR_REM_A: &str = "IR-REM-A";

/// Alternate IR Seeker ???
pub const MODE_IR_S_ALT: &str = "IR-S-ALT";

/// Calibration ???
pub const MODE_IR_CAL: &str = "IR-CAL";

#[derive(Debug, Clone, Device)]
pub struct InfraredSensor {
    driver: Driver,
}

impl Sensor for InfraredSensor {}

impl InfraredSensor {
    /// Try to get a `InfraredSensor` on the given port. Returns `None` if port is not used or another device is connected.
    pub fn new(port: SensorPort) -> Ev3Result<InfraredSensor> {
        let name = Driver::find_name_by_port_and_driver("lego-sensor", &port, "lego-ev3-ir")?;

        Ok(InfraredSensor {
            driver: Driver::new("lego-sensor", &name),
        })
    }

    /// Try to find a `InfraredSensor`. Only returns a sensor if their is exactly one connected, `None` otherwise.
    pub fn find() -> Ev3Result<InfraredSensor> {
        let name = Driver::find_name_by_driver("lego-sensor", "lego-ev3-ir")?;

        Ok(InfraredSensor {
            driver: Driver::new("lego-sensor", &name),
        })
    }

    pub fn set_mode_ir_prox(&self) -> Ev3Result<()> {
        self.set_mode(MODE_IR_PROX)
    }

    pub fn set_mode_ir_seek(&self) -> Ev3Result<()> {
        self.set_mode(MODE_IR_SEEK)
    }

    pub fn set_mode_ir_remote(&self) -> Ev3Result<()> {
        self.set_mode(MODE_IR_REMOTE)
    }

    pub fn set_mode_ir_rem_a(&self) -> Ev3Result<()> {
        self.set_mode(MODE_IR_REM_A)
    }

    pub fn set_mode_ir_s_alt(&self) -> Ev3Result<()> {
        self.set_mode(MODE_IR_S_ALT)
    }

    pub fn set_mode_ir_cal(&self) -> Ev3Result<()> {
        self.set_mode(MODE_IR_CAL)
    }
}
