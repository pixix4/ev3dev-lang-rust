use super::MotorPort;
use crate::{wait, Attribute, Device, Driver, Ev3Result};
use std::time::Duration;

/// EV3/NXT large servo motor
#[derive(Debug, Clone, Device)]
pub struct LargeMotor {
    driver: Driver,
}

impl LargeMotor {
    findable!("tacho-motor", "lego-ev3-l-motor", MotorPort);
    tacho_motor!();
}
