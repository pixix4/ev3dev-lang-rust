use super::MotorPort;
use crate::{wait, Attribute, Device, Driver, Ev3Result};
use std::time::Duration;

/// EV3 medium servo motor
#[derive(Debug, Clone, Device)]
pub struct MediumMotor {
    driver: Driver,
}

impl MediumMotor {
    findable!("tacho-motor", "lego-ev3-m-motor", MotorPort);
    tacho_motor!();
}
