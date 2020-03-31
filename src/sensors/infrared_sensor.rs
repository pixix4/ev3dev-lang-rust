//! LEGO EV3 infrared sensor.

use super::SensorPort;
use crate::{Attribute, Device, Driver, Ev3Result};

/// LEGO EV3 infrared sensor.
#[derive(Debug, Clone, Device)]
pub struct InfraredSensor {
    driver: Driver,
}

impl InfraredSensor {
    findable!("lego-sensor", "lego-ev3-ir", SensorPort);

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

    sensor!();

    /// Proximity
    pub fn set_mode_ir_prox(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_IR_PROX)
    }

    /// IR Seeker
    pub fn set_mode_ir_seek(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_IR_SEEK)
    }

    /// IR Remote Control
    pub fn set_mode_ir_remote(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_IR_REMOTE)
    }

    /// IR Remote Control
    pub fn set_mode_ir_rem_a(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_IR_REM_A)
    }

    /// Alternate IR Seeker ???
    pub fn set_mode_ir_s_alt(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_IR_S_ALT)
    }

    /// Calibration ???
    pub fn set_mode_ir_cal(&self) -> Ev3Result<()> {
        self.set_mode(Self::MODE_IR_CAL)
    }
}
