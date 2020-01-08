//! LEGO EV3 infrared sensor.

use super::Sensor;
use crate::{Attribute, Device, Driver, Ev3Result, Findable};

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

/// LEGO EV3 infrared sensor.
#[derive(Debug, Clone, Device, Findable, Sensor)]
#[class_name = "lego-sensor"]
#[driver_name = "lego-ev3-ir"]
#[port = "crate::sensors::SensorPort"]
pub struct InfraredSensor {
    driver: Driver,
}

impl InfraredSensor {
    /// Proximity
    pub fn set_mode_ir_prox(&self) -> Ev3Result<()> {
        self.set_mode(MODE_IR_PROX)
    }

    /// IR Seeker
    pub fn set_mode_ir_seek(&self) -> Ev3Result<()> {
        self.set_mode(MODE_IR_SEEK)
    }

    /// IR Remote Control
    pub fn set_mode_ir_remote(&self) -> Ev3Result<()> {
        self.set_mode(MODE_IR_REMOTE)
    }

    /// IR Remote Control
    pub fn set_mode_ir_rem_a(&self) -> Ev3Result<()> {
        self.set_mode(MODE_IR_REM_A)
    }

    /// Alternate IR Seeker ???
    pub fn set_mode_ir_s_alt(&self) -> Ev3Result<()> {
        self.set_mode(MODE_IR_S_ALT)
    }

    /// Calibration ???
    pub fn set_mode_ir_cal(&self) -> Ev3Result<()> {
        self.set_mode(MODE_IR_CAL)
    }
}
